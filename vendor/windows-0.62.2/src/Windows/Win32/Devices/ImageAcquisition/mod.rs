pub const ADVANCED_DUP: u32 = 8192u32;
pub const ADVANCED_DUPLEX: u32 = 1024u32;
pub const ALL_PAGES: u32 = 0u32;
pub const AUTO_ADVANCE: u32 = 512u32;
pub const AUTO_SOURCE: u32 = 32768u32;
pub const BACK_FIRST: u32 = 16u32;
pub const BACK_ONLY: u32 = 64u32;
pub const BARCODE_READER: u32 = 262144u32;
pub const BARCODE_READER_READY: u32 = 16384u32;
pub const BASE_VAL_WIA_ERROR: u32 = 0u32;
pub const BASE_VAL_WIA_SUCCESS: u32 = 0u32;
pub const BOTTOM_JUSTIFIED: u32 = 2u32;
pub const BUS_TYPE_FIREWIRE: u32 = 203u32;
pub const BUS_TYPE_PARALLEL: u32 = 202u32;
pub const BUS_TYPE_SCSI: u32 = 200u32;
pub const BUS_TYPE_USB: u32 = 201u32;
pub const CAPTUREMODE_BURST: u32 = 2u32;
pub const CAPTUREMODE_NORMAL: u32 = 1u32;
pub const CAPTUREMODE_TIMELAPSE: u32 = 3u32;
pub const CENTERED: u32 = 1u32;
pub const CFSTR_WIAITEMNAMES: windows_core::PCWSTR = windows_core::w!("WIAItemNames");
pub const CFSTR_WIAITEMPTR: windows_core::PCWSTR = windows_core::w!("WIAItemPointer");
pub const CLSID_WiaDefaultSegFilter: windows_core::GUID = windows_core::GUID::from_u128(0xd4f4d30b_0b29_4508_8922_0c5797d42765);
pub const CMD_GETADFAVAILABLE: u32 = 117u32;
pub const CMD_GETADFHASPAPER: u32 = 120u32;
pub const CMD_GETADFOPEN: u32 = 118u32;
pub const CMD_GETADFREADY: u32 = 119u32;
pub const CMD_GETADFSTATUS: u32 = 121u32;
pub const CMD_GETADFUNLOADREADY: u32 = 122u32;
pub const CMD_GETCAPABILITIES: u32 = 132u32;
pub const CMD_GETSUPPORTEDFILEFORMATS: u32 = 138u32;
pub const CMD_GETSUPPORTEDMEMORYFORMATS: u32 = 139u32;
pub const CMD_GETTPAAVAILABLE: u32 = 123u32;
pub const CMD_GETTPAOPENED: u32 = 124u32;
pub const CMD_GET_INTERRUPT_EVENT: u32 = 133u32;
pub const CMD_INITIALIZE: u32 = 100u32;
pub const CMD_LOAD_ADF: u32 = 115u32;
pub const CMD_RESETSCANNER: u32 = 131u32;
pub const CMD_SENDSCSICOMMAND: u32 = 127u32;
pub const CMD_SETCOLORDITHER: u32 = 111u32;
pub const CMD_SETCONTRAST: u32 = 104u32;
pub const CMD_SETDATATYPE: u32 = 106u32;
pub const CMD_SETDITHER: u32 = 107u32;
pub const CMD_SETFILTER: u32 = 114u32;
pub const CMD_SETFORMAT: u32 = 140u32;
pub const CMD_SETGSDNAME: u32 = 134u32;
pub const CMD_SETINTENSITY: u32 = 105u32;
pub const CMD_SETLAMP: u32 = 126u32;
pub const CMD_SETMATRIX: u32 = 112u32;
pub const CMD_SETMIRROR: u32 = 108u32;
pub const CMD_SETNEGATIVE: u32 = 109u32;
pub const CMD_SETSCANMODE: u32 = 135u32;
pub const CMD_SETSPEED: u32 = 113u32;
pub const CMD_SETSTIDEVICEHKEY: u32 = 136u32;
pub const CMD_SETTONEMAP: u32 = 110u32;
pub const CMD_SETXRESOLUTION: u32 = 102u32;
pub const CMD_SETYRESOLUTION: u32 = 103u32;
pub const CMD_STI_DEVICERESET: u32 = 128u32;
pub const CMD_STI_DIAGNOSTIC: u32 = 130u32;
pub const CMD_STI_GETSTATUS: u32 = 129u32;
pub const CMD_TPAREADY: u32 = 125u32;
pub const CMD_UNINITIALIZE: u32 = 101u32;
pub const CMD_UNLOAD_ADF: u32 = 116u32;
pub const COPY_PARENT_PROPERTY_VALUES: u32 = 1073741824u32;
pub const DETECT_DUP: u32 = 64u32;
pub const DETECT_DUP_AVAIL: u32 = 256u32;
pub const DETECT_FEED: u32 = 32u32;
pub const DETECT_FEED_AVAIL: u32 = 128u32;
pub const DETECT_FILM_TPA: u32 = 1024u32;
pub const DETECT_FLAT: u32 = 8u32;
pub const DETECT_SCAN: u32 = 16u32;
pub const DETECT_STOR: u32 = 4096u32;
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct DEVICEDIALOGDATA {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pIWiaItemRoot: core::mem::ManuallyDrop<Option<IWiaItem>>,
    pub dwFlags: u32,
    pub lIntent: i32,
    pub lItemCount: i32,
    pub ppWiaItems: *mut Option<IWiaItem>,
}
impl Default for DEVICEDIALOGDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct DEVICEDIALOGDATA2 {
    pub cbSize: u32,
    pub pIWiaItemRoot: core::mem::ManuallyDrop<Option<IWiaItem2>>,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub bstrFolderName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrFilename: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub lNumFiles: i32,
    pub pbstrFilePaths: *mut windows_core::BSTR,
    pub pWiaItem: core::mem::ManuallyDrop<Option<IWiaItem2>>,
}
impl Default for DEVICEDIALOGDATA2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEVICE_ATTENTION: u32 = 1024u32;
pub const DUP: u32 = 4u32;
pub const DUPLEX: u32 = 4u32;
pub const DUP_READY: u32 = 4u32;
pub type DeviceDialogFunction = Option<unsafe extern "system" fn(param0: *mut DEVICEDIALOGDATA) -> windows_core::HRESULT>;
pub const EFFECTMODE_BW: u32 = 2u32;
pub const EFFECTMODE_SEPIA: u32 = 3u32;
pub const EFFECTMODE_STANDARD: u32 = 1u32;
pub const ENDORSER: u32 = 131072u32;
pub const ENDORSER_READY: u32 = 8192u32;
pub const ESC_TWAIN_CAPABILITY: u32 = 2001u32;
pub const ESC_TWAIN_PRIVATE_SUPPORTED_CAPS: u32 = 2002u32;
pub const EXPOSUREMETERING_AVERAGE: u32 = 1u32;
pub const EXPOSUREMETERING_CENTERSPOT: u32 = 4u32;
pub const EXPOSUREMETERING_CENTERWEIGHT: u32 = 2u32;
pub const EXPOSUREMETERING_MULTISPOT: u32 = 3u32;
pub const EXPOSUREMODE_APERTURE_PRIORITY: u32 = 3u32;
pub const EXPOSUREMODE_AUTO: u32 = 2u32;
pub const EXPOSUREMODE_MANUAL: u32 = 1u32;
pub const EXPOSUREMODE_PORTRAIT: u32 = 7u32;
pub const EXPOSUREMODE_PROGRAM_ACTION: u32 = 6u32;
pub const EXPOSUREMODE_PROGRAM_CREATIVE: u32 = 5u32;
pub const EXPOSUREMODE_SHUTTER_PRIORITY: u32 = 4u32;
pub const FEED: u32 = 1u32;
pub const FEEDER: u32 = 1u32;
pub const FEED_READY: u32 = 1u32;
pub const FILM_TPA: u32 = 512u32;
pub const FILM_TPA_READY: u32 = 64u32;
pub const FLASHMODE_AUTO: u32 = 1u32;
pub const FLASHMODE_EXTERNALSYNC: u32 = 6u32;
pub const FLASHMODE_FILL: u32 = 3u32;
pub const FLASHMODE_OFF: u32 = 2u32;
pub const FLASHMODE_REDEYE_AUTO: u32 = 4u32;
pub const FLASHMODE_REDEYE_FILL: u32 = 5u32;
pub const FLAT: u32 = 2u32;
pub const FLATBED: u32 = 2u32;
pub const FLAT_COVER_UP: u32 = 8u32;
pub const FLAT_READY: u32 = 2u32;
pub const FOCUSMETERING_CENTERSPOT: u32 = 1u32;
pub const FOCUSMETERING_MULTISPOT: u32 = 2u32;
pub const FOCUSMODE_AUTO: u32 = 2u32;
pub const FOCUSMODE_MACROAUTO: u32 = 3u32;
pub const FOCUSMODE_MANUAL: u32 = 1u32;
pub const FRONT_FIRST: u32 = 8u32;
pub const FRONT_ONLY: u32 = 32u32;
pub const GUID_DEVINTERFACE_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0x6bdd1fc6_810f_11d0_bec7_08002be2092f);
windows_core::imp::define_interface!(IEnumWIA_DEV_CAPS, IEnumWIA_DEV_CAPS_Vtbl, 0x1fcc4287_aca6_11d2_a093_00c04f72dc3c);
windows_core::imp::interface_hierarchy!(IEnumWIA_DEV_CAPS, windows_core::IUnknown);
impl IEnumWIA_DEV_CAPS {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumWIA_DEV_CAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_DEV_CAPS_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut WIA_DEV_CAP, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumWIA_DEV_CAPS_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IEnumWIA_DEV_CAPS_Vtbl {
    pub const fn new<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut WIA_DEV_CAP, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_CAPS_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_CAPS_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_CAPS_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWIA_DEV_CAPS_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumWIA_DEV_CAPS_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWIA_DEV_CAPS_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcelt.write(core::mem::transmute(ok__));
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
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWIA_DEV_CAPS as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumWIA_DEV_CAPS {}
windows_core::imp::define_interface!(IEnumWIA_DEV_INFO, IEnumWIA_DEV_INFO_Vtbl, 0x5e38b83c_8cf1_11d1_bf92_0060081ed811);
windows_core::imp::interface_hierarchy!(IEnumWIA_DEV_INFO, windows_core::IUnknown);
impl IEnumWIA_DEV_INFO {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut Option<IWiaPropertyStorage>, pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumWIA_DEV_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_DEV_INFO_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumWIA_DEV_INFO_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: windows_core::OutRef<IWiaPropertyStorage>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWIA_DEV_INFO>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IEnumWIA_DEV_INFO_Vtbl {
    pub const fn new<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_INFO_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_INFO_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_DEV_INFO_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWIA_DEV_INFO_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumWIA_DEV_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWIA_DEV_INFO_Impl::GetCount(this) {
                    Ok(ok__) => {
                        celt.write(core::mem::transmute(ok__));
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
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWIA_DEV_INFO as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumWIA_DEV_INFO {}
windows_core::imp::define_interface!(IEnumWIA_FORMAT_INFO, IEnumWIA_FORMAT_INFO_Vtbl, 0x81befc5b_656d_44f1_b24c_d41d51b4dc81);
windows_core::imp::interface_hierarchy!(IEnumWIA_FORMAT_INFO, windows_core::IUnknown);
impl IEnumWIA_FORMAT_INFO {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumWIA_FORMAT_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWIA_FORMAT_INFO_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut WIA_FORMAT_INFO, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumWIA_FORMAT_INFO_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWIA_FORMAT_INFO>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IEnumWIA_FORMAT_INFO_Vtbl {
    pub const fn new<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut WIA_FORMAT_INFO, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_FORMAT_INFO_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_FORMAT_INFO_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWIA_FORMAT_INFO_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWIA_FORMAT_INFO_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumWIA_FORMAT_INFO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWIA_FORMAT_INFO_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcelt.write(core::mem::transmute(ok__));
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
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWIA_FORMAT_INFO as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumWIA_FORMAT_INFO {}
windows_core::imp::define_interface!(IEnumWiaItem, IEnumWiaItem_Vtbl, 0x5e8383fc_3391_11d2_9a33_00c04fa36145);
windows_core::imp::interface_hierarchy!(IEnumWiaItem, windows_core::IUnknown);
impl IEnumWiaItem {
    pub unsafe fn Next(&self, celt: u32, ppiwiaitem: *mut Option<IWiaItem>, pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppiwiaitem), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumWiaItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWiaItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumWiaItem_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppiwiaitem: windows_core::OutRef<IWiaItem>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWiaItem>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IEnumWiaItem_Vtbl {
    pub const fn new<Identity: IEnumWiaItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppiwiaitem: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppiwiaitem), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWiaItem_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWiaItem_Impl::GetCount(this) {
                    Ok(ok__) => {
                        celt.write(core::mem::transmute(ok__));
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
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWiaItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumWiaItem {}
windows_core::imp::define_interface!(IEnumWiaItem2, IEnumWiaItem2_Vtbl, 0x59970af4_cd0d_44d9_ab24_52295630e582);
windows_core::imp::interface_hierarchy!(IEnumWiaItem2, windows_core::IUnknown);
impl IEnumWiaItem2 {
    pub unsafe fn Next(&self, celt: u32, ppiwiaitem2: *mut Option<IWiaItem2>, pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(ppiwiaitem2), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumWiaItem2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWiaItem2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumWiaItem2_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppiwiaitem2: windows_core::OutRef<IWiaItem2>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumWiaItem2>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IEnumWiaItem2_Vtbl {
    pub const fn new<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppiwiaitem2: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem2_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppiwiaitem2), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem2_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumWiaItem2_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWiaItem2_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumWiaItem2_Impl::GetCount(this) {
                    Ok(ok__) => {
                        celt.write(core::mem::transmute(ok__));
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
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumWiaItem2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumWiaItem2 {}
pub const IMPRINTER: u32 = 65536u32;
pub const IMPRINTER_READY: u32 = 4096u32;
pub const IT_MSG_DATA: u32 = 2u32;
pub const IT_MSG_DATA_HEADER: u32 = 1u32;
pub const IT_MSG_FILE_PREVIEW_DATA: u32 = 6u32;
pub const IT_MSG_FILE_PREVIEW_DATA_HEADER: u32 = 7u32;
pub const IT_MSG_NEW_PAGE: u32 = 5u32;
pub const IT_MSG_STATUS: u32 = 3u32;
pub const IT_MSG_TERMINATION: u32 = 4u32;
pub const IT_STATUS_MASK: u32 = 7u32;
pub const IT_STATUS_PROCESSING_DATA: u32 = 2u32;
pub const IT_STATUS_TRANSFER_FROM_DEVICE: u32 = 1u32;
pub const IT_STATUS_TRANSFER_TO_CLIENT: u32 = 4u32;
windows_core::imp::define_interface!(IWiaAppErrorHandler, IWiaAppErrorHandler_Vtbl, 0x6c16186c_d0a6_400c_80f4_d26986a0e734);
windows_core::imp::interface_hierarchy!(IWiaAppErrorHandler, windows_core::IUnknown);
impl IWiaAppErrorHandler {
    pub unsafe fn GetWindow(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReportStatus<P1>(&self, lflags: i32, pwiaitem2: P1, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWiaItem2>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReportStatus)(windows_core::Interface::as_raw(self), lflags, pwiaitem2.param().abi(), hrstatus, lpercentcomplete).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaAppErrorHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub ReportStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, windows_core::HRESULT, i32) -> windows_core::HRESULT,
}
pub trait IWiaAppErrorHandler_Impl: windows_core::IUnknownImpl {
    fn GetWindow(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn ReportStatus(&self, lflags: i32, pwiaitem2: windows_core::Ref<IWiaItem2>, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::Result<()>;
}
impl IWiaAppErrorHandler_Vtbl {
    pub const fn new<Identity: IWiaAppErrorHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWindow<Identity: IWiaAppErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaAppErrorHandler_Impl::GetWindow(this) {
                    Ok(ok__) => {
                        phwnd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReportStatus<Identity: IWiaAppErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiaitem2: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaAppErrorHandler_Impl::ReportStatus(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiaitem2), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&lpercentcomplete)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWindow: GetWindow::<Identity, OFFSET>,
            ReportStatus: ReportStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaAppErrorHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaAppErrorHandler {}
windows_core::imp::define_interface!(IWiaDataCallback, IWiaDataCallback_Vtbl, 0xa558a866_a5b0_11d2_a08f_00c04f72dc3c);
windows_core::imp::interface_hierarchy!(IWiaDataCallback, windows_core::IUnknown);
impl IWiaDataCallback {
    pub unsafe fn BandedDataCallback(&self, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *mut u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BandedDataCallback)(windows_core::Interface::as_raw(self), lmessage, lstatus, lpercentcomplete, loffset, llength, lreserved, lreslength, pbbuffer as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDataCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BandedDataCallback: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, i32, i32, i32, *mut u8) -> windows_core::HRESULT,
}
pub trait IWiaDataCallback_Impl: windows_core::IUnknownImpl {
    fn BandedDataCallback(&self, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *mut u8) -> windows_core::Result<()>;
}
impl IWiaDataCallback_Vtbl {
    pub const fn new<Identity: IWiaDataCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BandedDataCallback<Identity: IWiaDataCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmessage: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, lreserved: i32, lreslength: i32, pbbuffer: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDataCallback_Impl::BandedDataCallback(this, core::mem::transmute_copy(&lmessage), core::mem::transmute_copy(&lstatus), core::mem::transmute_copy(&lpercentcomplete), core::mem::transmute_copy(&loffset), core::mem::transmute_copy(&llength), core::mem::transmute_copy(&lreserved), core::mem::transmute_copy(&lreslength), core::mem::transmute_copy(&pbbuffer)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), BandedDataCallback: BandedDataCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaDataCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaDataCallback {}
windows_core::imp::define_interface!(IWiaDataTransfer, IWiaDataTransfer_Vtbl, 0xa6cef998_a5b0_11d2_a08f_00c04f72dc3c);
windows_core::imp::interface_hierarchy!(IWiaDataTransfer, windows_core::IUnknown);
impl IWiaDataTransfer {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn idtGetData<P1>(&self, pmedium: *mut super::super::System::Com::STGMEDIUM, piwiadatacallback: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWiaDataCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).idtGetData)(windows_core::Interface::as_raw(self), core::mem::transmute(pmedium), piwiadatacallback.param().abi()).ok() }
    }
    pub unsafe fn idtGetBandedData<P1>(&self, pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO, piwiadatacallback: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWiaDataCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).idtGetBandedData)(windows_core::Interface::as_raw(self), pwiadatatransinfo as _, piwiadatacallback.param().abi()).ok() }
    }
    pub unsafe fn idtQueryGetData(&self, pfe: Option<*const WIA_FORMAT_INFO>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).idtQueryGetData)(windows_core::Interface::as_raw(self), pfe.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn idtEnumWIA_FORMAT_INFO(&self) -> windows_core::Result<IEnumWIA_FORMAT_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).idtEnumWIA_FORMAT_INFO)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn idtGetExtendedTransferInfo(&self, pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).idtGetExtendedTransferInfo)(windows_core::Interface::as_raw(self), pextendedtransferinfo as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDataTransfer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub idtGetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Com::STGMEDIUM, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    idtGetData: usize,
    pub idtGetBandedData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WIA_DATA_TRANSFER_INFO, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub idtQueryGetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const WIA_FORMAT_INFO) -> windows_core::HRESULT,
    pub idtEnumWIA_FORMAT_INFO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub idtGetExtendedTransferInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WIA_EXTENDED_TRANSFER_INFO) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IWiaDataTransfer_Impl: windows_core::IUnknownImpl {
    fn idtGetData(&self, pmedium: *mut super::super::System::Com::STGMEDIUM, piwiadatacallback: windows_core::Ref<IWiaDataCallback>) -> windows_core::Result<()>;
    fn idtGetBandedData(&self, pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO, piwiadatacallback: windows_core::Ref<IWiaDataCallback>) -> windows_core::Result<()>;
    fn idtQueryGetData(&self, pfe: *const WIA_FORMAT_INFO) -> windows_core::Result<()>;
    fn idtEnumWIA_FORMAT_INFO(&self) -> windows_core::Result<IEnumWIA_FORMAT_INFO>;
    fn idtGetExtendedTransferInfo(&self, pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IWiaDataTransfer_Vtbl {
    pub const fn new<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn idtGetData<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmedium: *mut super::super::System::Com::STGMEDIUM, piwiadatacallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDataTransfer_Impl::idtGetData(this, core::mem::transmute_copy(&pmedium), core::mem::transmute_copy(&piwiadatacallback)).into()
            }
        }
        unsafe extern "system" fn idtGetBandedData<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwiadatatransinfo: *mut WIA_DATA_TRANSFER_INFO, piwiadatacallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDataTransfer_Impl::idtGetBandedData(this, core::mem::transmute_copy(&pwiadatatransinfo), core::mem::transmute_copy(&piwiadatacallback)).into()
            }
        }
        unsafe extern "system" fn idtQueryGetData<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfe: *const WIA_FORMAT_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDataTransfer_Impl::idtQueryGetData(this, core::mem::transmute_copy(&pfe)).into()
            }
        }
        unsafe extern "system" fn idtEnumWIA_FORMAT_INFO<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDataTransfer_Impl::idtEnumWIA_FORMAT_INFO(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn idtGetExtendedTransferInfo<Identity: IWiaDataTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextendedtransferinfo: *mut WIA_EXTENDED_TRANSFER_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDataTransfer_Impl::idtGetExtendedTransferInfo(this, core::mem::transmute_copy(&pextendedtransferinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            idtGetData: idtGetData::<Identity, OFFSET>,
            idtGetBandedData: idtGetBandedData::<Identity, OFFSET>,
            idtQueryGetData: idtQueryGetData::<Identity, OFFSET>,
            idtEnumWIA_FORMAT_INFO: idtEnumWIA_FORMAT_INFO::<Identity, OFFSET>,
            idtGetExtendedTransferInfo: idtGetExtendedTransferInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaDataTransfer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IWiaDataTransfer {}
windows_core::imp::define_interface!(IWiaDevMgr, IWiaDevMgr_Vtbl, 0x5eb2502a_8cf1_11d1_bf92_0060081ed811);
windows_core::imp::interface_hierarchy!(IWiaDevMgr, windows_core::IUnknown);
impl IWiaDevMgr {
    pub unsafe fn EnumDeviceInfo(&self, lflag: i32) -> windows_core::Result<IEnumWIA_DEV_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDeviceInfo)(windows_core::Interface::as_raw(self), lflag, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDevice(&self, bstrdeviceid: &windows_core::BSTR) -> windows_core::Result<IWiaItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdeviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SelectDeviceDlg(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR, ppitemroot: *mut Option<IWiaItem>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SelectDeviceDlg)(windows_core::Interface::as_raw(self), hwndparent, ldevicetype, lflags, core::mem::transmute(pbstrdeviceid), core::mem::transmute(ppitemroot)).ok() }
    }
    pub unsafe fn SelectDeviceDlgID(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SelectDeviceDlgID)(windows_core::Interface::as_raw(self), hwndparent, ldevicetype, lflags, core::mem::transmute(pbstrdeviceid)).ok() }
    }
    pub unsafe fn GetImageDlg<P4>(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: P4, bstrfilename: &windows_core::BSTR, pguidformat: *mut windows_core::GUID) -> windows_core::Result<()>
    where
        P4: windows_core::Param<IWiaItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetImageDlg)(windows_core::Interface::as_raw(self), hwndparent, ldevicetype, lflags, lintent, pitemroot.param().abi(), core::mem::transmute_copy(bstrfilename), pguidformat as _).ok() }
    }
    pub unsafe fn RegisterEventCallbackProgram(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, bstrcommandline: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterEventCallbackProgram)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, core::mem::transmute_copy(bstrcommandline), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrdescription), core::mem::transmute_copy(bstricon)).ok() }
    }
    pub unsafe fn RegisterEventCallbackInterface<P3>(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, piwiaeventcallback: P3) -> windows_core::Result<windows_core::IUnknown>
    where
        P3: windows_core::Param<IWiaEventCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterEventCallbackInterface)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, piwiaeventcallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterEventCallbackCLSID(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterEventCallbackCLSID)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, pclsid, core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrdescription), core::mem::transmute_copy(bstricon)).ok() }
    }
    pub unsafe fn AddDeviceDlg(&self, hwndparent: super::super::Foundation::HWND, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddDeviceDlg)(windows_core::Interface::as_raw(self), hwndparent, lflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDevMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectDeviceDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, i32, i32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectDeviceDlgID: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetImageDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, i32, i32, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub RegisterEventCallbackProgram: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterEventCallbackInterface: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterEventCallbackCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddDeviceDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, i32) -> windows_core::HRESULT,
}
pub trait IWiaDevMgr_Impl: windows_core::IUnknownImpl {
    fn EnumDeviceInfo(&self, lflag: i32) -> windows_core::Result<IEnumWIA_DEV_INFO>;
    fn CreateDevice(&self, bstrdeviceid: &windows_core::BSTR) -> windows_core::Result<IWiaItem>;
    fn SelectDeviceDlg(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR, ppitemroot: windows_core::OutRef<IWiaItem>) -> windows_core::Result<()>;
    fn SelectDeviceDlgID(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetImageDlg(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: windows_core::Ref<IWiaItem>, bstrfilename: &windows_core::BSTR, pguidformat: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn RegisterEventCallbackProgram(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, bstrcommandline: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RegisterEventCallbackInterface(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, piwiaeventcallback: windows_core::Ref<IWiaEventCallback>) -> windows_core::Result<windows_core::IUnknown>;
    fn RegisterEventCallbackCLSID(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddDeviceDlg(&self, hwndparent: super::super::Foundation::HWND, lflags: i32) -> windows_core::Result<()>;
}
impl IWiaDevMgr_Vtbl {
    pub const fn new<Identity: IWiaDevMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumDeviceInfo<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflag: i32, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr_Impl::EnumDeviceInfo(this, core::mem::transmute_copy(&lflag)) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDevice<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceid: *mut core::ffi::c_void, ppwiaitemroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr_Impl::CreateDevice(this, core::mem::transmute(&bstrdeviceid)) {
                    Ok(ok__) => {
                        ppwiaitemroot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SelectDeviceDlg<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut *mut core::ffi::c_void, ppitemroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr_Impl::SelectDeviceDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pbstrdeviceid), core::mem::transmute_copy(&ppitemroot)).into()
            }
        }
        unsafe extern "system" fn SelectDeviceDlgID<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr_Impl::SelectDeviceDlgID(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pbstrdeviceid)).into()
            }
        }
        unsafe extern "system" fn GetImageDlg<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, lintent: i32, pitemroot: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, pguidformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr_Impl::GetImageDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lintent), core::mem::transmute_copy(&pitemroot), core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&pguidformat)).into()
            }
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, bstrcommandline: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void, bstricon: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr_Impl::RegisterEventCallbackProgram(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute(&bstrcommandline), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
            }
        }
        unsafe extern "system" fn RegisterEventCallbackInterface<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, piwiaeventcallback: *mut core::ffi::c_void, peventobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr_Impl::RegisterEventCallbackInterface(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&piwiaeventcallback)) {
                    Ok(ok__) => {
                        peventobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void, bstricon: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr_Impl::RegisterEventCallbackCLSID(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&pclsid), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
            }
        }
        unsafe extern "system" fn AddDeviceDlg<Identity: IWiaDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr_Impl::AddDeviceDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&lflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumDeviceInfo: EnumDeviceInfo::<Identity, OFFSET>,
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            SelectDeviceDlg: SelectDeviceDlg::<Identity, OFFSET>,
            SelectDeviceDlgID: SelectDeviceDlgID::<Identity, OFFSET>,
            GetImageDlg: GetImageDlg::<Identity, OFFSET>,
            RegisterEventCallbackProgram: RegisterEventCallbackProgram::<Identity, OFFSET>,
            RegisterEventCallbackInterface: RegisterEventCallbackInterface::<Identity, OFFSET>,
            RegisterEventCallbackCLSID: RegisterEventCallbackCLSID::<Identity, OFFSET>,
            AddDeviceDlg: AddDeviceDlg::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaDevMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaDevMgr {}
windows_core::imp::define_interface!(IWiaDevMgr2, IWiaDevMgr2_Vtbl, 0x79c07cf1_cbdd_41ee_8ec3_f00080cada7a);
windows_core::imp::interface_hierarchy!(IWiaDevMgr2, windows_core::IUnknown);
impl IWiaDevMgr2 {
    pub unsafe fn EnumDeviceInfo(&self, lflags: i32) -> windows_core::Result<IEnumWIA_DEV_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDeviceInfo)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDevice(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR) -> windows_core::Result<IWiaItem2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SelectDeviceDlg(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR, ppitemroot: *mut Option<IWiaItem2>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SelectDeviceDlg)(windows_core::Interface::as_raw(self), hwndparent, ldevicetype, lflags, core::mem::transmute(pbstrdeviceid), core::mem::transmute(ppitemroot)).ok() }
    }
    pub unsafe fn SelectDeviceDlgID(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SelectDeviceDlgID)(windows_core::Interface::as_raw(self), hwndparent, ldevicetype, lflags, core::mem::transmute(pbstrdeviceid)).ok() }
    }
    pub unsafe fn RegisterEventCallbackInterface<P3>(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, piwiaeventcallback: P3) -> windows_core::Result<windows_core::IUnknown>
    where
        P3: windows_core::Param<IWiaEventCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterEventCallbackInterface)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, piwiaeventcallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterEventCallbackProgram(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, bstrfullappname: &windows_core::BSTR, bstrcommandlinearg: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterEventCallbackProgram)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, core::mem::transmute_copy(bstrfullappname), core::mem::transmute_copy(bstrcommandlinearg), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrdescription), core::mem::transmute_copy(bstricon)).ok() }
    }
    pub unsafe fn RegisterEventCallbackCLSID(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterEventCallbackCLSID)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), peventguid, pclsid, core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrdescription), core::mem::transmute_copy(bstricon)).ok() }
    }
    pub unsafe fn GetImageDlg(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, hwndparent: super::super::Foundation::HWND, bstrfoldername: &windows_core::BSTR, bstrfilename: &windows_core::BSTR, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: *mut Option<IWiaItem2>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetImageDlg)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrdeviceid), hwndparent, core::mem::transmute_copy(bstrfoldername), core::mem::transmute_copy(bstrfilename), plnumfiles as _, ppbstrfilepaths as _, core::mem::transmute(ppitem)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDevMgr2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectDeviceDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, i32, i32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectDeviceDlgID: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterEventCallbackInterface: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterEventCallbackProgram: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterEventCallbackCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetImageDlg: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32, *mut *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWiaDevMgr2_Impl: windows_core::IUnknownImpl {
    fn EnumDeviceInfo(&self, lflags: i32) -> windows_core::Result<IEnumWIA_DEV_INFO>;
    fn CreateDevice(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR) -> windows_core::Result<IWiaItem2>;
    fn SelectDeviceDlg(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR, ppitemroot: windows_core::OutRef<IWiaItem2>) -> windows_core::Result<()>;
    fn SelectDeviceDlgID(&self, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn RegisterEventCallbackInterface(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, piwiaeventcallback: windows_core::Ref<IWiaEventCallback>) -> windows_core::Result<windows_core::IUnknown>;
    fn RegisterEventCallbackProgram(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, bstrfullappname: &windows_core::BSTR, bstrcommandlinearg: &windows_core::BSTR, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RegisterEventCallbackCLSID(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: &windows_core::BSTR, bstrdescription: &windows_core::BSTR, bstricon: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetImageDlg(&self, lflags: i32, bstrdeviceid: &windows_core::BSTR, hwndparent: super::super::Foundation::HWND, bstrfoldername: &windows_core::BSTR, bstrfilename: &windows_core::BSTR, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: windows_core::OutRef<IWiaItem2>) -> windows_core::Result<()>;
}
impl IWiaDevMgr2_Vtbl {
    pub const fn new<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumDeviceInfo<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr2_Impl::EnumDeviceInfo(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDevice<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, ppwiaitem2root: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr2_Impl::CreateDevice(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid)) {
                    Ok(ok__) => {
                        ppwiaitem2root.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SelectDeviceDlg<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut *mut core::ffi::c_void, ppitemroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr2_Impl::SelectDeviceDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pbstrdeviceid), core::mem::transmute_copy(&ppitemroot)).into()
            }
        }
        unsafe extern "system" fn SelectDeviceDlgID<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ldevicetype: i32, lflags: i32, pbstrdeviceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr2_Impl::SelectDeviceDlgID(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&ldevicetype), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pbstrdeviceid)).into()
            }
        }
        unsafe extern "system" fn RegisterEventCallbackInterface<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, piwiaeventcallback: *mut core::ffi::c_void, peventobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDevMgr2_Impl::RegisterEventCallbackInterface(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&piwiaeventcallback)) {
                    Ok(ok__) => {
                        peventobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterEventCallbackProgram<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, bstrfullappname: *mut core::ffi::c_void, bstrcommandlinearg: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void, bstricon: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr2_Impl::RegisterEventCallbackProgram(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute(&bstrfullappname), core::mem::transmute(&bstrcommandlinearg), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
            }
        }
        unsafe extern "system" fn RegisterEventCallbackCLSID<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, pclsid: *const windows_core::GUID, bstrname: *mut core::ffi::c_void, bstrdescription: *mut core::ffi::c_void, bstricon: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr2_Impl::RegisterEventCallbackCLSID(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&peventguid), core::mem::transmute_copy(&pclsid), core::mem::transmute(&bstrname), core::mem::transmute(&bstrdescription), core::mem::transmute(&bstricon)).into()
            }
        }
        unsafe extern "system" fn GetImageDlg<Identity: IWiaDevMgr2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrdeviceid: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, bstrfoldername: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDevMgr2_Impl::GetImageDlg(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&hwndparent), core::mem::transmute(&bstrfoldername), core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&plnumfiles), core::mem::transmute_copy(&ppbstrfilepaths), core::mem::transmute_copy(&ppitem)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumDeviceInfo: EnumDeviceInfo::<Identity, OFFSET>,
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            SelectDeviceDlg: SelectDeviceDlg::<Identity, OFFSET>,
            SelectDeviceDlgID: SelectDeviceDlgID::<Identity, OFFSET>,
            RegisterEventCallbackInterface: RegisterEventCallbackInterface::<Identity, OFFSET>,
            RegisterEventCallbackProgram: RegisterEventCallbackProgram::<Identity, OFFSET>,
            RegisterEventCallbackCLSID: RegisterEventCallbackCLSID::<Identity, OFFSET>,
            GetImageDlg: GetImageDlg::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaDevMgr2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaDevMgr2 {}
windows_core::imp::define_interface!(IWiaDrvItem, IWiaDrvItem_Vtbl, 0x1f02b5c5_b00c_11d2_a094_00c04f72dc3c);
windows_core::imp::interface_hierarchy!(IWiaDrvItem, windows_core::IUnknown);
impl IWiaDrvItem {
    pub unsafe fn GetItemFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDeviceSpecContext(&self) -> windows_core::Result<*mut u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceSpecContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFullItemName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFullItemName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetItemName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AddItemToFolder<P0>(&self, __midl__iwiadrvitem0004: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWiaDrvItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddItemToFolder)(windows_core::Interface::as_raw(self), __midl__iwiadrvitem0004.param().abi()).ok() }
    }
    pub unsafe fn UnlinkItemTree(&self, __midl__iwiadrvitem0005: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlinkItemTree)(windows_core::Interface::as_raw(self), __midl__iwiadrvitem0005).ok() }
    }
    pub unsafe fn RemoveItemFromFolder(&self, __midl__iwiadrvitem0006: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveItemFromFolder)(windows_core::Interface::as_raw(self), __midl__iwiadrvitem0006).ok() }
    }
    pub unsafe fn FindItemByName(&self, __midl__iwiadrvitem0007: i32, __midl__iwiadrvitem0008: &windows_core::BSTR) -> windows_core::Result<IWiaDrvItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindItemByName)(windows_core::Interface::as_raw(self), __midl__iwiadrvitem0007, core::mem::transmute_copy(__midl__iwiadrvitem0008), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindChildItemByName(&self, __midl__iwiadrvitem0010: &windows_core::BSTR) -> windows_core::Result<IWiaDrvItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindChildItemByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(__midl__iwiadrvitem0010), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetParentItem(&self) -> windows_core::Result<IWiaDrvItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFirstChildItem(&self) -> windows_core::Result<IWiaDrvItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstChildItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNextSiblingItem(&self) -> windows_core::Result<IWiaDrvItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextSiblingItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DumpItemData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DumpItemData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaDrvItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItemFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetDeviceSpecContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8) -> windows_core::HRESULT,
    pub GetFullItemName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItemName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddItemToFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnlinkItemTree: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub RemoveItemFromFolder: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FindItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindChildItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParentItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFirstChildItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextSiblingItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DumpItemData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWiaDrvItem_Impl: windows_core::IUnknownImpl {
    fn GetItemFlags(&self) -> windows_core::Result<i32>;
    fn GetDeviceSpecContext(&self) -> windows_core::Result<*mut u8>;
    fn GetFullItemName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetItemName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AddItemToFolder(&self, __midl__iwiadrvitem0004: windows_core::Ref<IWiaDrvItem>) -> windows_core::Result<()>;
    fn UnlinkItemTree(&self, __midl__iwiadrvitem0005: i32) -> windows_core::Result<()>;
    fn RemoveItemFromFolder(&self, __midl__iwiadrvitem0006: i32) -> windows_core::Result<()>;
    fn FindItemByName(&self, __midl__iwiadrvitem0007: i32, __midl__iwiadrvitem0008: &windows_core::BSTR) -> windows_core::Result<IWiaDrvItem>;
    fn FindChildItemByName(&self, __midl__iwiadrvitem0010: &windows_core::BSTR) -> windows_core::Result<IWiaDrvItem>;
    fn GetParentItem(&self) -> windows_core::Result<IWiaDrvItem>;
    fn GetFirstChildItem(&self) -> windows_core::Result<IWiaDrvItem>;
    fn GetNextSiblingItem(&self) -> windows_core::Result<IWiaDrvItem>;
    fn DumpItemData(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IWiaDrvItem_Vtbl {
    pub const fn new<Identity: IWiaDrvItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItemFlags<Identity: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0000: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDrvItem_Impl::GetItemFlags(this) {
                    Ok(ok__) => {
                        __midl__iwiadrvitem0000.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceSpecContext<Identity: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0001: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDrvItem_Impl::GetDeviceSpecContext(this) {
                    Ok(ok__) => {
                        __midl__iwiadrvitem0001.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFullItemName<Identity: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0002: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDrvItem_Impl::GetFullItemName(this) {
                    Ok(ok__) => {
                        __midl__iwiadrvitem0002.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItemName<Identity: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0003: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDrvItem_Impl::GetItemName(this) {
                    Ok(ok__) => {
                        __midl__iwiadrvitem0003.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddItemToFolder<Identity: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0004: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDrvItem_Impl::AddItemToFolder(this, core::mem::transmute_copy(&__midl__iwiadrvitem0004)).into()
            }
        }
        unsafe extern "system" fn UnlinkItemTree<Identity: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0005: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDrvItem_Impl::UnlinkItemTree(this, core::mem::transmute_copy(&__midl__iwiadrvitem0005)).into()
            }
        }
        unsafe extern "system" fn RemoveItemFromFolder<Identity: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0006: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaDrvItem_Impl::RemoveItemFromFolder(this, core::mem::transmute_copy(&__midl__iwiadrvitem0006)).into()
            }
        }
        unsafe extern "system" fn FindItemByName<Identity: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0007: i32, __midl__iwiadrvitem0008: *mut core::ffi::c_void, __midl__iwiadrvitem0009: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDrvItem_Impl::FindItemByName(this, core::mem::transmute_copy(&__midl__iwiadrvitem0007), core::mem::transmute(&__midl__iwiadrvitem0008)) {
                    Ok(ok__) => {
                        __midl__iwiadrvitem0009.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindChildItemByName<Identity: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0010: *mut core::ffi::c_void, __midl__iwiadrvitem0011: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDrvItem_Impl::FindChildItemByName(this, core::mem::transmute(&__midl__iwiadrvitem0010)) {
                    Ok(ok__) => {
                        __midl__iwiadrvitem0011.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParentItem<Identity: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0012: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDrvItem_Impl::GetParentItem(this) {
                    Ok(ok__) => {
                        __midl__iwiadrvitem0012.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFirstChildItem<Identity: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0013: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDrvItem_Impl::GetFirstChildItem(this) {
                    Ok(ok__) => {
                        __midl__iwiadrvitem0013.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNextSiblingItem<Identity: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0014: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDrvItem_Impl::GetNextSiblingItem(this) {
                    Ok(ok__) => {
                        __midl__iwiadrvitem0014.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DumpItemData<Identity: IWiaDrvItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiadrvitem0015: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaDrvItem_Impl::DumpItemData(this) {
                    Ok(ok__) => {
                        __midl__iwiadrvitem0015.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemFlags: GetItemFlags::<Identity, OFFSET>,
            GetDeviceSpecContext: GetDeviceSpecContext::<Identity, OFFSET>,
            GetFullItemName: GetFullItemName::<Identity, OFFSET>,
            GetItemName: GetItemName::<Identity, OFFSET>,
            AddItemToFolder: AddItemToFolder::<Identity, OFFSET>,
            UnlinkItemTree: UnlinkItemTree::<Identity, OFFSET>,
            RemoveItemFromFolder: RemoveItemFromFolder::<Identity, OFFSET>,
            FindItemByName: FindItemByName::<Identity, OFFSET>,
            FindChildItemByName: FindChildItemByName::<Identity, OFFSET>,
            GetParentItem: GetParentItem::<Identity, OFFSET>,
            GetFirstChildItem: GetFirstChildItem::<Identity, OFFSET>,
            GetNextSiblingItem: GetNextSiblingItem::<Identity, OFFSET>,
            DumpItemData: DumpItemData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaDrvItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaDrvItem {}
windows_core::imp::define_interface!(IWiaErrorHandler, IWiaErrorHandler_Vtbl, 0x0e4a51b1_bc1f_443d_a835_72e890759ef3);
windows_core::imp::interface_hierarchy!(IWiaErrorHandler, windows_core::IUnknown);
impl IWiaErrorHandler {
    pub unsafe fn ReportStatus<P2>(&self, lflags: i32, hwndparent: super::super::Foundation::HWND, pwiaitem2: P2, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<IWiaItem2>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReportStatus)(windows_core::Interface::as_raw(self), lflags, hwndparent, pwiaitem2.param().abi(), hrstatus, lpercentcomplete).ok() }
    }
    pub unsafe fn GetStatusDescription<P1>(&self, lflags: i32, pwiaitem2: P1, hrstatus: windows_core::HRESULT) -> windows_core::Result<windows_core::BSTR>
    where
        P1: windows_core::Param<IWiaItem2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatusDescription)(windows_core::Interface::as_raw(self), lflags, pwiaitem2.param().abi(), hrstatus, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaErrorHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReportStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::Foundation::HWND, *mut core::ffi::c_void, windows_core::HRESULT, i32) -> windows_core::HRESULT,
    pub GetStatusDescription: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, windows_core::HRESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWiaErrorHandler_Impl: windows_core::IUnknownImpl {
    fn ReportStatus(&self, lflags: i32, hwndparent: super::super::Foundation::HWND, pwiaitem2: windows_core::Ref<IWiaItem2>, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::Result<()>;
    fn GetStatusDescription(&self, lflags: i32, pwiaitem2: windows_core::Ref<IWiaItem2>, hrstatus: windows_core::HRESULT) -> windows_core::Result<windows_core::BSTR>;
}
impl IWiaErrorHandler_Vtbl {
    pub const fn new<Identity: IWiaErrorHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReportStatus<Identity: IWiaErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, hwndparent: super::super::Foundation::HWND, pwiaitem2: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, lpercentcomplete: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaErrorHandler_Impl::ReportStatus(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&pwiaitem2), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&lpercentcomplete)).into()
            }
        }
        unsafe extern "system" fn GetStatusDescription<Identity: IWiaErrorHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiaitem2: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaErrorHandler_Impl::GetStatusDescription(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiaitem2), core::mem::transmute_copy(&hrstatus)) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReportStatus: ReportStatus::<Identity, OFFSET>,
            GetStatusDescription: GetStatusDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaErrorHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaErrorHandler {}
windows_core::imp::define_interface!(IWiaEventCallback, IWiaEventCallback_Vtbl, 0xae6287b0_0084_11d2_973b_00a0c9068f2e);
windows_core::imp::interface_hierarchy!(IWiaEventCallback, windows_core::IUnknown);
impl IWiaEventCallback {
    pub unsafe fn ImageEventCallback(&self, peventguid: *const windows_core::GUID, bstreventdescription: &windows_core::BSTR, bstrdeviceid: &windows_core::BSTR, bstrdevicedescription: &windows_core::BSTR, dwdevicetype: u32, bstrfullitemname: &windows_core::BSTR, puleventtype: *mut u32, ulreserved: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ImageEventCallback)(windows_core::Interface::as_raw(self), peventguid, core::mem::transmute_copy(bstreventdescription), core::mem::transmute_copy(bstrdeviceid), core::mem::transmute_copy(bstrdevicedescription), dwdevicetype, core::mem::transmute_copy(bstrfullitemname), puleventtype as _, ulreserved).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaEventCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ImageEventCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32, u32) -> windows_core::HRESULT,
}
pub trait IWiaEventCallback_Impl: windows_core::IUnknownImpl {
    fn ImageEventCallback(&self, peventguid: *const windows_core::GUID, bstreventdescription: &windows_core::BSTR, bstrdeviceid: &windows_core::BSTR, bstrdevicedescription: &windows_core::BSTR, dwdevicetype: u32, bstrfullitemname: &windows_core::BSTR, puleventtype: *mut u32, ulreserved: u32) -> windows_core::Result<()>;
}
impl IWiaEventCallback_Vtbl {
    pub const fn new<Identity: IWiaEventCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ImageEventCallback<Identity: IWiaEventCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, bstreventdescription: *mut core::ffi::c_void, bstrdeviceid: *mut core::ffi::c_void, bstrdevicedescription: *mut core::ffi::c_void, dwdevicetype: u32, bstrfullitemname: *mut core::ffi::c_void, puleventtype: *mut u32, ulreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaEventCallback_Impl::ImageEventCallback(this, core::mem::transmute_copy(&peventguid), core::mem::transmute(&bstreventdescription), core::mem::transmute(&bstrdeviceid), core::mem::transmute(&bstrdevicedescription), core::mem::transmute_copy(&dwdevicetype), core::mem::transmute(&bstrfullitemname), core::mem::transmute_copy(&puleventtype), core::mem::transmute_copy(&ulreserved)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ImageEventCallback: ImageEventCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaEventCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaEventCallback {}
windows_core::imp::define_interface!(IWiaImageFilter, IWiaImageFilter_Vtbl, 0xa8a79ffa_450b_41f1_8f87_849ccd94ebf6);
windows_core::imp::interface_hierarchy!(IWiaImageFilter, windows_core::IUnknown);
impl IWiaImageFilter {
    pub unsafe fn InitializeFilter<P0, P1>(&self, pwiaitem2: P0, pwiatransfercallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWiaItem2>,
        P1: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeFilter)(windows_core::Interface::as_raw(self), pwiaitem2.param().abi(), pwiatransfercallback.param().abi()).ok() }
    }
    pub unsafe fn SetNewCallback<P0>(&self, pwiatransfercallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNewCallback)(windows_core::Interface::as_raw(self), pwiatransfercallback.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FilterPreviewImage<P1, P3>(&self, lflags: i32, pwiachilditem2: P1, inputimageextents: super::super::Foundation::RECT, pinputstream: P3) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWiaItem2>,
        P3: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).FilterPreviewImage)(windows_core::Interface::as_raw(self), lflags, pwiachilditem2.param().abi(), core::mem::transmute(inputimageextents), pinputstream.param().abi()).ok() }
    }
    pub unsafe fn ApplyProperties<P0>(&self, pwiapropertystorage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWiaPropertyStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).ApplyProperties)(windows_core::Interface::as_raw(self), pwiapropertystorage.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaImageFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNewCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FilterPreviewImage: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, super::super::Foundation::RECT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FilterPreviewImage: usize,
    pub ApplyProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaImageFilter_Impl: windows_core::IUnknownImpl {
    fn InitializeFilter(&self, pwiaitem2: windows_core::Ref<IWiaItem2>, pwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn SetNewCallback(&self, pwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn FilterPreviewImage(&self, lflags: i32, pwiachilditem2: windows_core::Ref<IWiaItem2>, inputimageextents: &super::super::Foundation::RECT, pinputstream: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn ApplyProperties(&self, pwiapropertystorage: windows_core::Ref<IWiaPropertyStorage>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWiaImageFilter_Vtbl {
    pub const fn new<Identity: IWiaImageFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeFilter<Identity: IWiaImageFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwiaitem2: *mut core::ffi::c_void, pwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaImageFilter_Impl::InitializeFilter(this, core::mem::transmute_copy(&pwiaitem2), core::mem::transmute_copy(&pwiatransfercallback)).into()
            }
        }
        unsafe extern "system" fn SetNewCallback<Identity: IWiaImageFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaImageFilter_Impl::SetNewCallback(this, core::mem::transmute_copy(&pwiatransfercallback)).into()
            }
        }
        unsafe extern "system" fn FilterPreviewImage<Identity: IWiaImageFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiachilditem2: *mut core::ffi::c_void, inputimageextents: super::super::Foundation::RECT, pinputstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaImageFilter_Impl::FilterPreviewImage(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiachilditem2), core::mem::transmute(&inputimageextents), core::mem::transmute_copy(&pinputstream)).into()
            }
        }
        unsafe extern "system" fn ApplyProperties<Identity: IWiaImageFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwiapropertystorage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaImageFilter_Impl::ApplyProperties(this, core::mem::transmute_copy(&pwiapropertystorage)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeFilter: InitializeFilter::<Identity, OFFSET>,
            SetNewCallback: SetNewCallback::<Identity, OFFSET>,
            FilterPreviewImage: FilterPreviewImage::<Identity, OFFSET>,
            ApplyProperties: ApplyProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaImageFilter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWiaImageFilter {}
windows_core::imp::define_interface!(IWiaItem, IWiaItem_Vtbl, 0x4db1ad10_3391_11d2_9a33_00c04fa36145);
windows_core::imp::interface_hierarchy!(IWiaItem, windows_core::IUnknown);
impl IWiaItem {
    pub unsafe fn GetItemType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AnalyzeItem(&self, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AnalyzeItem)(windows_core::Interface::as_raw(self), lflags).ok() }
    }
    pub unsafe fn EnumChildItems(&self) -> windows_core::Result<IEnumWiaItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumChildItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteItem(&self, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), lflags).ok() }
    }
    pub unsafe fn CreateChildItem(&self, lflags: i32, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateChildItem)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstritemname), core::mem::transmute_copy(bstrfullitemname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const windows_core::GUID) -> windows_core::Result<IEnumWIA_DEV_CAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRegisterEventInfo)(windows_core::Interface::as_raw(self), lflags, peventguid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindItemByName(&self, lflags: i32, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindItemByName)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrfullitemname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeviceDlg(&self, hwndparent: super::super::Foundation::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut Option<IWiaItem>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeviceDlg)(windows_core::Interface::as_raw(self), hwndparent, lflags, lintent, plitemcount as _, ppiwiaitem as _).ok() }
    }
    pub unsafe fn DeviceCommand(&self, lflags: i32, pcmdguid: *const windows_core::GUID, piwiaitem: *mut Option<IWiaItem>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeviceCommand)(windows_core::Interface::as_raw(self), lflags, pcmdguid, core::mem::transmute(piwiaitem)).ok() }
    }
    pub unsafe fn GetRootItem(&self) -> windows_core::Result<IWiaItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumDeviceCapabilities(&self, lflags: i32) -> windows_core::Result<IEnumWIA_DEV_CAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDeviceCapabilities)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DumpItemData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DumpItemData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DumpDrvItemData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DumpDrvItemData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DumpTreeItemData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DumpTreeItemData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Diagnostic(&self, pbuffer: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Diagnostic)(windows_core::Interface::as_raw(self), pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItemType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AnalyzeItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnumChildItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CreateChildItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumRegisterEventInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceDlg: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, i32, i32, *mut i32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceCommand: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRootItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDeviceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DumpItemData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DumpDrvItemData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DumpTreeItemData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
}
pub trait IWiaItem_Impl: windows_core::IUnknownImpl {
    fn GetItemType(&self) -> windows_core::Result<i32>;
    fn AnalyzeItem(&self, lflags: i32) -> windows_core::Result<()>;
    fn EnumChildItems(&self) -> windows_core::Result<IEnumWiaItem>;
    fn DeleteItem(&self, lflags: i32) -> windows_core::Result<()>;
    fn CreateChildItem(&self, lflags: i32, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem>;
    fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const windows_core::GUID) -> windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn FindItemByName(&self, lflags: i32, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem>;
    fn DeviceDlg(&self, hwndparent: super::super::Foundation::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut Option<IWiaItem>) -> windows_core::Result<()>;
    fn DeviceCommand(&self, lflags: i32, pcmdguid: *const windows_core::GUID, piwiaitem: windows_core::OutRef<IWiaItem>) -> windows_core::Result<()>;
    fn GetRootItem(&self) -> windows_core::Result<IWiaItem>;
    fn EnumDeviceCapabilities(&self, lflags: i32) -> windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn DumpItemData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DumpDrvItemData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DumpTreeItemData(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Diagnostic(&self, ulsize: u32, pbuffer: *const u8) -> windows_core::Result<()>;
}
impl IWiaItem_Vtbl {
    pub const fn new<Identity: IWiaItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItemType<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::GetItemType(this) {
                    Ok(ok__) => {
                        pitemtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AnalyzeItem<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::AnalyzeItem(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn EnumChildItems<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienumwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::EnumChildItems(this) {
                    Ok(ok__) => {
                        ppienumwiaitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::DeleteItem(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn CreateChildItem<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstritemname: *mut core::ffi::c_void, bstrfullitemname: *mut core::ffi::c_void, ppiwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::CreateChildItem(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstritemname), core::mem::transmute(&bstrfullitemname)) {
                    Ok(ok__) => {
                        ppiwiaitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, peventguid: *const windows_core::GUID, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::EnumRegisterEventInfo(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&peventguid)) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindItemByName<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrfullitemname: *mut core::ffi::c_void, ppiwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::FindItemByName(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrfullitemname)) {
                    Ok(ok__) => {
                        ppiwiaitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceDlg<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, lflags: i32, lintent: i32, plitemcount: *mut i32, ppiwiaitem: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::DeviceDlg(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lintent), core::mem::transmute_copy(&plitemcount), core::mem::transmute_copy(&ppiwiaitem)).into()
            }
        }
        unsafe extern "system" fn DeviceCommand<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pcmdguid: *const windows_core::GUID, piwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::DeviceCommand(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcmdguid), core::mem::transmute_copy(&piwiaitem)).into()
            }
        }
        unsafe extern "system" fn GetRootItem<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::GetRootItem(this) {
                    Ok(ok__) => {
                        ppiwiaitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::EnumDeviceCapabilities(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppienumwia_dev_caps.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DumpItemData<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::DumpItemData(this) {
                    Ok(ok__) => {
                        bstrdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DumpDrvItemData<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::DumpDrvItemData(this) {
                    Ok(ok__) => {
                        bstrdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DumpTreeItemData<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem_Impl::DumpTreeItemData(this) {
                    Ok(ok__) => {
                        bstrdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Diagnostic<Identity: IWiaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem_Impl::Diagnostic(this, core::mem::transmute_copy(&ulsize), core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemType: GetItemType::<Identity, OFFSET>,
            AnalyzeItem: AnalyzeItem::<Identity, OFFSET>,
            EnumChildItems: EnumChildItems::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
            CreateChildItem: CreateChildItem::<Identity, OFFSET>,
            EnumRegisterEventInfo: EnumRegisterEventInfo::<Identity, OFFSET>,
            FindItemByName: FindItemByName::<Identity, OFFSET>,
            DeviceDlg: DeviceDlg::<Identity, OFFSET>,
            DeviceCommand: DeviceCommand::<Identity, OFFSET>,
            GetRootItem: GetRootItem::<Identity, OFFSET>,
            EnumDeviceCapabilities: EnumDeviceCapabilities::<Identity, OFFSET>,
            DumpItemData: DumpItemData::<Identity, OFFSET>,
            DumpDrvItemData: DumpDrvItemData::<Identity, OFFSET>,
            DumpTreeItemData: DumpTreeItemData::<Identity, OFFSET>,
            Diagnostic: Diagnostic::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaItem {}
windows_core::imp::define_interface!(IWiaItem2, IWiaItem2_Vtbl, 0x6cba0075_1287_407d_9b77_cf0e030435cc);
windows_core::imp::interface_hierarchy!(IWiaItem2, windows_core::IUnknown);
impl IWiaItem2 {
    pub unsafe fn CreateChildItem(&self, litemflags: i32, lcreationflags: i32, bstritemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateChildItem)(windows_core::Interface::as_raw(self), litemflags, lcreationflags, core::mem::transmute_copy(bstritemname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteItem(&self, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), lflags).ok() }
    }
    pub unsafe fn EnumChildItems(&self, pcategoryguid: Option<*const windows_core::GUID>) -> windows_core::Result<IEnumWiaItem2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumChildItems)(windows_core::Interface::as_raw(self), pcategoryguid.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindItemByName(&self, lflags: i32, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindItemByName)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrfullitemname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetItemCategory(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetItemType(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeviceDlg(&self, lflags: i32, hwndparent: super::super::Foundation::HWND, bstrfoldername: &windows_core::BSTR, bstrfilename: &windows_core::BSTR, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: Option<*mut Option<IWiaItem2>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeviceDlg)(windows_core::Interface::as_raw(self), lflags, hwndparent, core::mem::transmute_copy(bstrfoldername), core::mem::transmute_copy(bstrfilename), plnumfiles as _, ppbstrfilepaths as _, ppitem.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn DeviceCommand(&self, lflags: i32, pcmdguid: *const windows_core::GUID, ppiwiaitem2: *mut Option<IWiaItem2>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeviceCommand)(windows_core::Interface::as_raw(self), lflags, pcmdguid, core::mem::transmute(ppiwiaitem2)).ok() }
    }
    pub unsafe fn EnumDeviceCapabilities(&self, lflags: i32) -> windows_core::Result<IEnumWIA_DEV_CAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDeviceCapabilities)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CheckExtension(&self, lflags: i32, bstrname: &windows_core::BSTR, riidextensioninterface: *const windows_core::GUID, pbextensionexists: *mut windows_core::BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CheckExtension)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrname), riidextensioninterface, pbextensionexists as _).ok() }
    }
    pub unsafe fn GetExtension(&self, lflags: i32, bstrname: &windows_core::BSTR, riidextensioninterface: *const windows_core::GUID, ppout: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetExtension)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstrname), riidextensioninterface, ppout as _).ok() }
    }
    pub unsafe fn GetParentItem(&self) -> windows_core::Result<IWiaItem2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRootItem(&self) -> windows_core::Result<IWiaItem2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPreviewComponent(&self, lflags: i32) -> windows_core::Result<IWiaPreview> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreviewComponent)(windows_core::Interface::as_raw(self), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const windows_core::GUID) -> windows_core::Result<IEnumWIA_DEV_CAPS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRegisterEventInfo)(windows_core::Interface::as_raw(self), lflags, peventguid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Diagnostic(&self, pbuffer: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Diagnostic)(windows_core::Interface::as_raw(self), pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItem2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateChildItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnumChildItems: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetItemCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetItemType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DeviceDlg: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32, *mut *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceCommand: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDeviceCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CheckExtension: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetExtension: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParentItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRootItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPreviewComponent: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumRegisterEventInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Diagnostic: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
}
pub trait IWiaItem2_Impl: windows_core::IUnknownImpl {
    fn CreateChildItem(&self, litemflags: i32, lcreationflags: i32, bstritemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem2>;
    fn DeleteItem(&self, lflags: i32) -> windows_core::Result<()>;
    fn EnumChildItems(&self, pcategoryguid: *const windows_core::GUID) -> windows_core::Result<IEnumWiaItem2>;
    fn FindItemByName(&self, lflags: i32, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<IWiaItem2>;
    fn GetItemCategory(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetItemType(&self) -> windows_core::Result<i32>;
    fn DeviceDlg(&self, lflags: i32, hwndparent: super::super::Foundation::HWND, bstrfoldername: &windows_core::BSTR, bstrfilename: &windows_core::BSTR, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut windows_core::BSTR, ppitem: windows_core::OutRef<IWiaItem2>) -> windows_core::Result<()>;
    fn DeviceCommand(&self, lflags: i32, pcmdguid: *const windows_core::GUID, ppiwiaitem2: windows_core::OutRef<IWiaItem2>) -> windows_core::Result<()>;
    fn EnumDeviceCapabilities(&self, lflags: i32) -> windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn CheckExtension(&self, lflags: i32, bstrname: &windows_core::BSTR, riidextensioninterface: *const windows_core::GUID, pbextensionexists: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetExtension(&self, lflags: i32, bstrname: &windows_core::BSTR, riidextensioninterface: *const windows_core::GUID, ppout: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetParentItem(&self) -> windows_core::Result<IWiaItem2>;
    fn GetRootItem(&self) -> windows_core::Result<IWiaItem2>;
    fn GetPreviewComponent(&self, lflags: i32) -> windows_core::Result<IWiaPreview>;
    fn EnumRegisterEventInfo(&self, lflags: i32, peventguid: *const windows_core::GUID) -> windows_core::Result<IEnumWIA_DEV_CAPS>;
    fn Diagnostic(&self, ulsize: u32, pbuffer: *const u8) -> windows_core::Result<()>;
}
impl IWiaItem2_Vtbl {
    pub const fn new<Identity: IWiaItem2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateChildItem<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, litemflags: i32, lcreationflags: i32, bstritemname: *mut core::ffi::c_void, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::CreateChildItem(this, core::mem::transmute_copy(&litemflags), core::mem::transmute_copy(&lcreationflags), core::mem::transmute(&bstritemname)) {
                    Ok(ok__) => {
                        ppiwiaitem2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::DeleteItem(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn EnumChildItems<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcategoryguid: *const windows_core::GUID, ppienumwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::EnumChildItems(this, core::mem::transmute_copy(&pcategoryguid)) {
                    Ok(ok__) => {
                        ppienumwiaitem2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindItemByName<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrfullitemname: *mut core::ffi::c_void, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::FindItemByName(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrfullitemname)) {
                    Ok(ok__) => {
                        ppiwiaitem2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItemCategory<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemcategoryguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::GetItemCategory(this) {
                    Ok(ok__) => {
                        pitemcategoryguid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItemType<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemtype: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::GetItemType(this) {
                    Ok(ok__) => {
                        pitemtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceDlg<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, hwndparent: super::super::Foundation::HWND, bstrfoldername: *mut core::ffi::c_void, bstrfilename: *mut core::ffi::c_void, plnumfiles: *mut i32, ppbstrfilepaths: *mut *mut *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::DeviceDlg(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&hwndparent), core::mem::transmute(&bstrfoldername), core::mem::transmute(&bstrfilename), core::mem::transmute_copy(&plnumfiles), core::mem::transmute_copy(&ppbstrfilepaths), core::mem::transmute_copy(&ppitem)).into()
            }
        }
        unsafe extern "system" fn DeviceCommand<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pcmdguid: *const windows_core::GUID, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::DeviceCommand(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcmdguid), core::mem::transmute_copy(&ppiwiaitem2)).into()
            }
        }
        unsafe extern "system" fn EnumDeviceCapabilities<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppienumwia_dev_caps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::EnumDeviceCapabilities(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppienumwia_dev_caps.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckExtension<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrname: *mut core::ffi::c_void, riidextensioninterface: *const windows_core::GUID, pbextensionexists: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::CheckExtension(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrname), core::mem::transmute_copy(&riidextensioninterface), core::mem::transmute_copy(&pbextensionexists)).into()
            }
        }
        unsafe extern "system" fn GetExtension<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstrname: *mut core::ffi::c_void, riidextensioninterface: *const windows_core::GUID, ppout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::GetExtension(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstrname), core::mem::transmute_copy(&riidextensioninterface), core::mem::transmute_copy(&ppout)).into()
            }
        }
        unsafe extern "system" fn GetParentItem<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::GetParentItem(this) {
                    Ok(ok__) => {
                        ppiwiaitem2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRootItem<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwiaitem2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::GetRootItem(this) {
                    Ok(ok__) => {
                        ppiwiaitem2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreviewComponent<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, ppwiapreview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::GetPreviewComponent(this, core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        ppwiapreview.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumRegisterEventInfo<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, peventguid: *const windows_core::GUID, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItem2_Impl::EnumRegisterEventInfo(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&peventguid)) {
                    Ok(ok__) => {
                        ppienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Diagnostic<Identity: IWiaItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsize: u32, pbuffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItem2_Impl::Diagnostic(this, core::mem::transmute_copy(&ulsize), core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateChildItem: CreateChildItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
            EnumChildItems: EnumChildItems::<Identity, OFFSET>,
            FindItemByName: FindItemByName::<Identity, OFFSET>,
            GetItemCategory: GetItemCategory::<Identity, OFFSET>,
            GetItemType: GetItemType::<Identity, OFFSET>,
            DeviceDlg: DeviceDlg::<Identity, OFFSET>,
            DeviceCommand: DeviceCommand::<Identity, OFFSET>,
            EnumDeviceCapabilities: EnumDeviceCapabilities::<Identity, OFFSET>,
            CheckExtension: CheckExtension::<Identity, OFFSET>,
            GetExtension: GetExtension::<Identity, OFFSET>,
            GetParentItem: GetParentItem::<Identity, OFFSET>,
            GetRootItem: GetRootItem::<Identity, OFFSET>,
            GetPreviewComponent: GetPreviewComponent::<Identity, OFFSET>,
            EnumRegisterEventInfo: EnumRegisterEventInfo::<Identity, OFFSET>,
            Diagnostic: Diagnostic::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaItem2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaItem2 {}
windows_core::imp::define_interface!(IWiaItemExtras, IWiaItemExtras_Vtbl, 0x6291ef2c_36ef_4532_876a_8e132593778d);
windows_core::imp::interface_hierarchy!(IWiaItemExtras, windows_core::IUnknown);
impl IWiaItemExtras {
    pub unsafe fn GetExtendedErrorInfo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExtendedErrorInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Escape(&self, dwescapecode: u32, lpindata: &[u8], poutdata: &mut [u8], pdwactualdatasize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Escape)(windows_core::Interface::as_raw(self), dwescapecode, core::mem::transmute(lpindata.as_ptr()), lpindata.len().try_into().unwrap(), core::mem::transmute(poutdata.as_ptr()), poutdata.len().try_into().unwrap(), pdwactualdatasize as _).ok() }
    }
    pub unsafe fn CancelPendingIO(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelPendingIO)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaItemExtras_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetExtendedErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub CancelPendingIO: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWiaItemExtras_Impl: windows_core::IUnknownImpl {
    fn GetExtendedErrorInfo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Escape(&self, dwescapecode: u32, lpindata: *const u8, cbindatasize: u32, poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> windows_core::Result<()>;
    fn CancelPendingIO(&self) -> windows_core::Result<()>;
}
impl IWiaItemExtras_Vtbl {
    pub const fn new<Identity: IWiaItemExtras_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetExtendedErrorInfo<Identity: IWiaItemExtras_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrerrortext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaItemExtras_Impl::GetExtendedErrorInfo(this) {
                    Ok(ok__) => {
                        bstrerrortext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Escape<Identity: IWiaItemExtras_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwescapecode: u32, lpindata: *const u8, cbindatasize: u32, poutdata: *mut u8, dwoutdatasize: u32, pdwactualdatasize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItemExtras_Impl::Escape(this, core::mem::transmute_copy(&dwescapecode), core::mem::transmute_copy(&lpindata), core::mem::transmute_copy(&cbindatasize), core::mem::transmute_copy(&poutdata), core::mem::transmute_copy(&dwoutdatasize), core::mem::transmute_copy(&pdwactualdatasize)).into()
            }
        }
        unsafe extern "system" fn CancelPendingIO<Identity: IWiaItemExtras_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaItemExtras_Impl::CancelPendingIO(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetExtendedErrorInfo: GetExtendedErrorInfo::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            CancelPendingIO: CancelPendingIO::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaItemExtras as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaItemExtras {}
windows_core::imp::define_interface!(IWiaLog, IWiaLog_Vtbl, 0xa00c10b6_82a1_452f_8b6c_86062aad6890);
windows_core::imp::interface_hierarchy!(IWiaLog, windows_core::IUnknown);
impl IWiaLog {
    pub unsafe fn InitializeLog(&self, hinstance: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitializeLog)(windows_core::Interface::as_raw(self), hinstance).ok() }
    }
    pub unsafe fn hResult(&self, hresult: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).hResult)(windows_core::Interface::as_raw(self), hresult).ok() }
    }
    pub unsafe fn Log(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Log)(windows_core::Interface::as_raw(self), lflags, lresid, ldetail, core::mem::transmute_copy(bstrtext)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaLog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializeLog: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub hResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub Log: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWiaLog_Impl: windows_core::IUnknownImpl {
    fn InitializeLog(&self, hinstance: i32) -> windows_core::Result<()>;
    fn hResult(&self, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Log(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl IWiaLog_Vtbl {
    pub const fn new<Identity: IWiaLog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeLog<Identity: IWiaLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinstance: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLog_Impl::InitializeLog(this, core::mem::transmute_copy(&hinstance)).into()
            }
        }
        unsafe extern "system" fn hResult<Identity: IWiaLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLog_Impl::hResult(this, core::mem::transmute_copy(&hresult)).into()
            }
        }
        unsafe extern "system" fn Log<Identity: IWiaLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLog_Impl::Log(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lresid), core::mem::transmute_copy(&ldetail), core::mem::transmute(&bstrtext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeLog: InitializeLog::<Identity, OFFSET>,
            hResult: hResult::<Identity, OFFSET>,
            Log: Log::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaLog as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaLog {}
windows_core::imp::define_interface!(IWiaLogEx, IWiaLogEx_Vtbl, 0xaf1f22ac_7a40_4787_b421_aeb47a1fbd0b);
windows_core::imp::interface_hierarchy!(IWiaLogEx, windows_core::IUnknown);
impl IWiaLogEx {
    pub unsafe fn InitializeLogEx(&self, hinstance: *const u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitializeLogEx)(windows_core::Interface::as_raw(self), hinstance).ok() }
    }
    pub unsafe fn hResult(&self, hresult: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).hResult)(windows_core::Interface::as_raw(self), hresult).ok() }
    }
    pub unsafe fn Log(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Log)(windows_core::Interface::as_raw(self), lflags, lresid, ldetail, core::mem::transmute_copy(bstrtext)).ok() }
    }
    pub unsafe fn hResultEx(&self, lmethodid: i32, hresult: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).hResultEx)(windows_core::Interface::as_raw(self), lmethodid, hresult).ok() }
    }
    pub unsafe fn LogEx(&self, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LogEx)(windows_core::Interface::as_raw(self), lmethodid, lflags, lresid, ldetail, core::mem::transmute_copy(bstrtext)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaLogEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializeLogEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8) -> windows_core::HRESULT,
    pub hResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub Log: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub hResultEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::HRESULT) -> windows_core::HRESULT,
    pub LogEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWiaLogEx_Impl: windows_core::IUnknownImpl {
    fn InitializeLogEx(&self, hinstance: *const u8) -> windows_core::Result<()>;
    fn hResult(&self, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn Log(&self, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn hResultEx(&self, lmethodid: i32, hresult: windows_core::HRESULT) -> windows_core::Result<()>;
    fn LogEx(&self, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl IWiaLogEx_Vtbl {
    pub const fn new<Identity: IWiaLogEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeLogEx<Identity: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hinstance: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLogEx_Impl::InitializeLogEx(this, core::mem::transmute_copy(&hinstance)).into()
            }
        }
        unsafe extern "system" fn hResult<Identity: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLogEx_Impl::hResult(this, core::mem::transmute_copy(&hresult)).into()
            }
        }
        unsafe extern "system" fn Log<Identity: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, lresid: i32, ldetail: i32, bstrtext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLogEx_Impl::Log(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lresid), core::mem::transmute_copy(&ldetail), core::mem::transmute(&bstrtext)).into()
            }
        }
        unsafe extern "system" fn hResultEx<Identity: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmethodid: i32, hresult: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLogEx_Impl::hResultEx(this, core::mem::transmute_copy(&lmethodid), core::mem::transmute_copy(&hresult)).into()
            }
        }
        unsafe extern "system" fn LogEx<Identity: IWiaLogEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmethodid: i32, lflags: i32, lresid: i32, ldetail: i32, bstrtext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaLogEx_Impl::LogEx(this, core::mem::transmute_copy(&lmethodid), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&lresid), core::mem::transmute_copy(&ldetail), core::mem::transmute(&bstrtext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeLogEx: InitializeLogEx::<Identity, OFFSET>,
            hResult: hResult::<Identity, OFFSET>,
            Log: Log::<Identity, OFFSET>,
            hResultEx: hResultEx::<Identity, OFFSET>,
            LogEx: LogEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaLogEx as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaLogEx {}
windows_core::imp::define_interface!(IWiaMiniDrv, IWiaMiniDrv_Vtbl, 0xd8cdee14_3c6c_11d2_9a35_00c04fa36145);
windows_core::imp::interface_hierarchy!(IWiaMiniDrv, windows_core::IUnknown);
impl IWiaMiniDrv {
    pub unsafe fn drvInitializeWia<P4, P5>(&self, __midl__iwiaminidrv0000: *const u8, __midl__iwiaminidrv0001: i32, __midl__iwiaminidrv0002: &windows_core::BSTR, __midl__iwiaminidrv0003: &windows_core::BSTR, __midl__iwiaminidrv0004: P4, __midl__iwiaminidrv0005: P5, __midl__iwiaminidrv0006: *mut Option<IWiaDrvItem>, __midl__iwiaminidrv0007: *mut Option<windows_core::IUnknown>, __midl__iwiaminidrv0008: *mut i32) -> windows_core::Result<()>
    where
        P4: windows_core::Param<windows_core::IUnknown>,
        P5: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).drvInitializeWia)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0000, __midl__iwiaminidrv0001, core::mem::transmute_copy(__midl__iwiaminidrv0002), core::mem::transmute_copy(__midl__iwiaminidrv0003), __midl__iwiaminidrv0004.param().abi(), __midl__iwiaminidrv0005.param().abi(), core::mem::transmute(__midl__iwiaminidrv0006), core::mem::transmute(__midl__iwiaminidrv0007), __midl__iwiaminidrv0008 as _).ok() }
    }
    pub unsafe fn drvAcquireItemData(&self, __midl__iwiaminidrv0009: *const u8, __midl__iwiaminidrv0010: i32, __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0012: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).drvAcquireItemData)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0009, __midl__iwiaminidrv0010, core::mem::transmute(__midl__iwiaminidrv0011), __midl__iwiaminidrv0012 as _).ok() }
    }
    pub unsafe fn drvInitItemProperties(&self, __midl__iwiaminidrv0013: *const u8, __midl__iwiaminidrv0014: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).drvInitItemProperties)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0013, __midl__iwiaminidrv0014, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn drvValidateItemProperties(&self, __midl__iwiaminidrv0016: *const u8, __midl__iwiaminidrv0017: i32, __midl__iwiaminidrv0018: u32, __midl__iwiaminidrv0019: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).drvValidateItemProperties)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0016, __midl__iwiaminidrv0017, __midl__iwiaminidrv0018, __midl__iwiaminidrv0019, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn drvWriteItemProperties(&self, __midl__iwiaminidrv0021: *const u8, __midl__iwiaminidrv0022: i32, __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).drvWriteItemProperties)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0021, __midl__iwiaminidrv0022, core::mem::transmute(__midl__iwiaminidrv0023), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn drvReadItemProperties(&self, __midl__iwiaminidrv0025: *const u8, __midl__iwiaminidrv0026: i32, __midl__iwiaminidrv0027: u32, __midl__iwiaminidrv0028: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).drvReadItemProperties)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0025, __midl__iwiaminidrv0026, __midl__iwiaminidrv0027, __midl__iwiaminidrv0028, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn drvLockWiaDevice(&self, __midl__iwiaminidrv0030: *const u8, __midl__iwiaminidrv0031: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).drvLockWiaDevice)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0030, __midl__iwiaminidrv0031, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn drvUnLockWiaDevice(&self, __midl__iwiaminidrv0033: *const u8, __midl__iwiaminidrv0034: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).drvUnLockWiaDevice)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0033, __midl__iwiaminidrv0034, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn drvAnalyzeItem(&self, __midl__iwiaminidrv0036: *const u8, __midl__iwiaminidrv0037: i32, __midl__iwiaminidrv0038: *const i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).drvAnalyzeItem)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0036, __midl__iwiaminidrv0037, __midl__iwiaminidrv0038).ok() }
    }
    pub unsafe fn drvGetDeviceErrorStr(&self, __midl__iwiaminidrv0039: i32, __midl__iwiaminidrv0040: i32, __midl__iwiaminidrv0041: *mut windows_core::PWSTR, __midl__iwiaminidrv0042: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).drvGetDeviceErrorStr)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0039, __midl__iwiaminidrv0040, __midl__iwiaminidrv0041 as _, __midl__iwiaminidrv0042 as _).ok() }
    }
    pub unsafe fn drvDeviceCommand(&self, __midl__iwiaminidrv0043: *const u8, __midl__iwiaminidrv0044: i32, __midl__iwiaminidrv0045: *const windows_core::GUID, __midl__iwiaminidrv0046: *mut Option<IWiaDrvItem>, __midl__iwiaminidrv0047: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).drvDeviceCommand)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0043, __midl__iwiaminidrv0044, __midl__iwiaminidrv0045, core::mem::transmute(__midl__iwiaminidrv0046), __midl__iwiaminidrv0047 as _).ok() }
    }
    pub unsafe fn drvGetCapabilities(&self, __midl__iwiaminidrv0048: *const u8, __midl__iwiaminidrv0049: i32, __midl__iwiaminidrv0050: *mut i32, __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV, __midl__iwiaminidrv0052: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).drvGetCapabilities)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0048, __midl__iwiaminidrv0049, __midl__iwiaminidrv0050 as _, __midl__iwiaminidrv0051 as _, __midl__iwiaminidrv0052 as _).ok() }
    }
    pub unsafe fn drvDeleteItem(&self, __midl__iwiaminidrv0053: *const u8, __midl__iwiaminidrv0054: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).drvDeleteItem)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0053, __midl__iwiaminidrv0054, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn drvFreeDrvItemContext(&self, __midl__iwiaminidrv0056: i32, __midl__iwiaminidrv0057: *const u8) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).drvFreeDrvItemContext)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0056, __midl__iwiaminidrv0057, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn drvGetWiaFormatInfo(&self, __midl__iwiaminidrv0059: *const u8, __midl__iwiaminidrv0060: i32, __midl__iwiaminidrv0061: *mut i32, __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO, __midl__iwiaminidrv0063: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).drvGetWiaFormatInfo)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0059, __midl__iwiaminidrv0060, __midl__iwiaminidrv0061 as _, __midl__iwiaminidrv0062 as _, __midl__iwiaminidrv0063 as _).ok() }
    }
    pub unsafe fn drvNotifyPnpEvent(&self, peventguid: *const windows_core::GUID, bstrdeviceid: &windows_core::BSTR, ulreserved: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).drvNotifyPnpEvent)(windows_core::Interface::as_raw(self), peventguid, core::mem::transmute_copy(bstrdeviceid), ulreserved).ok() }
    }
    pub unsafe fn drvUnInitializeWia(&self, __midl__iwiaminidrv0064: *const u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).drvUnInitializeWia)(windows_core::Interface::as_raw(self), __midl__iwiaminidrv0064).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaMiniDrv_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub drvInitializeWia: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub drvAcquireItemData: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, i32, *mut MINIDRV_TRANSFER_CONTEXT, *mut i32) -> windows_core::HRESULT,
    pub drvInitItemProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub drvValidateItemProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, i32, u32, *const super::super::System::Com::StructuredStorage::PROPSPEC, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    drvValidateItemProperties: usize,
    pub drvWriteItemProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, i32, *const MINIDRV_TRANSFER_CONTEXT, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub drvReadItemProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, i32, u32, *const super::super::System::Com::StructuredStorage::PROPSPEC, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    drvReadItemProperties: usize,
    pub drvLockWiaDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, i32, *mut i32) -> windows_core::HRESULT,
    pub drvUnLockWiaDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, i32, *mut i32) -> windows_core::HRESULT,
    pub drvAnalyzeItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, i32, *const i32) -> windows_core::HRESULT,
    pub drvGetDeviceErrorStr: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut windows_core::PWSTR, *mut i32) -> windows_core::HRESULT,
    pub drvDeviceCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, i32, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub drvGetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, i32, *mut i32, *mut *mut WIA_DEV_CAP_DRV, *mut i32) -> windows_core::HRESULT,
    pub drvDeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, i32, *mut i32) -> windows_core::HRESULT,
    pub drvFreeDrvItemContext: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const u8, *mut i32) -> windows_core::HRESULT,
    pub drvGetWiaFormatInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, i32, *mut i32, *mut *mut WIA_FORMAT_INFO, *mut i32) -> windows_core::HRESULT,
    pub drvNotifyPnpEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub drvUnInitializeWia: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWiaMiniDrv_Impl: windows_core::IUnknownImpl {
    fn drvInitializeWia(&self, __midl__iwiaminidrv0000: *const u8, __midl__iwiaminidrv0001: i32, __midl__iwiaminidrv0002: &windows_core::BSTR, __midl__iwiaminidrv0003: &windows_core::BSTR, __midl__iwiaminidrv0004: windows_core::Ref<windows_core::IUnknown>, __midl__iwiaminidrv0005: windows_core::Ref<windows_core::IUnknown>, __midl__iwiaminidrv0006: windows_core::OutRef<IWiaDrvItem>, __midl__iwiaminidrv0007: windows_core::OutRef<windows_core::IUnknown>, __midl__iwiaminidrv0008: *mut i32) -> windows_core::Result<()>;
    fn drvAcquireItemData(&self, __midl__iwiaminidrv0009: *const u8, __midl__iwiaminidrv0010: i32, __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0012: *mut i32) -> windows_core::Result<()>;
    fn drvInitItemProperties(&self, __midl__iwiaminidrv0013: *const u8, __midl__iwiaminidrv0014: i32) -> windows_core::Result<i32>;
    fn drvValidateItemProperties(&self, __midl__iwiaminidrv0016: *const u8, __midl__iwiaminidrv0017: i32, __midl__iwiaminidrv0018: u32, __midl__iwiaminidrv0019: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> windows_core::Result<i32>;
    fn drvWriteItemProperties(&self, __midl__iwiaminidrv0021: *const u8, __midl__iwiaminidrv0022: i32, __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT) -> windows_core::Result<i32>;
    fn drvReadItemProperties(&self, __midl__iwiaminidrv0025: *const u8, __midl__iwiaminidrv0026: i32, __midl__iwiaminidrv0027: u32, __midl__iwiaminidrv0028: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> windows_core::Result<i32>;
    fn drvLockWiaDevice(&self, __midl__iwiaminidrv0030: *const u8, __midl__iwiaminidrv0031: i32) -> windows_core::Result<i32>;
    fn drvUnLockWiaDevice(&self, __midl__iwiaminidrv0033: *const u8, __midl__iwiaminidrv0034: i32) -> windows_core::Result<i32>;
    fn drvAnalyzeItem(&self, __midl__iwiaminidrv0036: *const u8, __midl__iwiaminidrv0037: i32, __midl__iwiaminidrv0038: *const i32) -> windows_core::Result<()>;
    fn drvGetDeviceErrorStr(&self, __midl__iwiaminidrv0039: i32, __midl__iwiaminidrv0040: i32, __midl__iwiaminidrv0041: *mut windows_core::PWSTR, __midl__iwiaminidrv0042: *mut i32) -> windows_core::Result<()>;
    fn drvDeviceCommand(&self, __midl__iwiaminidrv0043: *const u8, __midl__iwiaminidrv0044: i32, __midl__iwiaminidrv0045: *const windows_core::GUID, __midl__iwiaminidrv0046: windows_core::OutRef<IWiaDrvItem>, __midl__iwiaminidrv0047: *mut i32) -> windows_core::Result<()>;
    fn drvGetCapabilities(&self, __midl__iwiaminidrv0048: *const u8, __midl__iwiaminidrv0049: i32, __midl__iwiaminidrv0050: *mut i32, __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV, __midl__iwiaminidrv0052: *mut i32) -> windows_core::Result<()>;
    fn drvDeleteItem(&self, __midl__iwiaminidrv0053: *const u8, __midl__iwiaminidrv0054: i32) -> windows_core::Result<i32>;
    fn drvFreeDrvItemContext(&self, __midl__iwiaminidrv0056: i32, __midl__iwiaminidrv0057: *const u8) -> windows_core::Result<i32>;
    fn drvGetWiaFormatInfo(&self, __midl__iwiaminidrv0059: *const u8, __midl__iwiaminidrv0060: i32, __midl__iwiaminidrv0061: *mut i32, __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO, __midl__iwiaminidrv0063: *mut i32) -> windows_core::Result<()>;
    fn drvNotifyPnpEvent(&self, peventguid: *const windows_core::GUID, bstrdeviceid: &windows_core::BSTR, ulreserved: u32) -> windows_core::Result<()>;
    fn drvUnInitializeWia(&self, __midl__iwiaminidrv0064: *const u8) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWiaMiniDrv_Vtbl {
    pub const fn new<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn drvInitializeWia<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0000: *const u8, __midl__iwiaminidrv0001: i32, __midl__iwiaminidrv0002: *mut core::ffi::c_void, __midl__iwiaminidrv0003: *mut core::ffi::c_void, __midl__iwiaminidrv0004: *mut core::ffi::c_void, __midl__iwiaminidrv0005: *mut core::ffi::c_void, __midl__iwiaminidrv0006: *mut *mut core::ffi::c_void, __midl__iwiaminidrv0007: *mut *mut core::ffi::c_void, __midl__iwiaminidrv0008: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaMiniDrv_Impl::drvInitializeWia(this, core::mem::transmute_copy(&__midl__iwiaminidrv0000), core::mem::transmute_copy(&__midl__iwiaminidrv0001), core::mem::transmute(&__midl__iwiaminidrv0002), core::mem::transmute(&__midl__iwiaminidrv0003), core::mem::transmute_copy(&__midl__iwiaminidrv0004), core::mem::transmute_copy(&__midl__iwiaminidrv0005), core::mem::transmute_copy(&__midl__iwiaminidrv0006), core::mem::transmute_copy(&__midl__iwiaminidrv0007), core::mem::transmute_copy(&__midl__iwiaminidrv0008)).into()
            }
        }
        unsafe extern "system" fn drvAcquireItemData<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0009: *const u8, __midl__iwiaminidrv0010: i32, __midl__iwiaminidrv0011: *mut MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0012: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaMiniDrv_Impl::drvAcquireItemData(this, core::mem::transmute_copy(&__midl__iwiaminidrv0009), core::mem::transmute_copy(&__midl__iwiaminidrv0010), core::mem::transmute_copy(&__midl__iwiaminidrv0011), core::mem::transmute_copy(&__midl__iwiaminidrv0012)).into()
            }
        }
        unsafe extern "system" fn drvInitItemProperties<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0013: *const u8, __midl__iwiaminidrv0014: i32, __midl__iwiaminidrv0015: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaMiniDrv_Impl::drvInitItemProperties(this, core::mem::transmute_copy(&__midl__iwiaminidrv0013), core::mem::transmute_copy(&__midl__iwiaminidrv0014)) {
                    Ok(ok__) => {
                        __midl__iwiaminidrv0015.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn drvValidateItemProperties<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0016: *const u8, __midl__iwiaminidrv0017: i32, __midl__iwiaminidrv0018: u32, __midl__iwiaminidrv0019: *const super::super::System::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0020: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaMiniDrv_Impl::drvValidateItemProperties(this, core::mem::transmute_copy(&__midl__iwiaminidrv0016), core::mem::transmute_copy(&__midl__iwiaminidrv0017), core::mem::transmute_copy(&__midl__iwiaminidrv0018), core::mem::transmute_copy(&__midl__iwiaminidrv0019)) {
                    Ok(ok__) => {
                        __midl__iwiaminidrv0020.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn drvWriteItemProperties<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0021: *const u8, __midl__iwiaminidrv0022: i32, __midl__iwiaminidrv0023: *const MINIDRV_TRANSFER_CONTEXT, __midl__iwiaminidrv0024: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaMiniDrv_Impl::drvWriteItemProperties(this, core::mem::transmute_copy(&__midl__iwiaminidrv0021), core::mem::transmute_copy(&__midl__iwiaminidrv0022), core::mem::transmute_copy(&__midl__iwiaminidrv0023)) {
                    Ok(ok__) => {
                        __midl__iwiaminidrv0024.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn drvReadItemProperties<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0025: *const u8, __midl__iwiaminidrv0026: i32, __midl__iwiaminidrv0027: u32, __midl__iwiaminidrv0028: *const super::super::System::Com::StructuredStorage::PROPSPEC, __midl__iwiaminidrv0029: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaMiniDrv_Impl::drvReadItemProperties(this, core::mem::transmute_copy(&__midl__iwiaminidrv0025), core::mem::transmute_copy(&__midl__iwiaminidrv0026), core::mem::transmute_copy(&__midl__iwiaminidrv0027), core::mem::transmute_copy(&__midl__iwiaminidrv0028)) {
                    Ok(ok__) => {
                        __midl__iwiaminidrv0029.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn drvLockWiaDevice<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0030: *const u8, __midl__iwiaminidrv0031: i32, __midl__iwiaminidrv0032: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaMiniDrv_Impl::drvLockWiaDevice(this, core::mem::transmute_copy(&__midl__iwiaminidrv0030), core::mem::transmute_copy(&__midl__iwiaminidrv0031)) {
                    Ok(ok__) => {
                        __midl__iwiaminidrv0032.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn drvUnLockWiaDevice<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0033: *const u8, __midl__iwiaminidrv0034: i32, __midl__iwiaminidrv0035: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaMiniDrv_Impl::drvUnLockWiaDevice(this, core::mem::transmute_copy(&__midl__iwiaminidrv0033), core::mem::transmute_copy(&__midl__iwiaminidrv0034)) {
                    Ok(ok__) => {
                        __midl__iwiaminidrv0035.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn drvAnalyzeItem<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0036: *const u8, __midl__iwiaminidrv0037: i32, __midl__iwiaminidrv0038: *const i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaMiniDrv_Impl::drvAnalyzeItem(this, core::mem::transmute_copy(&__midl__iwiaminidrv0036), core::mem::transmute_copy(&__midl__iwiaminidrv0037), core::mem::transmute_copy(&__midl__iwiaminidrv0038)).into()
            }
        }
        unsafe extern "system" fn drvGetDeviceErrorStr<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0039: i32, __midl__iwiaminidrv0040: i32, __midl__iwiaminidrv0041: *mut windows_core::PWSTR, __midl__iwiaminidrv0042: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaMiniDrv_Impl::drvGetDeviceErrorStr(this, core::mem::transmute_copy(&__midl__iwiaminidrv0039), core::mem::transmute_copy(&__midl__iwiaminidrv0040), core::mem::transmute_copy(&__midl__iwiaminidrv0041), core::mem::transmute_copy(&__midl__iwiaminidrv0042)).into()
            }
        }
        unsafe extern "system" fn drvDeviceCommand<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0043: *const u8, __midl__iwiaminidrv0044: i32, __midl__iwiaminidrv0045: *const windows_core::GUID, __midl__iwiaminidrv0046: *mut *mut core::ffi::c_void, __midl__iwiaminidrv0047: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaMiniDrv_Impl::drvDeviceCommand(this, core::mem::transmute_copy(&__midl__iwiaminidrv0043), core::mem::transmute_copy(&__midl__iwiaminidrv0044), core::mem::transmute_copy(&__midl__iwiaminidrv0045), core::mem::transmute_copy(&__midl__iwiaminidrv0046), core::mem::transmute_copy(&__midl__iwiaminidrv0047)).into()
            }
        }
        unsafe extern "system" fn drvGetCapabilities<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0048: *const u8, __midl__iwiaminidrv0049: i32, __midl__iwiaminidrv0050: *mut i32, __midl__iwiaminidrv0051: *mut *mut WIA_DEV_CAP_DRV, __midl__iwiaminidrv0052: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaMiniDrv_Impl::drvGetCapabilities(this, core::mem::transmute_copy(&__midl__iwiaminidrv0048), core::mem::transmute_copy(&__midl__iwiaminidrv0049), core::mem::transmute_copy(&__midl__iwiaminidrv0050), core::mem::transmute_copy(&__midl__iwiaminidrv0051), core::mem::transmute_copy(&__midl__iwiaminidrv0052)).into()
            }
        }
        unsafe extern "system" fn drvDeleteItem<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0053: *const u8, __midl__iwiaminidrv0054: i32, __midl__iwiaminidrv0055: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaMiniDrv_Impl::drvDeleteItem(this, core::mem::transmute_copy(&__midl__iwiaminidrv0053), core::mem::transmute_copy(&__midl__iwiaminidrv0054)) {
                    Ok(ok__) => {
                        __midl__iwiaminidrv0055.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn drvFreeDrvItemContext<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0056: i32, __midl__iwiaminidrv0057: *const u8, __midl__iwiaminidrv0058: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaMiniDrv_Impl::drvFreeDrvItemContext(this, core::mem::transmute_copy(&__midl__iwiaminidrv0056), core::mem::transmute_copy(&__midl__iwiaminidrv0057)) {
                    Ok(ok__) => {
                        __midl__iwiaminidrv0058.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn drvGetWiaFormatInfo<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0059: *const u8, __midl__iwiaminidrv0060: i32, __midl__iwiaminidrv0061: *mut i32, __midl__iwiaminidrv0062: *mut *mut WIA_FORMAT_INFO, __midl__iwiaminidrv0063: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaMiniDrv_Impl::drvGetWiaFormatInfo(this, core::mem::transmute_copy(&__midl__iwiaminidrv0059), core::mem::transmute_copy(&__midl__iwiaminidrv0060), core::mem::transmute_copy(&__midl__iwiaminidrv0061), core::mem::transmute_copy(&__midl__iwiaminidrv0062), core::mem::transmute_copy(&__midl__iwiaminidrv0063)).into()
            }
        }
        unsafe extern "system" fn drvNotifyPnpEvent<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventguid: *const windows_core::GUID, bstrdeviceid: *mut core::ffi::c_void, ulreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaMiniDrv_Impl::drvNotifyPnpEvent(this, core::mem::transmute_copy(&peventguid), core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&ulreserved)).into()
            }
        }
        unsafe extern "system" fn drvUnInitializeWia<Identity: IWiaMiniDrv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, __midl__iwiaminidrv0064: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaMiniDrv_Impl::drvUnInitializeWia(this, core::mem::transmute_copy(&__midl__iwiaminidrv0064)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            drvInitializeWia: drvInitializeWia::<Identity, OFFSET>,
            drvAcquireItemData: drvAcquireItemData::<Identity, OFFSET>,
            drvInitItemProperties: drvInitItemProperties::<Identity, OFFSET>,
            drvValidateItemProperties: drvValidateItemProperties::<Identity, OFFSET>,
            drvWriteItemProperties: drvWriteItemProperties::<Identity, OFFSET>,
            drvReadItemProperties: drvReadItemProperties::<Identity, OFFSET>,
            drvLockWiaDevice: drvLockWiaDevice::<Identity, OFFSET>,
            drvUnLockWiaDevice: drvUnLockWiaDevice::<Identity, OFFSET>,
            drvAnalyzeItem: drvAnalyzeItem::<Identity, OFFSET>,
            drvGetDeviceErrorStr: drvGetDeviceErrorStr::<Identity, OFFSET>,
            drvDeviceCommand: drvDeviceCommand::<Identity, OFFSET>,
            drvGetCapabilities: drvGetCapabilities::<Identity, OFFSET>,
            drvDeleteItem: drvDeleteItem::<Identity, OFFSET>,
            drvFreeDrvItemContext: drvFreeDrvItemContext::<Identity, OFFSET>,
            drvGetWiaFormatInfo: drvGetWiaFormatInfo::<Identity, OFFSET>,
            drvNotifyPnpEvent: drvNotifyPnpEvent::<Identity, OFFSET>,
            drvUnInitializeWia: drvUnInitializeWia::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaMiniDrv as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IWiaMiniDrv {}
windows_core::imp::define_interface!(IWiaMiniDrvCallBack, IWiaMiniDrvCallBack_Vtbl, 0x33a57d5a_3de8_11d2_9a36_00c04fa36145);
windows_core::imp::interface_hierarchy!(IWiaMiniDrvCallBack, windows_core::IUnknown);
impl IWiaMiniDrvCallBack {
    pub unsafe fn MiniDrvCallback(&self, lreason: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, ptranctx: *const MINIDRV_TRANSFER_CONTEXT, lreserved: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MiniDrvCallback)(windows_core::Interface::as_raw(self), lreason, lstatus, lpercentcomplete, loffset, llength, core::mem::transmute(ptranctx), lreserved).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaMiniDrvCallBack_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MiniDrvCallback: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32, i32, i32, *const MINIDRV_TRANSFER_CONTEXT, i32) -> windows_core::HRESULT,
}
pub trait IWiaMiniDrvCallBack_Impl: windows_core::IUnknownImpl {
    fn MiniDrvCallback(&self, lreason: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, ptranctx: *const MINIDRV_TRANSFER_CONTEXT, lreserved: i32) -> windows_core::Result<()>;
}
impl IWiaMiniDrvCallBack_Vtbl {
    pub const fn new<Identity: IWiaMiniDrvCallBack_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MiniDrvCallback<Identity: IWiaMiniDrvCallBack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lreason: i32, lstatus: i32, lpercentcomplete: i32, loffset: i32, llength: i32, ptranctx: *const MINIDRV_TRANSFER_CONTEXT, lreserved: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaMiniDrvCallBack_Impl::MiniDrvCallback(this, core::mem::transmute_copy(&lreason), core::mem::transmute_copy(&lstatus), core::mem::transmute_copy(&lpercentcomplete), core::mem::transmute_copy(&loffset), core::mem::transmute_copy(&llength), core::mem::transmute_copy(&ptranctx), core::mem::transmute_copy(&lreserved)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), MiniDrvCallback: MiniDrvCallback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaMiniDrvCallBack as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaMiniDrvCallBack {}
windows_core::imp::define_interface!(IWiaMiniDrvTransferCallback, IWiaMiniDrvTransferCallback_Vtbl, 0xa9d2ee89_2ce5_4ff0_8adb_c961d1d774ca);
windows_core::imp::interface_hierarchy!(IWiaMiniDrvTransferCallback, windows_core::IUnknown);
impl IWiaMiniDrvTransferCallback {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNextStream(&self, lflags: i32, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextStream)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstritemname), core::mem::transmute_copy(bstrfullitemname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SendMessage(&self, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendMessage)(windows_core::Interface::as_raw(self), lflags, pwiatransferparams).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaMiniDrvTransferCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNextStream: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNextStream: usize,
    pub SendMessage: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const WiaTransferParams) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaMiniDrvTransferCallback_Impl: windows_core::IUnknownImpl {
    fn GetNextStream(&self, lflags: i32, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SendMessage(&self, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWiaMiniDrvTransferCallback_Vtbl {
    pub const fn new<Identity: IWiaMiniDrvTransferCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNextStream<Identity: IWiaMiniDrvTransferCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstritemname: *mut core::ffi::c_void, bstrfullitemname: *mut core::ffi::c_void, ppistream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaMiniDrvTransferCallback_Impl::GetNextStream(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstritemname), core::mem::transmute(&bstrfullitemname)) {
                    Ok(ok__) => {
                        ppistream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendMessage<Identity: IWiaMiniDrvTransferCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaMiniDrvTransferCallback_Impl::SendMessage(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiatransferparams)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNextStream: GetNextStream::<Identity, OFFSET>,
            SendMessage: SendMessage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaMiniDrvTransferCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWiaMiniDrvTransferCallback {}
windows_core::imp::define_interface!(IWiaNotifyDevMgr, IWiaNotifyDevMgr_Vtbl, 0x70681ea0_e7bf_4291_9fb1_4e8813a3f78e);
windows_core::imp::interface_hierarchy!(IWiaNotifyDevMgr, windows_core::IUnknown);
impl IWiaNotifyDevMgr {
    pub unsafe fn NewDeviceArrival(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NewDeviceArrival)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaNotifyDevMgr_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NewDeviceArrival: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWiaNotifyDevMgr_Impl: windows_core::IUnknownImpl {
    fn NewDeviceArrival(&self) -> windows_core::Result<()>;
}
impl IWiaNotifyDevMgr_Vtbl {
    pub const fn new<Identity: IWiaNotifyDevMgr_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NewDeviceArrival<Identity: IWiaNotifyDevMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaNotifyDevMgr_Impl::NewDeviceArrival(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NewDeviceArrival: NewDeviceArrival::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaNotifyDevMgr as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaNotifyDevMgr {}
windows_core::imp::define_interface!(IWiaPreview, IWiaPreview_Vtbl, 0x95c2b4fd_33f2_4d86_ad40_9431f0df08f7);
windows_core::imp::interface_hierarchy!(IWiaPreview, windows_core::IUnknown);
impl IWiaPreview {
    pub unsafe fn GetNewPreview<P1, P2>(&self, lflags: i32, pwiaitem2: P1, pwiatransfercallback: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWiaItem2>,
        P2: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetNewPreview)(windows_core::Interface::as_raw(self), lflags, pwiaitem2.param().abi(), pwiatransfercallback.param().abi()).ok() }
    }
    pub unsafe fn UpdatePreview<P1, P2>(&self, lflags: i32, pchildwiaitem2: P1, pwiatransfercallback: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWiaItem2>,
        P2: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdatePreview)(windows_core::Interface::as_raw(self), lflags, pchildwiaitem2.param().abi(), pwiatransfercallback.param().abi()).ok() }
    }
    pub unsafe fn DetectRegions(&self, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DetectRegions)(windows_core::Interface::as_raw(self), lflags).ok() }
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaPreview_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNewPreview: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdatePreview: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DetectRegions: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWiaPreview_Impl: windows_core::IUnknownImpl {
    fn GetNewPreview(&self, lflags: i32, pwiaitem2: windows_core::Ref<IWiaItem2>, pwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn UpdatePreview(&self, lflags: i32, pchildwiaitem2: windows_core::Ref<IWiaItem2>, pwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn DetectRegions(&self, lflags: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl IWiaPreview_Vtbl {
    pub const fn new<Identity: IWiaPreview_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNewPreview<Identity: IWiaPreview_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiaitem2: *mut core::ffi::c_void, pwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPreview_Impl::GetNewPreview(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiaitem2), core::mem::transmute_copy(&pwiatransfercallback)).into()
            }
        }
        unsafe extern "system" fn UpdatePreview<Identity: IWiaPreview_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pchildwiaitem2: *mut core::ffi::c_void, pwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPreview_Impl::UpdatePreview(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pchildwiaitem2), core::mem::transmute_copy(&pwiatransfercallback)).into()
            }
        }
        unsafe extern "system" fn DetectRegions<Identity: IWiaPreview_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPreview_Impl::DetectRegions(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IWiaPreview_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPreview_Impl::Clear(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNewPreview: GetNewPreview::<Identity, OFFSET>,
            UpdatePreview: UpdatePreview::<Identity, OFFSET>,
            DetectRegions: DetectRegions::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaPreview as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaPreview {}
windows_core::imp::define_interface!(IWiaPropertyStorage, IWiaPropertyStorage_Vtbl, 0x98b5e8a0_29cc_491a_aac0_e6db4fdcceb6);
windows_core::imp::interface_hierarchy!(IWiaPropertyStorage, windows_core::IUnknown);
impl IWiaPropertyStorage {
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn ReadMultiple(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadMultiple)(windows_core::Interface::as_raw(self), cpspec, rgpspec, core::mem::transmute(rgpropvar)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn WriteMultiple(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteMultiple)(windows_core::Interface::as_raw(self), cpspec, rgpspec, core::mem::transmute(rgpropvar), propidnamefirst).ok() }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn DeleteMultiple(&self, rgpspec: &[super::super::System::Com::StructuredStorage::PROPSPEC]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteMultiple)(windows_core::Interface::as_raw(self), rgpspec.len().try_into().unwrap(), core::mem::transmute(rgpspec.as_ptr())).ok() }
    }
    pub unsafe fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadPropertyNames)(windows_core::Interface::as_raw(self), cpropid, rgpropid, rglpwstrname as _).ok() }
    }
    pub unsafe fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WritePropertyNames)(windows_core::Interface::as_raw(self), cpropid, rgpropid, rglpwstrname).ok() }
    }
    pub unsafe fn DeletePropertyNames(&self, rgpropid: &[u32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeletePropertyNames)(windows_core::Interface::as_raw(self), rgpropid.len().try_into().unwrap(), core::mem::transmute(rgpropid.as_ptr())).ok() }
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), grfcommitflags).ok() }
    }
    pub unsafe fn Revert(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Enum(&self) -> windows_core::Result<super::super::System::Com::StructuredStorage::IEnumSTATPROPSTG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetTimes(&self, pctime: *const super::super::Foundation::FILETIME, patime: *const super::super::Foundation::FILETIME, pmtime: *const super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTimes)(windows_core::Interface::as_raw(self), pctime, patime, pmtime).ok() }
    }
    pub unsafe fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClass)(windows_core::Interface::as_raw(self), clsid).ok() }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Stat(&self, pstatpsstg: *mut super::super::System::Com::StructuredStorage::STATPROPSETSTG) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stat)(windows_core::Interface::as_raw(self), pstatpsstg as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetPropertyAttributes(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyAttributes)(windows_core::Interface::as_raw(self), cpspec, rgpspec, rgflags as _, core::mem::transmute(rgpropvar)).ok() }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyStream(&self, pcompatibilityid: *mut windows_core::GUID, ppistream: *mut Option<super::super::System::Com::IStream>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyStream)(windows_core::Interface::as_raw(self), pcompatibilityid as _, core::mem::transmute(ppistream)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPropertyStream<P1>(&self, pcompatibilityid: *mut windows_core::GUID, pistream: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPropertyStream)(windows_core::Interface::as_raw(self), pcompatibilityid as _, pistream.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaPropertyStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub ReadMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::System::Com::StructuredStorage::PROPSPEC, *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    ReadMultiple: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub WriteMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::System::Com::StructuredStorage::PROPSPEC, *const super::super::System::Com::StructuredStorage::PROPVARIANT, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    WriteMultiple: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub DeleteMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::System::Com::StructuredStorage::PROPSPEC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    DeleteMultiple: usize,
    pub ReadPropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub WritePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub DeletePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Enum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Enum: usize,
    pub SetTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::FILETIME, *const super::super::Foundation::FILETIME, *const super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub SetClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Stat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Com::StructuredStorage::STATPROPSETSTG) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Stat: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetPropertyAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::System::Com::StructuredStorage::PROPSPEC, *mut u32, *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetPropertyAttributes: usize,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPropertyStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPropertyStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPropertyStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPropertyStream: usize,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IWiaPropertyStorage_Impl: windows_core::IUnknownImpl {
    fn ReadMultiple(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn WriteMultiple(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> windows_core::Result<()>;
    fn DeleteMultiple(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> windows_core::Result<()>;
    fn ReadPropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn WritePropertyNames(&self, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeletePropertyNames(&self, cpropid: u32, rgpropid: *const u32) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn Enum(&self) -> windows_core::Result<super::super::System::Com::StructuredStorage::IEnumSTATPROPSTG>;
    fn SetTimes(&self, pctime: *const super::super::Foundation::FILETIME, patime: *const super::super::Foundation::FILETIME, pmtime: *const super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Stat(&self, pstatpsstg: *mut super::super::System::Com::StructuredStorage::STATPROPSETSTG) -> windows_core::Result<()>;
    fn GetPropertyAttributes(&self, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetPropertyStream(&self, pcompatibilityid: *mut windows_core::GUID, ppistream: windows_core::OutRef<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn SetPropertyStream(&self, pcompatibilityid: *mut windows_core::GUID, pistream: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IWiaPropertyStorage_Vtbl {
    pub const fn new<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadMultiple<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::ReadMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgpropvar)).into()
            }
        }
        unsafe extern "system" fn WriteMultiple<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgpropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT, propidnamefirst: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::WriteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgpropvar), core::mem::transmute_copy(&propidnamefirst)).into()
            }
        }
        unsafe extern "system" fn DeleteMultiple<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::DeleteMultiple(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec)).into()
            }
        }
        unsafe extern "system" fn ReadPropertyNames<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::ReadPropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid), core::mem::transmute_copy(&rglpwstrname)).into()
            }
        }
        unsafe extern "system" fn WritePropertyNames<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::WritePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid), core::mem::transmute_copy(&rglpwstrname)).into()
            }
        }
        unsafe extern "system" fn DeletePropertyNames<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::DeletePropertyNames(this, core::mem::transmute_copy(&cpropid), core::mem::transmute_copy(&rgpropid)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfcommitflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::Commit(this, core::mem::transmute_copy(&grfcommitflags)).into()
            }
        }
        unsafe extern "system" fn Revert<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::Revert(this).into()
            }
        }
        unsafe extern "system" fn Enum<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaPropertyStorage_Impl::Enum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTimes<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctime: *const super::super::Foundation::FILETIME, patime: *const super::super::Foundation::FILETIME, pmtime: *const super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::SetTimes(this, core::mem::transmute_copy(&pctime), core::mem::transmute_copy(&patime), core::mem::transmute_copy(&pmtime)).into()
            }
        }
        unsafe extern "system" fn SetClass<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::SetClass(this, core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn Stat<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatpsstg: *mut super::super::System::Com::StructuredStorage::STATPROPSETSTG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::Stat(this, core::mem::transmute_copy(&pstatpsstg)).into()
            }
        }
        unsafe extern "system" fn GetPropertyAttributes<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpspec: u32, rgpspec: *const super::super::System::Com::StructuredStorage::PROPSPEC, rgflags: *mut u32, rgpropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::GetPropertyAttributes(this, core::mem::transmute_copy(&cpspec), core::mem::transmute_copy(&rgpspec), core::mem::transmute_copy(&rgflags), core::mem::transmute_copy(&rgpropvar)).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulnumprops: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaPropertyStorage_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pulnumprops.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyStream<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcompatibilityid: *mut windows_core::GUID, ppistream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::GetPropertyStream(this, core::mem::transmute_copy(&pcompatibilityid), core::mem::transmute_copy(&ppistream)).into()
            }
        }
        unsafe extern "system" fn SetPropertyStream<Identity: IWiaPropertyStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcompatibilityid: *mut windows_core::GUID, pistream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaPropertyStorage_Impl::SetPropertyStream(this, core::mem::transmute_copy(&pcompatibilityid), core::mem::transmute_copy(&pistream)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadMultiple: ReadMultiple::<Identity, OFFSET>,
            WriteMultiple: WriteMultiple::<Identity, OFFSET>,
            DeleteMultiple: DeleteMultiple::<Identity, OFFSET>,
            ReadPropertyNames: ReadPropertyNames::<Identity, OFFSET>,
            WritePropertyNames: WritePropertyNames::<Identity, OFFSET>,
            DeletePropertyNames: DeletePropertyNames::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Revert: Revert::<Identity, OFFSET>,
            Enum: Enum::<Identity, OFFSET>,
            SetTimes: SetTimes::<Identity, OFFSET>,
            SetClass: SetClass::<Identity, OFFSET>,
            Stat: Stat::<Identity, OFFSET>,
            GetPropertyAttributes: GetPropertyAttributes::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetPropertyStream: GetPropertyStream::<Identity, OFFSET>,
            SetPropertyStream: SetPropertyStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaPropertyStorage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWiaPropertyStorage {}
windows_core::imp::define_interface!(IWiaSegmentationFilter, IWiaSegmentationFilter_Vtbl, 0xec46a697_ac04_4447_8f65_ff63d5154b21);
windows_core::imp::interface_hierarchy!(IWiaSegmentationFilter, windows_core::IUnknown);
impl IWiaSegmentationFilter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DetectRegions<P1, P2>(&self, lflags: i32, pinputstream: P1, pwiaitem2: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Com::IStream>,
        P2: windows_core::Param<IWiaItem2>,
    {
        unsafe { (windows_core::Interface::vtable(self).DetectRegions)(windows_core::Interface::as_raw(self), lflags, pinputstream.param().abi(), pwiaitem2.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaSegmentationFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DetectRegions: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DetectRegions: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaSegmentationFilter_Impl: windows_core::IUnknownImpl {
    fn DetectRegions(&self, lflags: i32, pinputstream: windows_core::Ref<super::super::System::Com::IStream>, pwiaitem2: windows_core::Ref<IWiaItem2>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWiaSegmentationFilter_Vtbl {
    pub const fn new<Identity: IWiaSegmentationFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DetectRegions<Identity: IWiaSegmentationFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pinputstream: *mut core::ffi::c_void, pwiaitem2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaSegmentationFilter_Impl::DetectRegions(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pinputstream), core::mem::transmute_copy(&pwiaitem2)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DetectRegions: DetectRegions::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaSegmentationFilter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWiaSegmentationFilter {}
windows_core::imp::define_interface!(IWiaTransfer, IWiaTransfer_Vtbl, 0xc39d6942_2f4e_4d04_92fe_4ef4d3a1de5a);
windows_core::imp::interface_hierarchy!(IWiaTransfer, windows_core::IUnknown);
impl IWiaTransfer {
    pub unsafe fn Download<P1>(&self, lflags: i32, piwiatransfercallback: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Download)(windows_core::Interface::as_raw(self), lflags, piwiatransfercallback.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Upload<P1, P2>(&self, lflags: i32, psource: P1, piwiatransfercallback: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Com::IStream>,
        P2: windows_core::Param<IWiaTransferCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Upload)(windows_core::Interface::as_raw(self), lflags, psource.param().abi(), piwiatransfercallback.param().abi()).ok() }
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EnumWIA_FORMAT_INFO(&self) -> windows_core::Result<IEnumWIA_FORMAT_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumWIA_FORMAT_INFO)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaTransfer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Download: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Upload: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Upload: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumWIA_FORMAT_INFO: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaTransfer_Impl: windows_core::IUnknownImpl {
    fn Download(&self, lflags: i32, piwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn Upload(&self, lflags: i32, psource: windows_core::Ref<super::super::System::Com::IStream>, piwiatransfercallback: windows_core::Ref<IWiaTransferCallback>) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn EnumWIA_FORMAT_INFO(&self) -> windows_core::Result<IEnumWIA_FORMAT_INFO>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWiaTransfer_Vtbl {
    pub const fn new<Identity: IWiaTransfer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Download<Identity: IWiaTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, piwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaTransfer_Impl::Download(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&piwiatransfercallback)).into()
            }
        }
        unsafe extern "system" fn Upload<Identity: IWiaTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, psource: *mut core::ffi::c_void, piwiatransfercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaTransfer_Impl::Upload(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&psource), core::mem::transmute_copy(&piwiatransfercallback)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IWiaTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaTransfer_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn EnumWIA_FORMAT_INFO<Identity: IWiaTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaTransfer_Impl::EnumWIA_FORMAT_INFO(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Download: Download::<Identity, OFFSET>,
            Upload: Upload::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            EnumWIA_FORMAT_INFO: EnumWIA_FORMAT_INFO::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaTransfer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWiaTransfer {}
windows_core::imp::define_interface!(IWiaTransferCallback, IWiaTransferCallback_Vtbl, 0x27d4eaaf_28a6_4ca5_9aab_e678168b9527);
windows_core::imp::interface_hierarchy!(IWiaTransferCallback, windows_core::IUnknown);
impl IWiaTransferCallback {
    pub unsafe fn TransferCallback(&self, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).TransferCallback)(windows_core::Interface::as_raw(self), lflags, pwiatransferparams).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNextStream(&self, lflags: i32, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextStream)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(bstritemname), core::mem::transmute_copy(bstrfullitemname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaTransferCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TransferCallback: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const WiaTransferParams) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNextStream: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNextStream: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWiaTransferCallback_Impl: windows_core::IUnknownImpl {
    fn TransferCallback(&self, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> windows_core::Result<()>;
    fn GetNextStream(&self, lflags: i32, bstritemname: &windows_core::BSTR, bstrfullitemname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWiaTransferCallback_Vtbl {
    pub const fn new<Identity: IWiaTransferCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TransferCallback<Identity: IWiaTransferCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pwiatransferparams: *const WiaTransferParams) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaTransferCallback_Impl::TransferCallback(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pwiatransferparams)).into()
            }
        }
        unsafe extern "system" fn GetNextStream<Identity: IWiaTransferCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, bstritemname: *mut core::ffi::c_void, bstrfullitemname: *mut core::ffi::c_void, ppdestination: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaTransferCallback_Impl::GetNextStream(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&bstritemname), core::mem::transmute(&bstrfullitemname)) {
                    Ok(ok__) => {
                        ppdestination.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TransferCallback: TransferCallback::<Identity, OFFSET>,
            GetNextStream: GetNextStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaTransferCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWiaTransferCallback {}
windows_core::imp::define_interface!(IWiaUIExtension, IWiaUIExtension_Vtbl, 0xda319113_50ee_4c80_b460_57d005d44a2c);
windows_core::imp::interface_hierarchy!(IWiaUIExtension, windows_core::IUnknown);
impl IWiaUIExtension {
    pub unsafe fn DeviceDialog(&self, pdevicedialogdata: *const DEVICEDIALOGDATA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeviceDialog)(windows_core::Interface::as_raw(self), core::mem::transmute(pdevicedialogdata)).ok() }
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetDeviceIcon(&self, bstrdeviceid: &windows_core::BSTR, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceIcon)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdeviceid), phicon as _, nsize).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetDeviceBitmapLogo(&self, bstrdeviceid: &windows_core::BSTR, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, nmaxwidth: u32, nmaxheight: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceBitmapLogo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdeviceid), phbitmap as _, nmaxwidth, nmaxheight).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaUIExtension_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DeviceDialog: unsafe extern "system" fn(*mut core::ffi::c_void, *const DEVICEDIALOGDATA) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetDeviceIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::UI::WindowsAndMessaging::HICON, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetDeviceIcon: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetDeviceBitmapLogo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Graphics::Gdi::HBITMAP, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetDeviceBitmapLogo: usize,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWiaUIExtension_Impl: windows_core::IUnknownImpl {
    fn DeviceDialog(&self, pdevicedialogdata: *const DEVICEDIALOGDATA) -> windows_core::Result<()>;
    fn GetDeviceIcon(&self, bstrdeviceid: &windows_core::BSTR, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> windows_core::Result<()>;
    fn GetDeviceBitmapLogo(&self, bstrdeviceid: &windows_core::BSTR, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, nmaxwidth: u32, nmaxheight: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWiaUIExtension_Vtbl {
    pub const fn new<Identity: IWiaUIExtension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeviceDialog<Identity: IWiaUIExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaUIExtension_Impl::DeviceDialog(this, core::mem::transmute_copy(&pdevicedialogdata)).into()
            }
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: IWiaUIExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceid: *mut core::ffi::c_void, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaUIExtension_Impl::GetDeviceIcon(this, core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&phicon), core::mem::transmute_copy(&nsize)).into()
            }
        }
        unsafe extern "system" fn GetDeviceBitmapLogo<Identity: IWiaUIExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceid: *mut core::ffi::c_void, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, nmaxwidth: u32, nmaxheight: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaUIExtension_Impl::GetDeviceBitmapLogo(this, core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&phbitmap), core::mem::transmute_copy(&nmaxwidth), core::mem::transmute_copy(&nmaxheight)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeviceDialog: DeviceDialog::<Identity, OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Identity, OFFSET>,
            GetDeviceBitmapLogo: GetDeviceBitmapLogo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaUIExtension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IWiaUIExtension {}
windows_core::imp::define_interface!(IWiaUIExtension2, IWiaUIExtension2_Vtbl, 0x305600d7_5088_46d7_9a15_b77b09cdba7a);
windows_core::imp::interface_hierarchy!(IWiaUIExtension2, windows_core::IUnknown);
impl IWiaUIExtension2 {
    pub unsafe fn DeviceDialog(&self, pdevicedialogdata: *const DEVICEDIALOGDATA2) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeviceDialog)(windows_core::Interface::as_raw(self), core::mem::transmute(pdevicedialogdata)).ok() }
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetDeviceIcon(&self, bstrdeviceid: &windows_core::BSTR, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceIcon)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdeviceid), phicon as _, nsize).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaUIExtension2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DeviceDialog: unsafe extern "system" fn(*mut core::ffi::c_void, *const DEVICEDIALOGDATA2) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetDeviceIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::UI::WindowsAndMessaging::HICON, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetDeviceIcon: usize,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IWiaUIExtension2_Impl: windows_core::IUnknownImpl {
    fn DeviceDialog(&self, pdevicedialogdata: *const DEVICEDIALOGDATA2) -> windows_core::Result<()>;
    fn GetDeviceIcon(&self, bstrdeviceid: &windows_core::BSTR, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IWiaUIExtension2_Vtbl {
    pub const fn new<Identity: IWiaUIExtension2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeviceDialog<Identity: IWiaUIExtension2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevicedialogdata: *const DEVICEDIALOGDATA2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaUIExtension2_Impl::DeviceDialog(this, core::mem::transmute_copy(&pdevicedialogdata)).into()
            }
        }
        unsafe extern "system" fn GetDeviceIcon<Identity: IWiaUIExtension2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdeviceid: *mut core::ffi::c_void, phicon: *mut super::super::UI::WindowsAndMessaging::HICON, nsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaUIExtension2_Impl::GetDeviceIcon(this, core::mem::transmute(&bstrdeviceid), core::mem::transmute_copy(&phicon), core::mem::transmute_copy(&nsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeviceDialog: DeviceDialog::<Identity, OFFSET>,
            GetDeviceIcon: GetDeviceIcon::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaUIExtension2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IWiaUIExtension2 {}
windows_core::imp::define_interface!(IWiaVideo, IWiaVideo_Vtbl, 0xd52920aa_db88_41f0_946c_e00dc0a19cfa);
windows_core::imp::interface_hierarchy!(IWiaVideo, windows_core::IUnknown);
impl IWiaVideo {
    pub unsafe fn PreviewVisible(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreviewVisible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPreviewVisible(&self, bpreviewvisible: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPreviewVisible)(windows_core::Interface::as_raw(self), bpreviewvisible.into()).ok() }
    }
    pub unsafe fn ImagesDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImagesDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetImagesDirectory(&self, bstrimagedirectory: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetImagesDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrimagedirectory)).ok() }
    }
    pub unsafe fn CreateVideoByWiaDevID(&self, bstrwiadeviceid: &windows_core::BSTR, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: bool, bautobeginplayback: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateVideoByWiaDevID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrwiadeviceid), hwndparent, bstretchtofitparent.into(), bautobeginplayback.into()).ok() }
    }
    pub unsafe fn CreateVideoByDevNum(&self, uidevicenumber: u32, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: bool, bautobeginplayback: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateVideoByDevNum)(windows_core::Interface::as_raw(self), uidevicenumber, hwndparent, bstretchtofitparent.into(), bautobeginplayback.into()).ok() }
    }
    pub unsafe fn CreateVideoByName(&self, bstrfriendlyname: &windows_core::BSTR, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: bool, bautobeginplayback: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateVideoByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfriendlyname), hwndparent, bstretchtofitparent.into(), bautobeginplayback.into()).ok() }
    }
    pub unsafe fn DestroyVideo(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DestroyVideo)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Play(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Play)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn TakePicture(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TakePicture)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ResizeVideo(&self, bstretchtofitparent: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResizeVideo)(windows_core::Interface::as_raw(self), bstretchtofitparent.into()).ok() }
    }
    pub unsafe fn GetCurrentState(&self) -> windows_core::Result<WIAVIDEO_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiaVideo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PreviewVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetPreviewVisible: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub ImagesDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetImagesDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateVideoByWiaDevID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub CreateVideoByDevNum: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::HWND, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub CreateVideoByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub DestroyVideo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Play: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TakePicture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResizeVideo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCurrentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WIAVIDEO_STATE) -> windows_core::HRESULT,
}
pub trait IWiaVideo_Impl: windows_core::IUnknownImpl {
    fn PreviewVisible(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetPreviewVisible(&self, bpreviewvisible: windows_core::BOOL) -> windows_core::Result<()>;
    fn ImagesDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetImagesDirectory(&self, bstrimagedirectory: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CreateVideoByWiaDevID(&self, bstrwiadeviceid: &windows_core::BSTR, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: windows_core::BOOL, bautobeginplayback: windows_core::BOOL) -> windows_core::Result<()>;
    fn CreateVideoByDevNum(&self, uidevicenumber: u32, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: windows_core::BOOL, bautobeginplayback: windows_core::BOOL) -> windows_core::Result<()>;
    fn CreateVideoByName(&self, bstrfriendlyname: &windows_core::BSTR, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: windows_core::BOOL, bautobeginplayback: windows_core::BOOL) -> windows_core::Result<()>;
    fn DestroyVideo(&self) -> windows_core::Result<()>;
    fn Play(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn TakePicture(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ResizeVideo(&self, bstretchtofitparent: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetCurrentState(&self) -> windows_core::Result<WIAVIDEO_STATE>;
}
impl IWiaVideo_Vtbl {
    pub const fn new<Identity: IWiaVideo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PreviewVisible<Identity: IWiaVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpreviewvisible: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaVideo_Impl::PreviewVisible(this) {
                    Ok(ok__) => {
                        pbpreviewvisible.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPreviewVisible<Identity: IWiaVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bpreviewvisible: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaVideo_Impl::SetPreviewVisible(this, core::mem::transmute_copy(&bpreviewvisible)).into()
            }
        }
        unsafe extern "system" fn ImagesDirectory<Identity: IWiaVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrimagedirectory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaVideo_Impl::ImagesDirectory(this) {
                    Ok(ok__) => {
                        pbstrimagedirectory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetImagesDirectory<Identity: IWiaVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrimagedirectory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaVideo_Impl::SetImagesDirectory(this, core::mem::transmute(&bstrimagedirectory)).into()
            }
        }
        unsafe extern "system" fn CreateVideoByWiaDevID<Identity: IWiaVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrwiadeviceid: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: windows_core::BOOL, bautobeginplayback: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaVideo_Impl::CreateVideoByWiaDevID(this, core::mem::transmute(&bstrwiadeviceid), core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&bstretchtofitparent), core::mem::transmute_copy(&bautobeginplayback)).into()
            }
        }
        unsafe extern "system" fn CreateVideoByDevNum<Identity: IWiaVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uidevicenumber: u32, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: windows_core::BOOL, bautobeginplayback: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaVideo_Impl::CreateVideoByDevNum(this, core::mem::transmute_copy(&uidevicenumber), core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&bstretchtofitparent), core::mem::transmute_copy(&bautobeginplayback)).into()
            }
        }
        unsafe extern "system" fn CreateVideoByName<Identity: IWiaVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfriendlyname: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, bstretchtofitparent: windows_core::BOOL, bautobeginplayback: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaVideo_Impl::CreateVideoByName(this, core::mem::transmute(&bstrfriendlyname), core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&bstretchtofitparent), core::mem::transmute_copy(&bautobeginplayback)).into()
            }
        }
        unsafe extern "system" fn DestroyVideo<Identity: IWiaVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaVideo_Impl::DestroyVideo(this).into()
            }
        }
        unsafe extern "system" fn Play<Identity: IWiaVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaVideo_Impl::Play(this).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IWiaVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaVideo_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn TakePicture<Identity: IWiaVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnewimagefilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaVideo_Impl::TakePicture(this) {
                    Ok(ok__) => {
                        pbstrnewimagefilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResizeVideo<Identity: IWiaVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstretchtofitparent: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWiaVideo_Impl::ResizeVideo(this, core::mem::transmute_copy(&bstretchtofitparent)).into()
            }
        }
        unsafe extern "system" fn GetCurrentState<Identity: IWiaVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut WIAVIDEO_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWiaVideo_Impl::GetCurrentState(this) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PreviewVisible: PreviewVisible::<Identity, OFFSET>,
            SetPreviewVisible: SetPreviewVisible::<Identity, OFFSET>,
            ImagesDirectory: ImagesDirectory::<Identity, OFFSET>,
            SetImagesDirectory: SetImagesDirectory::<Identity, OFFSET>,
            CreateVideoByWiaDevID: CreateVideoByWiaDevID::<Identity, OFFSET>,
            CreateVideoByDevNum: CreateVideoByDevNum::<Identity, OFFSET>,
            CreateVideoByName: CreateVideoByName::<Identity, OFFSET>,
            DestroyVideo: DestroyVideo::<Identity, OFFSET>,
            Play: Play::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            TakePicture: TakePicture::<Identity, OFFSET>,
            ResizeVideo: ResizeVideo::<Identity, OFFSET>,
            GetCurrentState: GetCurrentState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWiaVideo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWiaVideo {}
pub const LAMP_ERR: u32 = 2048u32;
pub const LANDSCAPE: u32 = 1u32;
pub const LANSCAPE: u32 = 1u32;
pub const LEFT_JUSTIFIED: u32 = 0u32;
pub const LIGHT_SOURCE_DETECT_READY: u32 = 4u32;
pub const LIGHT_SOURCE_NEGATIVE: u32 = 4u32;
pub const LIGHT_SOURCE_POSITIVE: u32 = 2u32;
pub const LIGHT_SOURCE_PRESENT: u32 = 2u32;
pub const LIGHT_SOURCE_PRESENT_DETECT: u32 = 1u32;
pub const LIGHT_SOURCE_READY: u32 = 8u32;
pub const LIGHT_SOURCE_SELECT: u32 = 1u32;
pub const MAX_ANSI_CHAR: u32 = 255u32;
pub const MAX_IO_HANDLES: u32 = 16u32;
pub const MAX_RESERVED: u32 = 4u32;
pub const MCRO_ERROR_GENERAL_ERROR: u32 = 0u32;
pub const MCRO_ERROR_OFFLINE: u32 = 5u32;
pub const MCRO_ERROR_PAPER_EMPTY: u32 = 4u32;
pub const MCRO_ERROR_PAPER_JAM: u32 = 2u32;
pub const MCRO_ERROR_PAPER_PROBLEM: u32 = 3u32;
pub const MCRO_ERROR_USER_INTERVENTION: u32 = 6u32;
pub const MCRO_STATUS_OK: u32 = 1u32;
pub const MICR_READER: u32 = 1048576u32;
pub const MICR_READER_READY: u32 = 65536u32;
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct MINIDRV_TRANSFER_CONTEXT {
    pub lSize: i32,
    pub lWidthInPixels: i32,
    pub lLines: i32,
    pub lDepth: i32,
    pub lXRes: i32,
    pub lYRes: i32,
    pub lCompression: i32,
    pub guidFormatID: windows_core::GUID,
    pub tymed: i32,
    pub hFile: isize,
    pub cbOffset: i32,
    pub lBufferSize: i32,
    pub lActiveBuffer: i32,
    pub lNumBuffers: i32,
    pub pBaseBuffer: *mut u8,
    pub pTransferBuffer: *mut u8,
    pub bTransferDataCB: windows_core::BOOL,
    pub bClassDrvAllocBuf: windows_core::BOOL,
    pub lClientAddress: isize,
    pub pIWiaMiniDrvCallBack: core::mem::ManuallyDrop<Option<IWiaMiniDrvCallBack>>,
    pub lImageSize: i32,
    pub lHeaderSize: i32,
    pub lItemSize: i32,
    pub cbWidthInBytes: i32,
    pub lPage: i32,
    pub lCurIfdOffset: i32,
    pub lPrevIfdOffset: i32,
}
impl Default for MINIDRV_TRANSFER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIRRORED: u32 = 1u32;
pub const MULTIPLE_FEED: u32 = 512u32;
pub const NEXT_PAGE: u32 = 128u32;
pub const PAPER_JAM: u32 = 32u32;
pub const PATCH_CODE_READER: u32 = 524288u32;
pub const PATCH_CODE_READER_READY: u32 = 32768u32;
pub const PATH_COVER_UP: u32 = 16u32;
pub const PORTRAIT: u32 = 0u32;
pub const POWERMODE_BATTERY: u32 = 2u32;
pub const POWERMODE_LINE: u32 = 1u32;
pub const PREFEED: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RANGEVALUE {
    pub lMin: i32,
    pub lMax: i32,
    pub lStep: i32,
}
pub const RIGHT_JUSTIFIED: u32 = 2u32;
pub const ROT180: u32 = 2u32;
pub const ROT270: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCANINFO {
    pub ADF: i32,
    pub TPA: i32,
    pub Endorser: i32,
    pub OpticalXResolution: i32,
    pub OpticalYResolution: i32,
    pub BedWidth: i32,
    pub BedHeight: i32,
    pub IntensityRange: RANGEVALUE,
    pub ContrastRange: RANGEVALUE,
    pub SupportedCompressionType: i32,
    pub SupportedDataTypes: i32,
    pub WidthPixels: i32,
    pub WidthBytes: i32,
    pub Lines: i32,
    pub DataType: i32,
    pub PixelBits: i32,
    pub Intensity: i32,
    pub Contrast: i32,
    pub Xresolution: i32,
    pub Yresolution: i32,
    pub Window: SCANWINDOW,
    pub DitherPattern: i32,
    pub Negative: i32,
    pub Mirror: i32,
    pub AutoBack: i32,
    pub ColorDitherPattern: i32,
    pub ToneMap: i32,
    pub Compression: i32,
    pub RawDataFormat: i32,
    pub RawPixelOrder: i32,
    pub bNeedDataAlignment: i32,
    pub DelayBetweenRead: i32,
    pub MaxBufferSize: i32,
    pub DeviceIOHandles: [super::super::Foundation::HANDLE; 16],
    pub lReserved: [i32; 4],
    pub pMicroDriverContext: *mut core::ffi::c_void,
}
impl Default for SCANINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCANMODE_FINALSCAN: u32 = 0u32;
pub const SCANMODE_PREVIEWSCAN: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCANWINDOW {
    pub xPos: i32,
    pub yPos: i32,
    pub xExtent: i32,
    pub yExtent: i32,
}
pub const SCAN_FINISHED: u32 = 30u32;
pub const SCAN_FIRST: u32 = 10u32;
pub const SCAN_NEXT: u32 = 20u32;
pub const SHELLEX_WIAUIEXTENSION_NAME: windows_core::PCWSTR = windows_core::w!("WiaDialogExtensionHandlers");
pub const STOR: u32 = 2048u32;
pub const STORAGE_FULL: u32 = 256u32;
pub const STORAGE_READY: u32 = 128u32;
pub const SUPPORT_BW: u32 = 2u32;
pub const SUPPORT_COLOR: u32 = 1u32;
pub const SUPPORT_GRAYSCALE: u32 = 4u32;
pub const TOP_JUSTIFIED: u32 = 0u32;
pub const TRANSPARENCY_DYNAMIC_FRAME_SUPPORT: u32 = 1u32;
pub const TRANSPARENCY_STATIC_FRAME_SUPPORT: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TWAIN_CAPABILITY {
    pub lSize: i32,
    pub lMSG: i32,
    pub lCapID: i32,
    pub lConType: i32,
    pub lRC: i32,
    pub lCC: i32,
    pub lDataSize: i32,
    pub Data: [u8; 1],
}
impl Default for TWAIN_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TYMED_CALLBACK: u32 = 128u32;
pub const TYMED_MULTIPAGE_CALLBACK: u32 = 512u32;
pub const TYMED_MULTIPAGE_FILE: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VAL {
    pub lVal: i32,
    pub dblVal: f64,
    pub pGuid: *mut windows_core::GUID,
    pub pScanInfo: *mut SCANINFO,
    pub handle: super::super::Foundation::HGLOBAL,
    pub ppButtonNames: *mut *mut u16,
    pub pHandle: *mut super::super::Foundation::HANDLE,
    pub lReserved: i32,
    pub szVal: [i8; 255],
}
impl Default for VAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHITEBALANCE_AUTO: u32 = 2u32;
pub const WHITEBALANCE_DAYLIGHT: u32 = 4u32;
pub const WHITEBALANCE_FLASH: u32 = 7u32;
pub const WHITEBALANCE_FLORESCENT: u32 = 5u32;
pub const WHITEBALANCE_MANUAL: u32 = 1u32;
pub const WHITEBALANCE_ONEPUSH_AUTO: u32 = 3u32;
pub const WHITEBALANCE_TUNGSTEN: u32 = 6u32;
#[repr(C)]
pub struct WIAS_CHANGED_VALUE_INFO {
    pub bChanged: windows_core::BOOL,
    pub vt: i32,
    pub Old: WIAS_CHANGED_VALUE_INFO_0,
    pub Current: WIAS_CHANGED_VALUE_INFO_1,
}
impl Clone for WIAS_CHANGED_VALUE_INFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for WIAS_CHANGED_VALUE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WIAS_CHANGED_VALUE_INFO_1 {
    pub lVal: i32,
    pub fltVal: f32,
    pub bstrVal: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub guidVal: windows_core::GUID,
}
impl Clone for WIAS_CHANGED_VALUE_INFO_1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for WIAS_CHANGED_VALUE_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union WIAS_CHANGED_VALUE_INFO_0 {
    pub lVal: i32,
    pub fltVal: f32,
    pub bstrVal: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub guidVal: windows_core::GUID,
}
impl Clone for WIAS_CHANGED_VALUE_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for WIAS_CHANGED_VALUE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIAS_DOWN_SAMPLE_INFO {
    pub ulOriginalWidth: u32,
    pub ulOriginalHeight: u32,
    pub ulBitsPerPixel: u32,
    pub ulXRes: u32,
    pub ulYRes: u32,
    pub ulDownSampledWidth: u32,
    pub ulDownSampledHeight: u32,
    pub ulActualSize: u32,
    pub ulDestBufSize: u32,
    pub ulSrcBufSize: u32,
    pub pSrcBuffer: *mut u8,
    pub pDestBuffer: *mut u8,
}
impl Default for WIAS_DOWN_SAMPLE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIAS_ENDORSER_INFO {
    pub ulPageCount: u32,
    pub ulNumEndorserValues: u32,
    pub pEndorserValues: *mut WIAS_ENDORSER_VALUE,
}
impl Default for WIAS_ENDORSER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIAS_ENDORSER_VALUE {
    pub wszTokenName: windows_core::PWSTR,
    pub wszValue: windows_core::PWSTR,
}
pub const WIAU_DEBUG_TSTR: windows_core::PCSTR = windows_core::s!("S");
pub const WIAVIDEO_CREATING_VIDEO: WIAVIDEO_STATE = WIAVIDEO_STATE(2i32);
pub const WIAVIDEO_DESTROYING_VIDEO: WIAVIDEO_STATE = WIAVIDEO_STATE(6i32);
pub const WIAVIDEO_NO_VIDEO: WIAVIDEO_STATE = WIAVIDEO_STATE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIAVIDEO_STATE(pub i32);
pub const WIAVIDEO_VIDEO_CREATED: WIAVIDEO_STATE = WIAVIDEO_STATE(3i32);
pub const WIAVIDEO_VIDEO_PAUSED: WIAVIDEO_STATE = WIAVIDEO_STATE(5i32);
pub const WIAVIDEO_VIDEO_PLAYING: WIAVIDEO_STATE = WIAVIDEO_STATE(4i32);
pub const WIA_ACTION_EVENT: u32 = 2u32;
pub const WIA_ADVANCED_PREVIEW: u32 = 0u32;
pub const WIA_ALARM_BEEP1: u32 = 1u32;
pub const WIA_ALARM_BEEP10: u32 = 10u32;
pub const WIA_ALARM_BEEP2: u32 = 2u32;
pub const WIA_ALARM_BEEP3: u32 = 3u32;
pub const WIA_ALARM_BEEP4: u32 = 4u32;
pub const WIA_ALARM_BEEP5: u32 = 5u32;
pub const WIA_ALARM_BEEP6: u32 = 6u32;
pub const WIA_ALARM_BEEP7: u32 = 7u32;
pub const WIA_ALARM_BEEP8: u32 = 8u32;
pub const WIA_ALARM_BEEP9: u32 = 9u32;
pub const WIA_ALARM_NONE: u32 = 0u32;
pub const WIA_AUTO_CROP_DISABLED: u32 = 0u32;
pub const WIA_AUTO_CROP_MULTI: u32 = 2u32;
pub const WIA_AUTO_CROP_SINGLE: u32 = 1u32;
pub const WIA_AUTO_DESKEW_OFF: u32 = 1u32;
pub const WIA_AUTO_DESKEW_ON: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIA_BARCODES {
    pub Tag: u32,
    pub Version: u32,
    pub Size: u32,
    pub Count: u32,
    pub Barcodes: [WIA_BARCODE_INFO; 1],
}
impl Default for WIA_BARCODES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WIA_BARCODE_AUTO_SEARCH: u32 = 4u32;
pub const WIA_BARCODE_AZTEC: u32 = 36u32;
pub const WIA_BARCODE_CODABAR: u32 = 2u32;
pub const WIA_BARCODE_CODE128: u32 = 9u32;
pub const WIA_BARCODE_CODE128A: u32 = 10u32;
pub const WIA_BARCODE_CODE128B: u32 = 11u32;
pub const WIA_BARCODE_CODE128C: u32 = 12u32;
pub const WIA_BARCODE_CODE39: u32 = 5u32;
pub const WIA_BARCODE_CODE39_FULLASCII: u32 = 7u32;
pub const WIA_BARCODE_CODE39_MOD43: u32 = 6u32;
pub const WIA_BARCODE_CODE93: u32 = 8u32;
pub const WIA_BARCODE_CPCBINARY: u32 = 29u32;
pub const WIA_BARCODE_CUSTOMBASE: u32 = 32768u32;
pub const WIA_BARCODE_DATAMATRIX: u32 = 38u32;
pub const WIA_BARCODE_DATASTRIP: u32 = 39u32;
pub const WIA_BARCODE_EAN13: u32 = 17u32;
pub const WIA_BARCODE_EAN8: u32 = 16u32;
pub const WIA_BARCODE_EZCODE: u32 = 40u32;
pub const WIA_BARCODE_FIM: u32 = 30u32;
pub const WIA_BARCODE_GS1128: u32 = 13u32;
pub const WIA_BARCODE_GS1DATABAR: u32 = 14u32;
pub const WIA_BARCODE_HIGH_CAPACITY_COLOR: u32 = 26u32;
pub const WIA_BARCODE_HORIZONTAL_SEARCH: u32 = 0u32;
pub const WIA_BARCODE_HORIZONTAL_VERTICAL_SEARCH: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIA_BARCODE_INFO {
    pub Size: u32,
    pub Type: u32,
    pub Page: u32,
    pub Confidence: u32,
    pub XOffset: u32,
    pub YOffset: u32,
    pub Rotation: u32,
    pub Length: u32,
    pub Text: [u16; 1],
}
impl Default for WIA_BARCODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WIA_BARCODE_INTELLIGENT_MAIL: u32 = 23u32;
pub const WIA_BARCODE_INTERLEAVED_2OF5: u32 = 4u32;
pub const WIA_BARCODE_ITF14: u32 = 15u32;
pub const WIA_BARCODE_JAN: u32 = 34u32;
pub const WIA_BARCODE_MAXICODE: u32 = 27u32;
pub const WIA_BARCODE_MSI: u32 = 33u32;
pub const WIA_BARCODE_NONINTERLEAVED_2OF5: u32 = 3u32;
pub const WIA_BARCODE_PDF417: u32 = 28u32;
pub const WIA_BARCODE_PHARMACODE: u32 = 31u32;
pub const WIA_BARCODE_PLANET: u32 = 22u32;
pub const WIA_BARCODE_PLESSEY: u32 = 32u32;
pub const WIA_BARCODE_POSTBAR: u32 = 24u32;
pub const WIA_BARCODE_POSTNETA: u32 = 18u32;
pub const WIA_BARCODE_POSTNETB: u32 = 19u32;
pub const WIA_BARCODE_POSTNETC: u32 = 20u32;
pub const WIA_BARCODE_POSTNET_DPBC: u32 = 21u32;
pub const WIA_BARCODE_QRCODE: u32 = 41u32;
pub const WIA_BARCODE_READER_AUTO: u32 = 1u32;
pub const WIA_BARCODE_READER_DISABLED: u32 = 0u32;
pub const WIA_BARCODE_READER_FEEDER_BACK: u32 = 4u32;
pub const WIA_BARCODE_READER_FEEDER_DUPLEX: u32 = 5u32;
pub const WIA_BARCODE_READER_FEEDER_FRONT: u32 = 3u32;
pub const WIA_BARCODE_READER_FLATBED: u32 = 2u32;
pub const WIA_BARCODE_RM4SCC: u32 = 25u32;
pub const WIA_BARCODE_SHOTCODE: u32 = 42u32;
pub const WIA_BARCODE_SMALLAZTEC: u32 = 37u32;
pub const WIA_BARCODE_SPARQCODE: u32 = 43u32;
pub const WIA_BARCODE_TELEPEN: u32 = 35u32;
pub const WIA_BARCODE_UPCA: u32 = 0u32;
pub const WIA_BARCODE_UPCE: u32 = 1u32;
pub const WIA_BARCODE_VERTICAL_HORIZONTAL_SEARCH: u32 = 3u32;
pub const WIA_BARCODE_VERTICAL_SEARCH: u32 = 1u32;
pub const WIA_BASIC_PREVIEW: u32 = 1u32;
pub const WIA_BLANK_PAGE_DETECTION_DISABLED: u32 = 0u32;
pub const WIA_BLANK_PAGE_DISCARD: u32 = 1u32;
pub const WIA_BLANK_PAGE_JOB_SEPARATOR: u32 = 2u32;
pub const WIA_CATEGORY_AUTO: windows_core::GUID = windows_core::GUID::from_u128(0xdefe5fd8_6c97_4dde_b11e_cb509b270e11);
pub const WIA_CATEGORY_BARCODE_READER: windows_core::GUID = windows_core::GUID::from_u128(0x36e178a0_473f_494b_af8f_6c3f6d7486fc);
pub const WIA_CATEGORY_ENDORSER: windows_core::GUID = windows_core::GUID::from_u128(0x47102cc3_127f_4771_adfc_991ab8ee1e97);
pub const WIA_CATEGORY_FEEDER: windows_core::GUID = windows_core::GUID::from_u128(0xfe131934_f84c_42ad_8da4_6129cddd7288);
pub const WIA_CATEGORY_FEEDER_BACK: windows_core::GUID = windows_core::GUID::from_u128(0x61ca74d4_39db_42aa_89b1_8c19c9cd4c23);
pub const WIA_CATEGORY_FEEDER_FRONT: windows_core::GUID = windows_core::GUID::from_u128(0x4823175c_3b28_487b_a7e6_eebc17614fd1);
pub const WIA_CATEGORY_FILM: windows_core::GUID = windows_core::GUID::from_u128(0xfcf65be7_3ce3_4473_af85_f5d37d21b68a);
pub const WIA_CATEGORY_FINISHED_FILE: windows_core::GUID = windows_core::GUID::from_u128(0xff2b77ca_cf84_432b_a735_3a130dde2a88);
pub const WIA_CATEGORY_FLATBED: windows_core::GUID = windows_core::GUID::from_u128(0xfb607b1f_43f3_488b_855b_fb703ec342a6);
pub const WIA_CATEGORY_FOLDER: windows_core::GUID = windows_core::GUID::from_u128(0xc692a446_6f5a_481d_85bb_92e2e86fd30a);
pub const WIA_CATEGORY_IMPRINTER: windows_core::GUID = windows_core::GUID::from_u128(0xfc65016d_9202_43dd_91a7_64c2954cfb8b);
pub const WIA_CATEGORY_MICR_READER: windows_core::GUID = windows_core::GUID::from_u128(0x3b86c1ec_71bc_4645_b4d5_1b19da2be978);
pub const WIA_CATEGORY_PATCH_CODE_READER: windows_core::GUID = windows_core::GUID::from_u128(0x8faa1a6d_9c8a_42cd_98b3_ee9700cbc74f);
pub const WIA_CATEGORY_ROOT: windows_core::GUID = windows_core::GUID::from_u128(0xf193526f_59b8_4a26_9888_e16e4f97ce10);
pub const WIA_CMD_BUILD_DEVICE_TREE: windows_core::GUID = windows_core::GUID::from_u128(0x9cba5ce0_dbea_11d2_8416_00c04fa36145);
pub const WIA_CMD_CHANGE_DOCUMENT: windows_core::GUID = windows_core::GUID::from_u128(0x04e725b0_acae_11d2_a093_00c04f72dc3c);
pub const WIA_CMD_DELETE_ALL_ITEMS: windows_core::GUID = windows_core::GUID::from_u128(0xe208c170_acad_11d2_a093_00c04f72dc3c);
pub const WIA_CMD_DELETE_DEVICE_TREE: windows_core::GUID = windows_core::GUID::from_u128(0x73815942_dbea_11d2_8416_00c04fa36145);
pub const WIA_CMD_DIAGNOSTIC: windows_core::GUID = windows_core::GUID::from_u128(0x10ff52f5_de04_4cf0_a5ad_691f8dce0141);
pub const WIA_CMD_FORMAT: windows_core::GUID = windows_core::GUID::from_u128(0xc3a693aa_f788_4d34_a5b0_be7190759a24);
pub const WIA_CMD_PAUSE_FEEDER: windows_core::GUID = windows_core::GUID::from_u128(0x50985e4d_a5b2_4b71_9c95_6d7d7c469a43);
pub const WIA_CMD_START_FEEDER: windows_core::GUID = windows_core::GUID::from_u128(0x5a9df6c9_5f2d_4a39_9d6c_00456d047f00);
pub const WIA_CMD_STOP_FEEDER: windows_core::GUID = windows_core::GUID::from_u128(0xd847b06d_3905_459c_9509_9b29cdb691e7);
pub const WIA_CMD_SYNCHRONIZE: windows_core::GUID = windows_core::GUID::from_u128(0x9b26b7b2_acad_11d2_a093_00c04f72dc3c);
pub const WIA_CMD_TAKE_PICTURE: windows_core::GUID = windows_core::GUID::from_u128(0xaf933cac_acad_11d2_a093_00c04f72dc3c);
pub const WIA_CMD_UNLOAD_DOCUMENT: windows_core::GUID = windows_core::GUID::from_u128(0x1f3b3d8e_acae_11d2_a093_00c04f72dc3c);
pub const WIA_COLOR_DROP_BLUE: u32 = 3u32;
pub const WIA_COLOR_DROP_DISABLED: u32 = 0u32;
pub const WIA_COLOR_DROP_GREEN: u32 = 2u32;
pub const WIA_COLOR_DROP_RED: u32 = 1u32;
pub const WIA_COLOR_DROP_RGB: u32 = 4u32;
pub const WIA_COMPRESSION_AUTO: u32 = 100u32;
pub const WIA_COMPRESSION_BI_RLE4: u32 = 1u32;
pub const WIA_COMPRESSION_BI_RLE8: u32 = 2u32;
pub const WIA_COMPRESSION_G3: u32 = 3u32;
pub const WIA_COMPRESSION_G4: u32 = 4u32;
pub const WIA_COMPRESSION_JBIG: u32 = 6u32;
pub const WIA_COMPRESSION_JPEG: u32 = 5u32;
pub const WIA_COMPRESSION_JPEG2K: u32 = 7u32;
pub const WIA_COMPRESSION_NONE: u32 = 0u32;
pub const WIA_COMPRESSION_PNG: u32 = 8u32;
pub const WIA_DATA_AUTO: u32 = 100u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIA_DATA_CALLBACK_HEADER {
    pub lSize: i32,
    pub guidFormatID: windows_core::GUID,
    pub lBufferSize: i32,
    pub lPageCount: i32,
}
pub const WIA_DATA_COLOR: u32 = 3u32;
pub const WIA_DATA_COLOR_DITHER: u32 = 5u32;
pub const WIA_DATA_COLOR_THRESHOLD: u32 = 4u32;
pub const WIA_DATA_DITHER: u32 = 1u32;
pub const WIA_DATA_GRAYSCALE: u32 = 2u32;
pub const WIA_DATA_RAW_BGR: u32 = 7u32;
pub const WIA_DATA_RAW_CMY: u32 = 10u32;
pub const WIA_DATA_RAW_CMYK: u32 = 11u32;
pub const WIA_DATA_RAW_RGB: u32 = 6u32;
pub const WIA_DATA_RAW_YUV: u32 = 8u32;
pub const WIA_DATA_RAW_YUVK: u32 = 9u32;
pub const WIA_DATA_THRESHOLD: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIA_DATA_TRANSFER_INFO {
    pub ulSize: u32,
    pub ulSection: u32,
    pub ulBufferSize: u32,
    pub bDoubleBuffer: windows_core::BOOL,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ulReserved3: u32,
}
pub const WIA_DEPTH_AUTO: u32 = 0u32;
pub const WIA_DEVICE_COMMANDS: u32 = 1u32;
pub const WIA_DEVICE_CONNECTED: u32 = 1u32;
pub const WIA_DEVICE_DIALOG_SINGLE_IMAGE: u32 = 2u32;
pub const WIA_DEVICE_DIALOG_USE_COMMON_UI: u32 = 4u32;
pub const WIA_DEVICE_EVENTS: u32 = 2u32;
pub const WIA_DEVICE_NOT_CONNECTED: u32 = 0u32;
pub const WIA_DEVINFO_ENUM_ALL: u32 = 15u32;
pub const WIA_DEVINFO_ENUM_LOCAL: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WIA_DEV_CAP {
    pub guid: windows_core::GUID,
    pub ulFlags: u32,
    pub bstrName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrDescription: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrIcon: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrCommandline: core::mem::ManuallyDrop<windows_core::BSTR>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIA_DEV_CAP_DRV {
    pub guid: *mut windows_core::GUID,
    pub ulFlags: u32,
    pub wszName: windows_core::PWSTR,
    pub wszDescription: windows_core::PWSTR,
    pub wszIcon: windows_core::PWSTR,
}
impl Default for WIA_DEV_CAP_DRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WIA_DIP_BAUDRATE: u32 = 12u32;
pub const WIA_DIP_BAUDRATE_STR: windows_core::PCWSTR = windows_core::w!("BaudRate");
pub const WIA_DIP_DEV_DESC: u32 = 4u32;
pub const WIA_DIP_DEV_DESC_STR: windows_core::PCWSTR = windows_core::w!("Description");
pub const WIA_DIP_DEV_ID: u32 = 2u32;
pub const WIA_DIP_DEV_ID_STR: windows_core::PCWSTR = windows_core::w!("Unique Device ID");
pub const WIA_DIP_DEV_NAME: u32 = 7u32;
pub const WIA_DIP_DEV_NAME_STR: windows_core::PCWSTR = windows_core::w!("Name");
pub const WIA_DIP_DEV_TYPE: u32 = 5u32;
pub const WIA_DIP_DEV_TYPE_STR: windows_core::PCWSTR = windows_core::w!("Type");
pub const WIA_DIP_DRIVER_VERSION: u32 = 15u32;
pub const WIA_DIP_DRIVER_VERSION_STR: windows_core::PCWSTR = windows_core::w!("Driver Version");
pub const WIA_DIP_FIRST: u32 = 2u32;
pub const WIA_DIP_HW_CONFIG: u32 = 11u32;
pub const WIA_DIP_HW_CONFIG_STR: windows_core::PCWSTR = windows_core::w!("Hardware Configuration");
pub const WIA_DIP_PNP_ID: u32 = 16u32;
pub const WIA_DIP_PNP_ID_STR: windows_core::PCWSTR = windows_core::w!("PnP ID String");
pub const WIA_DIP_PORT_NAME: u32 = 6u32;
pub const WIA_DIP_PORT_NAME_STR: windows_core::PCWSTR = windows_core::w!("Port");
pub const WIA_DIP_REMOTE_DEV_ID: u32 = 9u32;
pub const WIA_DIP_REMOTE_DEV_ID_STR: windows_core::PCWSTR = windows_core::w!("Remote Device ID");
pub const WIA_DIP_SERVER_NAME: u32 = 8u32;
pub const WIA_DIP_SERVER_NAME_STR: windows_core::PCWSTR = windows_core::w!("Server");
pub const WIA_DIP_STI_DRIVER_VERSION: u32 = 17u32;
pub const WIA_DIP_STI_DRIVER_VERSION_STR: windows_core::PCWSTR = windows_core::w!("STI Driver Version");
pub const WIA_DIP_STI_GEN_CAPABILITIES: u32 = 13u32;
pub const WIA_DIP_STI_GEN_CAPABILITIES_STR: windows_core::PCWSTR = windows_core::w!("STI Generic Capabilities");
pub const WIA_DIP_UI_CLSID: u32 = 10u32;
pub const WIA_DIP_UI_CLSID_STR: windows_core::PCWSTR = windows_core::w!("UI Class ID");
pub const WIA_DIP_VEND_DESC: u32 = 3u32;
pub const WIA_DIP_VEND_DESC_STR: windows_core::PCWSTR = windows_core::w!("Manufacturer");
pub const WIA_DIP_WIA_VERSION: u32 = 14u32;
pub const WIA_DIP_WIA_VERSION_STR: windows_core::PCWSTR = windows_core::w!("WIA Version");
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct WIA_DITHER_PATTERN_DATA {
    pub lSize: i32,
    pub bstrPatternName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub lPatternWidth: i32,
    pub lPatternLength: i32,
    pub cbPattern: i32,
    pub pbPattern: *mut u8,
}
impl Default for WIA_DITHER_PATTERN_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WIA_DONT_SHOW_PREVIEW_CONTROL: u32 = 1u32;
pub const WIA_DONT_USE_SEGMENTATION_FILTER: u32 = 1u32;
pub const WIA_DPA_CONNECT_STATUS: u32 = 1027u32;
pub const WIA_DPA_CONNECT_STATUS_STR: windows_core::PCWSTR = windows_core::w!("Connect Status");
pub const WIA_DPA_DEVICE_TIME: u32 = 1028u32;
pub const WIA_DPA_DEVICE_TIME_STR: windows_core::PCWSTR = windows_core::w!("Device Time");
pub const WIA_DPA_FIRMWARE_VERSION: u32 = 1026u32;
pub const WIA_DPA_FIRMWARE_VERSION_STR: windows_core::PCWSTR = windows_core::w!("Firmware Version");
pub const WIA_DPC_ARTIST: u32 = 2091u32;
pub const WIA_DPC_ARTIST_STR: windows_core::PCWSTR = windows_core::w!("Artist");
pub const WIA_DPC_BATTERY_STATUS: u32 = 2065u32;
pub const WIA_DPC_BATTERY_STATUS_STR: windows_core::PCWSTR = windows_core::w!("Battery Status");
pub const WIA_DPC_BURST_INTERVAL: u32 = 2075u32;
pub const WIA_DPC_BURST_INTERVAL_STR: windows_core::PCWSTR = windows_core::w!("Burst Interval");
pub const WIA_DPC_BURST_NUMBER: u32 = 2076u32;
pub const WIA_DPC_BURST_NUMBER_STR: windows_core::PCWSTR = windows_core::w!("Burst Number");
pub const WIA_DPC_CAPTURE_DELAY: u32 = 2082u32;
pub const WIA_DPC_CAPTURE_DELAY_STR: windows_core::PCWSTR = windows_core::w!("Capture Delay");
pub const WIA_DPC_CAPTURE_MODE: u32 = 2081u32;
pub const WIA_DPC_CAPTURE_MODE_STR: windows_core::PCWSTR = windows_core::w!("Capture Mode");
pub const WIA_DPC_COMPRESSION_SETTING: u32 = 2071u32;
pub const WIA_DPC_COMPRESSION_SETTING_STR: windows_core::PCWSTR = windows_core::w!("Compression Setting");
pub const WIA_DPC_CONTRAST: u32 = 2080u32;
pub const WIA_DPC_CONTRAST_STR: windows_core::PCWSTR = windows_core::w!("Contrast");
pub const WIA_DPC_COPYRIGHT_INFO: u32 = 2092u32;
pub const WIA_DPC_COPYRIGHT_INFO_STR: windows_core::PCWSTR = windows_core::w!("Copyright Info");
pub const WIA_DPC_DIGITAL_ZOOM: u32 = 2078u32;
pub const WIA_DPC_DIGITAL_ZOOM_STR: windows_core::PCWSTR = windows_core::w!("Digital Zoom");
pub const WIA_DPC_DIMENSION: u32 = 2070u32;
pub const WIA_DPC_DIMENSION_STR: windows_core::PCWSTR = windows_core::w!("Dimension");
pub const WIA_DPC_EFFECT_MODE: u32 = 2077u32;
pub const WIA_DPC_EFFECT_MODE_STR: windows_core::PCWSTR = windows_core::w!("Effect Mode");
pub const WIA_DPC_EXPOSURE_COMP: u32 = 2053u32;
pub const WIA_DPC_EXPOSURE_COMP_STR: windows_core::PCWSTR = windows_core::w!("Exposure Compensation");
pub const WIA_DPC_EXPOSURE_INDEX: u32 = 2083u32;
pub const WIA_DPC_EXPOSURE_INDEX_STR: windows_core::PCWSTR = windows_core::w!("Exposure Index");
pub const WIA_DPC_EXPOSURE_METERING_MODE: u32 = 2084u32;
pub const WIA_DPC_EXPOSURE_METERING_MODE_STR: windows_core::PCWSTR = windows_core::w!("Exposure Metering Mode");
pub const WIA_DPC_EXPOSURE_MODE: u32 = 2052u32;
pub const WIA_DPC_EXPOSURE_MODE_STR: windows_core::PCWSTR = windows_core::w!("Exposure Mode");
pub const WIA_DPC_EXPOSURE_TIME: u32 = 2054u32;
pub const WIA_DPC_EXPOSURE_TIME_STR: windows_core::PCWSTR = windows_core::w!("Exposure Time");
pub const WIA_DPC_FLASH_MODE: u32 = 2056u32;
pub const WIA_DPC_FLASH_MODE_STR: windows_core::PCWSTR = windows_core::w!("Flash Mode");
pub const WIA_DPC_FNUMBER: u32 = 2055u32;
pub const WIA_DPC_FNUMBER_STR: windows_core::PCWSTR = windows_core::w!("F Number");
pub const WIA_DPC_FOCAL_LENGTH: u32 = 2087u32;
pub const WIA_DPC_FOCAL_LENGTH_STR: windows_core::PCWSTR = windows_core::w!("Focus Length");
pub const WIA_DPC_FOCUS_DISTANCE: u32 = 2086u32;
pub const WIA_DPC_FOCUS_DISTANCE_STR: windows_core::PCWSTR = windows_core::w!("Focus Distance");
pub const WIA_DPC_FOCUS_MANUAL_DIST: u32 = 2058u32;
pub const WIA_DPC_FOCUS_MANUAL_DIST_STR: windows_core::PCWSTR = windows_core::w!("Focus Manual Dist");
pub const WIA_DPC_FOCUS_METERING: u32 = 2072u32;
pub const WIA_DPC_FOCUS_METERING_MODE: u32 = 2085u32;
pub const WIA_DPC_FOCUS_METERING_MODE_STR: windows_core::PCWSTR = windows_core::w!("Focus Metering Mode");
pub const WIA_DPC_FOCUS_METERING_STR: windows_core::PCWSTR = windows_core::w!("Focus Metering Mode");
pub const WIA_DPC_FOCUS_MODE: u32 = 2057u32;
pub const WIA_DPC_FOCUS_MODE_STR: windows_core::PCWSTR = windows_core::w!("Focus Mode");
pub const WIA_DPC_PAN_POSITION: u32 = 2060u32;
pub const WIA_DPC_PAN_POSITION_STR: windows_core::PCWSTR = windows_core::w!("Pan Position");
pub const WIA_DPC_PICTURES_REMAINING: u32 = 2051u32;
pub const WIA_DPC_PICTURES_REMAINING_STR: windows_core::PCWSTR = windows_core::w!("Pictures Remaining");
pub const WIA_DPC_PICTURES_TAKEN: u32 = 2050u32;
pub const WIA_DPC_PICTURES_TAKEN_STR: windows_core::PCWSTR = windows_core::w!("Pictures Taken");
pub const WIA_DPC_PICT_HEIGHT: u32 = 2069u32;
pub const WIA_DPC_PICT_HEIGHT_STR: windows_core::PCWSTR = windows_core::w!("Picture Height");
pub const WIA_DPC_PICT_WIDTH: u32 = 2068u32;
pub const WIA_DPC_PICT_WIDTH_STR: windows_core::PCWSTR = windows_core::w!("Picture Width");
pub const WIA_DPC_POWER_MODE: u32 = 2064u32;
pub const WIA_DPC_POWER_MODE_STR: windows_core::PCWSTR = windows_core::w!("Power Mode");
pub const WIA_DPC_RGB_GAIN: u32 = 2088u32;
pub const WIA_DPC_RGB_GAIN_STR: windows_core::PCWSTR = windows_core::w!("RGB Gain");
pub const WIA_DPC_SHARPNESS: u32 = 2079u32;
pub const WIA_DPC_SHARPNESS_STR: windows_core::PCWSTR = windows_core::w!("Sharpness");
pub const WIA_DPC_THUMB_HEIGHT: u32 = 2067u32;
pub const WIA_DPC_THUMB_HEIGHT_STR: windows_core::PCWSTR = windows_core::w!("Thumbnail Height");
pub const WIA_DPC_THUMB_WIDTH: u32 = 2066u32;
pub const WIA_DPC_THUMB_WIDTH_STR: windows_core::PCWSTR = windows_core::w!("Thumbnail Width");
pub const WIA_DPC_TILT_POSITION: u32 = 2061u32;
pub const WIA_DPC_TILT_POSITION_STR: windows_core::PCWSTR = windows_core::w!("Tilt Position");
pub const WIA_DPC_TIMELAPSE_INTERVAL: u32 = 2073u32;
pub const WIA_DPC_TIMELAPSE_INTERVAL_STR: windows_core::PCWSTR = windows_core::w!("Timelapse Interval");
pub const WIA_DPC_TIMELAPSE_NUMBER: u32 = 2074u32;
pub const WIA_DPC_TIMELAPSE_NUMBER_STR: windows_core::PCWSTR = windows_core::w!("Timelapse Number");
pub const WIA_DPC_TIMER_MODE: u32 = 2062u32;
pub const WIA_DPC_TIMER_MODE_STR: windows_core::PCWSTR = windows_core::w!("Timer Mode");
pub const WIA_DPC_TIMER_VALUE: u32 = 2063u32;
pub const WIA_DPC_TIMER_VALUE_STR: windows_core::PCWSTR = windows_core::w!("Timer Value");
pub const WIA_DPC_UPLOAD_URL: u32 = 2090u32;
pub const WIA_DPC_UPLOAD_URL_STR: windows_core::PCWSTR = windows_core::w!("Upload URL");
pub const WIA_DPC_WHITE_BALANCE: u32 = 2089u32;
pub const WIA_DPC_WHITE_BALANCE_STR: windows_core::PCWSTR = windows_core::w!("White Balance");
pub const WIA_DPC_ZOOM_POSITION: u32 = 2059u32;
pub const WIA_DPC_ZOOM_POSITION_STR: windows_core::PCWSTR = windows_core::w!("Zoom Position");
pub const WIA_DPF_FIRST: u32 = 3330u32;
pub const WIA_DPF_MOUNT_POINT: u32 = 3330u32;
pub const WIA_DPF_MOUNT_POINT_STR: windows_core::PCWSTR = windows_core::w!("Directory mount point");
pub const WIA_DPS_DEVICE_ID: u32 = 3114u32;
pub const WIA_DPS_DEVICE_ID_STR: windows_core::PCWSTR = windows_core::w!("Device ID");
pub const WIA_DPS_DITHER_PATTERN_DATA: u32 = 3085u32;
pub const WIA_DPS_DITHER_PATTERN_DATA_STR: windows_core::PCWSTR = windows_core::w!("Dither Pattern Data");
pub const WIA_DPS_DITHER_SELECT: u32 = 3084u32;
pub const WIA_DPS_DITHER_SELECT_STR: windows_core::PCWSTR = windows_core::w!("Dither Select");
pub const WIA_DPS_DOCUMENT_HANDLING_CAPABILITIES: u32 = 3086u32;
pub const WIA_DPS_DOCUMENT_HANDLING_CAPABILITIES_STR: windows_core::PCWSTR = windows_core::w!("Document Handling Capabilities");
pub const WIA_DPS_DOCUMENT_HANDLING_CAPACITY: u32 = 3089u32;
pub const WIA_DPS_DOCUMENT_HANDLING_CAPACITY_STR: windows_core::PCWSTR = windows_core::w!("Document Handling Capacity");
pub const WIA_DPS_DOCUMENT_HANDLING_SELECT: u32 = 3088u32;
pub const WIA_DPS_DOCUMENT_HANDLING_SELECT_STR: windows_core::PCWSTR = windows_core::w!("Document Handling Select");
pub const WIA_DPS_DOCUMENT_HANDLING_STATUS: u32 = 3087u32;
pub const WIA_DPS_DOCUMENT_HANDLING_STATUS_STR: windows_core::PCWSTR = windows_core::w!("Document Handling Status");
pub const WIA_DPS_ENDORSER_CHARACTERS: u32 = 3092u32;
pub const WIA_DPS_ENDORSER_CHARACTERS_STR: windows_core::PCWSTR = windows_core::w!("Endorser Characters");
pub const WIA_DPS_ENDORSER_STRING: u32 = 3093u32;
pub const WIA_DPS_ENDORSER_STRING_STR: windows_core::PCWSTR = windows_core::w!("Endorser String");
pub const WIA_DPS_FILTER_SELECT: u32 = 3083u32;
pub const WIA_DPS_FILTER_SELECT_STR: windows_core::PCWSTR = windows_core::w!("Filter Select");
pub const WIA_DPS_FIRST: u32 = 3074u32;
pub const WIA_DPS_GLOBAL_IDENTITY: u32 = 3115u32;
pub const WIA_DPS_GLOBAL_IDENTITY_STR: windows_core::PCWSTR = windows_core::w!("Global Identity");
pub const WIA_DPS_HORIZONTAL_BED_REGISTRATION: u32 = 3079u32;
pub const WIA_DPS_HORIZONTAL_BED_REGISTRATION_STR: windows_core::PCWSTR = windows_core::w!("Horizontal Bed Registration");
pub const WIA_DPS_HORIZONTAL_BED_SIZE: u32 = 3074u32;
pub const WIA_DPS_HORIZONTAL_BED_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Horizontal Bed Size");
pub const WIA_DPS_HORIZONTAL_SHEET_FEED_SIZE: u32 = 3076u32;
pub const WIA_DPS_HORIZONTAL_SHEET_FEED_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Horizontal Sheet Feed Size");
pub const WIA_DPS_MAX_SCAN_TIME: u32 = 3095u32;
pub const WIA_DPS_MAX_SCAN_TIME_STR: windows_core::PCWSTR = windows_core::w!("Max Scan Time");
pub const WIA_DPS_MIN_HORIZONTAL_SHEET_FEED_SIZE: u32 = 3104u32;
pub const WIA_DPS_MIN_HORIZONTAL_SHEET_FEED_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Minimum Horizontal Sheet Feed Size");
pub const WIA_DPS_MIN_VERTICAL_SHEET_FEED_SIZE: u32 = 3105u32;
pub const WIA_DPS_MIN_VERTICAL_SHEET_FEED_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Minimum Vertical Sheet Feed Size");
pub const WIA_DPS_OPTICAL_XRES: u32 = 3090u32;
pub const WIA_DPS_OPTICAL_XRES_STR: windows_core::PCWSTR = windows_core::w!("Horizontal Optical Resolution");
pub const WIA_DPS_OPTICAL_YRES: u32 = 3091u32;
pub const WIA_DPS_OPTICAL_YRES_STR: windows_core::PCWSTR = windows_core::w!("Vertical Optical Resolution");
pub const WIA_DPS_PAD_COLOR: u32 = 3082u32;
pub const WIA_DPS_PAD_COLOR_STR: windows_core::PCWSTR = windows_core::w!("Pad Color");
pub const WIA_DPS_PAGES: u32 = 3096u32;
pub const WIA_DPS_PAGES_STR: windows_core::PCWSTR = windows_core::w!("Pages");
pub const WIA_DPS_PAGE_HEIGHT: u32 = 3099u32;
pub const WIA_DPS_PAGE_HEIGHT_STR: windows_core::PCWSTR = windows_core::w!("Page Height");
pub const WIA_DPS_PAGE_SIZE: u32 = 3097u32;
pub const WIA_DPS_PAGE_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Page Size");
pub const WIA_DPS_PAGE_WIDTH: u32 = 3098u32;
pub const WIA_DPS_PAGE_WIDTH_STR: windows_core::PCWSTR = windows_core::w!("Page Width");
pub const WIA_DPS_PLATEN_COLOR: u32 = 3081u32;
pub const WIA_DPS_PLATEN_COLOR_STR: windows_core::PCWSTR = windows_core::w!("Platen Color");
pub const WIA_DPS_PREVIEW: u32 = 3100u32;
pub const WIA_DPS_PREVIEW_STR: windows_core::PCWSTR = windows_core::w!("Preview");
pub const WIA_DPS_SCAN_AHEAD_PAGES: u32 = 3094u32;
pub const WIA_DPS_SCAN_AHEAD_PAGES_STR: windows_core::PCWSTR = windows_core::w!("Scan Ahead Pages");
pub const WIA_DPS_SCAN_AVAILABLE_ITEM: u32 = 3116u32;
pub const WIA_DPS_SCAN_AVAILABLE_ITEM_STR: windows_core::PCWSTR = windows_core::w!("Scan Available Item");
pub const WIA_DPS_SERVICE_ID: u32 = 3113u32;
pub const WIA_DPS_SERVICE_ID_STR: windows_core::PCWSTR = windows_core::w!("Service ID");
pub const WIA_DPS_SHEET_FEEDER_REGISTRATION: u32 = 3078u32;
pub const WIA_DPS_SHEET_FEEDER_REGISTRATION_STR: windows_core::PCWSTR = windows_core::w!("Sheet Feeder Registration");
pub const WIA_DPS_SHOW_PREVIEW_CONTROL: u32 = 3103u32;
pub const WIA_DPS_SHOW_PREVIEW_CONTROL_STR: windows_core::PCWSTR = windows_core::w!("Show preview control");
pub const WIA_DPS_TRANSPARENCY: u32 = 3101u32;
pub const WIA_DPS_TRANSPARENCY_CAPABILITIES: u32 = 3106u32;
pub const WIA_DPS_TRANSPARENCY_CAPABILITIES_STR: windows_core::PCWSTR = windows_core::w!("Transparency Adapter Capabilities");
pub const WIA_DPS_TRANSPARENCY_SELECT: u32 = 3102u32;
pub const WIA_DPS_TRANSPARENCY_SELECT_STR: windows_core::PCWSTR = windows_core::w!("Transparency Adapter Select");
pub const WIA_DPS_TRANSPARENCY_STATUS: u32 = 3107u32;
pub const WIA_DPS_TRANSPARENCY_STATUS_STR: windows_core::PCWSTR = windows_core::w!("Transparency Adapter Status");
pub const WIA_DPS_TRANSPARENCY_STR: windows_core::PCWSTR = windows_core::w!("Transparency Adapter");
pub const WIA_DPS_USER_NAME: u32 = 3112u32;
pub const WIA_DPS_USER_NAME_STR: windows_core::PCWSTR = windows_core::w!("User Name");
pub const WIA_DPS_VERTICAL_BED_REGISTRATION: u32 = 3080u32;
pub const WIA_DPS_VERTICAL_BED_REGISTRATION_STR: windows_core::PCWSTR = windows_core::w!("Vertical Bed Registration");
pub const WIA_DPS_VERTICAL_BED_SIZE: u32 = 3075u32;
pub const WIA_DPS_VERTICAL_BED_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Vertical Bed Size");
pub const WIA_DPS_VERTICAL_SHEET_FEED_SIZE: u32 = 3077u32;
pub const WIA_DPS_VERTICAL_SHEET_FEED_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Vertical Sheet Feed Size");
pub const WIA_DPV_DSHOW_DEVICE_PATH: u32 = 3588u32;
pub const WIA_DPV_DSHOW_DEVICE_PATH_STR: windows_core::PCWSTR = windows_core::w!("Directshow Device Path");
pub const WIA_DPV_IMAGES_DIRECTORY: u32 = 3587u32;
pub const WIA_DPV_IMAGES_DIRECTORY_STR: windows_core::PCWSTR = windows_core::w!("Images Directory");
pub const WIA_DPV_LAST_PICTURE_TAKEN: u32 = 3586u32;
pub const WIA_DPV_LAST_PICTURE_TAKEN_STR: windows_core::PCWSTR = windows_core::w!("Last Picture Taken");
pub const WIA_ENDORSER_TOK_DATE: windows_core::PCWSTR = windows_core::w!("$DATE$");
pub const WIA_ENDORSER_TOK_DAY: windows_core::PCWSTR = windows_core::w!("$DAY$");
pub const WIA_ENDORSER_TOK_MONTH: windows_core::PCWSTR = windows_core::w!("$MONTH$");
pub const WIA_ENDORSER_TOK_PAGE_COUNT: windows_core::PCWSTR = windows_core::w!("$PAGE_COUNT$");
pub const WIA_ENDORSER_TOK_TIME: windows_core::PCWSTR = windows_core::w!("$TIME$");
pub const WIA_ENDORSER_TOK_YEAR: windows_core::PCWSTR = windows_core::w!("$YEAR$");
pub const WIA_ERROR_BUSY: windows_core::HRESULT = windows_core::HRESULT(0x80210006_u32 as _);
pub const WIA_ERROR_COVER_OPEN: windows_core::HRESULT = windows_core::HRESULT(0x80210010_u32 as _);
pub const WIA_ERROR_DESTINATION: windows_core::HRESULT = windows_core::HRESULT(0x80210012_u32 as _);
pub const WIA_ERROR_DEVICE_COMMUNICATION: windows_core::HRESULT = windows_core::HRESULT(0x8021000A_u32 as _);
pub const WIA_ERROR_DEVICE_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x8021000D_u32 as _);
pub const WIA_ERROR_EXCEPTION_IN_DRIVER: windows_core::HRESULT = windows_core::HRESULT(0x8021000E_u32 as _);
pub const WIA_ERROR_GENERAL_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80210001_u32 as _);
pub const WIA_ERROR_INCORRECT_HARDWARE_SETTING: windows_core::HRESULT = windows_core::HRESULT(0x8021000C_u32 as _);
pub const WIA_ERROR_INVALID_COMMAND: windows_core::HRESULT = windows_core::HRESULT(0x8021000B_u32 as _);
pub const WIA_ERROR_INVALID_DRIVER_RESPONSE: windows_core::HRESULT = windows_core::HRESULT(0x8021000F_u32 as _);
pub const WIA_ERROR_ITEM_DELETED: windows_core::HRESULT = windows_core::HRESULT(0x80210009_u32 as _);
pub const WIA_ERROR_LAMP_OFF: windows_core::HRESULT = windows_core::HRESULT(0x80210011_u32 as _);
pub const WIA_ERROR_MAXIMUM_PRINTER_ENDORSER_COUNTER: windows_core::HRESULT = windows_core::HRESULT(0x80210015_u32 as _);
pub const WIA_ERROR_MULTI_FEED: windows_core::HRESULT = windows_core::HRESULT(0x80210014_u32 as _);
pub const WIA_ERROR_NETWORK_RESERVATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80210013_u32 as _);
pub const WIA_ERROR_OFFLINE: windows_core::HRESULT = windows_core::HRESULT(0x80210005_u32 as _);
pub const WIA_ERROR_PAPER_EMPTY: windows_core::HRESULT = windows_core::HRESULT(0x80210003_u32 as _);
pub const WIA_ERROR_PAPER_JAM: windows_core::HRESULT = windows_core::HRESULT(0x80210002_u32 as _);
pub const WIA_ERROR_PAPER_PROBLEM: windows_core::HRESULT = windows_core::HRESULT(0x80210004_u32 as _);
pub const WIA_ERROR_USER_INTERVENTION: windows_core::HRESULT = windows_core::HRESULT(0x80210008_u32 as _);
pub const WIA_ERROR_WARMING_UP: windows_core::HRESULT = windows_core::HRESULT(0x80210007_u32 as _);
pub const WIA_EVENT_CANCEL_IO: windows_core::GUID = windows_core::GUID::from_u128(0xc860f7b8_9ccd_41ea_bbbf_4dd09c5b1795);
pub const WIA_EVENT_COVER_CLOSED: windows_core::GUID = windows_core::GUID::from_u128(0x6714a1e6_e285_468c_9b8c_da7dc4cbaa05);
pub const WIA_EVENT_COVER_OPEN: windows_core::GUID = windows_core::GUID::from_u128(0x19a12136_fa1c_4f66_900f_8f914ec74ec9);
pub const WIA_EVENT_DEVICE_CONNECTED: windows_core::GUID = windows_core::GUID::from_u128(0xa28bbade_64b6_11d2_a231_00c04fa31809);
pub const WIA_EVENT_DEVICE_CONNECTED_STR: windows_core::PCWSTR = windows_core::w!("Device Connected");
pub const WIA_EVENT_DEVICE_DISCONNECTED: windows_core::GUID = windows_core::GUID::from_u128(0x143e4e83_6497_11d2_a231_00c04fa31809);
pub const WIA_EVENT_DEVICE_DISCONNECTED_STR: windows_core::PCWSTR = windows_core::w!("Device Disconnected");
pub const WIA_EVENT_DEVICE_NOT_READY: windows_core::GUID = windows_core::GUID::from_u128(0xd8962d7e_e4dc_4b4d_ba29_668a87f42e6f);
pub const WIA_EVENT_DEVICE_READY: windows_core::GUID = windows_core::GUID::from_u128(0x7523ec6c_988b_419e_9a0a_425ac31b37dc);
pub const WIA_EVENT_FEEDER_EMPTIED: windows_core::GUID = windows_core::GUID::from_u128(0xe70b4b82_6dda_46bb_8ff9_53ceb1a03e35);
pub const WIA_EVENT_FEEDER_LOADED: windows_core::GUID = windows_core::GUID::from_u128(0xcc8d701e_9aba_481d_bf74_78f763dc342a);
pub const WIA_EVENT_FLATBED_LID_CLOSED: windows_core::GUID = windows_core::GUID::from_u128(0xf879af0f_9b29_4283_ad95_d412164d39a9);
pub const WIA_EVENT_FLATBED_LID_OPEN: windows_core::GUID = windows_core::GUID::from_u128(0xba0a0623_437d_4f03_a97d_7793b123113c);
pub const WIA_EVENT_HANDLER_NO_ACTION: windows_core::GUID = windows_core::GUID::from_u128(0xe0372b7d_e115_4525_bc55_b629e68c745a);
pub const WIA_EVENT_HANDLER_PROMPT: windows_core::GUID = windows_core::GUID::from_u128(0x5f4baad0_4d59_4fcd_b213_783ce7a92f22);
pub const WIA_EVENT_ITEM_CREATED: windows_core::GUID = windows_core::GUID::from_u128(0x4c8f4ef5_e14f_11d2_b326_00c04f68ce61);
pub const WIA_EVENT_ITEM_DELETED: windows_core::GUID = windows_core::GUID::from_u128(0x1d22a559_e14f_11d2_b326_00c04f68ce61);
pub const WIA_EVENT_POWER_RESUME: windows_core::GUID = windows_core::GUID::from_u128(0x618f153e_f686_4350_9634_4115a304830c);
pub const WIA_EVENT_POWER_SUSPEND: windows_core::GUID = windows_core::GUID::from_u128(0xa0922ff9_c3b4_411c_9e29_03a66993d2be);
pub const WIA_EVENT_SCAN_EMAIL_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0xc686dcee_54f2_419e_9a27_2fc7f2e98f9e);
pub const WIA_EVENT_SCAN_FAX_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0xc00eb793_8c6e_11d2_977a_0000f87a926f);
pub const WIA_EVENT_SCAN_FILM_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0x9b2b662c_6185_438c_b68b_e39ee25e71cb);
pub const WIA_EVENT_SCAN_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0xa6c5a715_8c6e_11d2_977a_0000f87a926f);
pub const WIA_EVENT_SCAN_IMAGE2: windows_core::GUID = windows_core::GUID::from_u128(0xfc4767c1_c8b3_48a2_9cfa_2e90cb3d3590);
pub const WIA_EVENT_SCAN_IMAGE3: windows_core::GUID = windows_core::GUID::from_u128(0x154e27be_b617_4653_acc5_0fd7bd4c65ce);
pub const WIA_EVENT_SCAN_IMAGE4: windows_core::GUID = windows_core::GUID::from_u128(0xa65b704a_7f3c_4447_a75d_8a26dfca1fdf);
pub const WIA_EVENT_SCAN_OCR_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0x9d095b89_37d6_4877_afed_62a297dc6dbe);
pub const WIA_EVENT_SCAN_PRINT_IMAGE: windows_core::GUID = windows_core::GUID::from_u128(0xb441f425_8c6e_11d2_977a_0000f87a926f);
pub const WIA_EVENT_STI_PROXY: windows_core::GUID = windows_core::GUID::from_u128(0xd711f81f_1f0d_422d_8641_927d1b93e5e5);
pub const WIA_EVENT_STORAGE_CREATED: windows_core::GUID = windows_core::GUID::from_u128(0x353308b2_fe73_46c8_895e_fa4551ccc85a);
pub const WIA_EVENT_STORAGE_DELETED: windows_core::GUID = windows_core::GUID::from_u128(0x5e41e75e_9390_44c5_9a51_e47019e390cf);
pub const WIA_EVENT_TREE_UPDATED: windows_core::GUID = windows_core::GUID::from_u128(0xc9859b91_4ab2_4cd6_a1fc_582eec55e585);
pub const WIA_EVENT_VOLUME_INSERT: windows_core::GUID = windows_core::GUID::from_u128(0x9638bbfd_d1bd_11d2_b31f_00c04f68ce61);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIA_EXTENDED_TRANSFER_INFO {
    pub ulSize: u32,
    pub ulMinBufferSize: u32,
    pub ulOptimalBufferSize: u32,
    pub ulMaxBufferSize: u32,
    pub ulNumBuffers: u32,
}
pub const WIA_FEEDER_CONTROL_AUTO: u32 = 0u32;
pub const WIA_FEEDER_CONTROL_MANUAL: u32 = 1u32;
pub const WIA_FILM_BW_NEGATIVE: u32 = 2u32;
pub const WIA_FILM_COLOR_NEGATIVE: u32 = 1u32;
pub const WIA_FILM_COLOR_SLIDE: u32 = 0u32;
pub const WIA_FINAL_SCAN: u32 = 0u32;
pub const WIA_FLAG_NOM: u32 = 0u32;
pub const WIA_FLAG_NUM_ELEMS: u32 = 2u32;
pub const WIA_FLAG_VALUES: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIA_FORMAT_INFO {
    pub guidFormatID: windows_core::GUID,
    pub lTymed: i32,
}
pub const WIA_IMAGEPROC_FILTER_STR: windows_core::PCWSTR = windows_core::w!("ImageProcessingFilter");
pub const WIA_INTENT_BEST_PREVIEW: u32 = 262144u32;
pub const WIA_INTENT_IMAGE_TYPE_COLOR: u32 = 1u32;
pub const WIA_INTENT_IMAGE_TYPE_GRAYSCALE: u32 = 2u32;
pub const WIA_INTENT_IMAGE_TYPE_MASK: u32 = 15u32;
pub const WIA_INTENT_IMAGE_TYPE_TEXT: u32 = 4u32;
pub const WIA_INTENT_MAXIMIZE_QUALITY: u32 = 131072u32;
pub const WIA_INTENT_MINIMIZE_SIZE: u32 = 65536u32;
pub const WIA_INTENT_NONE: u32 = 0u32;
pub const WIA_INTENT_SIZE_MASK: u32 = 983040u32;
pub const WIA_IPA_ACCESS_RIGHTS: u32 = 4102u32;
pub const WIA_IPA_ACCESS_RIGHTS_STR: windows_core::PCWSTR = windows_core::w!("Access Rights");
pub const WIA_IPA_APP_COLOR_MAPPING: u32 = 4121u32;
pub const WIA_IPA_APP_COLOR_MAPPING_STR: windows_core::PCWSTR = windows_core::w!("Application Applies Color Mapping");
pub const WIA_IPA_BITS_PER_CHANNEL: u32 = 4110u32;
pub const WIA_IPA_BITS_PER_CHANNEL_STR: windows_core::PCWSTR = windows_core::w!("Bits Per Channel");
pub const WIA_IPA_BUFFER_SIZE: u32 = 4118u32;
pub const WIA_IPA_BUFFER_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Buffer Size");
pub const WIA_IPA_BYTES_PER_LINE: u32 = 4113u32;
pub const WIA_IPA_BYTES_PER_LINE_STR: windows_core::PCWSTR = windows_core::w!("Bytes Per Line");
pub const WIA_IPA_CHANNELS_PER_PIXEL: u32 = 4109u32;
pub const WIA_IPA_CHANNELS_PER_PIXEL_STR: windows_core::PCWSTR = windows_core::w!("Channels Per Pixel");
pub const WIA_IPA_COLOR_PROFILE: u32 = 4117u32;
pub const WIA_IPA_COLOR_PROFILE_STR: windows_core::PCWSTR = windows_core::w!("Color Profiles");
pub const WIA_IPA_COMPRESSION: u32 = 4107u32;
pub const WIA_IPA_COMPRESSION_STR: windows_core::PCWSTR = windows_core::w!("Compression");
pub const WIA_IPA_DATATYPE: u32 = 4103u32;
pub const WIA_IPA_DATATYPE_STR: windows_core::PCWSTR = windows_core::w!("Data Type");
pub const WIA_IPA_DEPTH: u32 = 4104u32;
pub const WIA_IPA_DEPTH_STR: windows_core::PCWSTR = windows_core::w!("Bits Per Pixel");
pub const WIA_IPA_FILENAME_EXTENSION: u32 = 4123u32;
pub const WIA_IPA_FILENAME_EXTENSION_STR: windows_core::PCWSTR = windows_core::w!("Filename extension");
pub const WIA_IPA_FIRST: u32 = 4098u32;
pub const WIA_IPA_FORMAT: u32 = 4106u32;
pub const WIA_IPA_FORMAT_STR: windows_core::PCWSTR = windows_core::w!("Format");
pub const WIA_IPA_FULL_ITEM_NAME: u32 = 4099u32;
pub const WIA_IPA_FULL_ITEM_NAME_STR: windows_core::PCWSTR = windows_core::w!("Full Item Name");
pub const WIA_IPA_GAMMA_CURVES: u32 = 4115u32;
pub const WIA_IPA_GAMMA_CURVES_STR: windows_core::PCWSTR = windows_core::w!("Gamma Curves");
pub const WIA_IPA_ICM_PROFILE_NAME: u32 = 4120u32;
pub const WIA_IPA_ICM_PROFILE_NAME_STR: windows_core::PCWSTR = windows_core::w!("Color Profile Name");
pub const WIA_IPA_ITEMS_STORED: u32 = 4127u32;
pub const WIA_IPA_ITEMS_STORED_STR: windows_core::PCWSTR = windows_core::w!("Items Stored");
pub const WIA_IPA_ITEM_CATEGORY: u32 = 4125u32;
pub const WIA_IPA_ITEM_CATEGORY_STR: windows_core::PCWSTR = windows_core::w!("Item Category");
pub const WIA_IPA_ITEM_FLAGS: u32 = 4101u32;
pub const WIA_IPA_ITEM_FLAGS_STR: windows_core::PCWSTR = windows_core::w!("Item Flags");
pub const WIA_IPA_ITEM_NAME: u32 = 4098u32;
pub const WIA_IPA_ITEM_NAME_STR: windows_core::PCWSTR = windows_core::w!("Item Name");
pub const WIA_IPA_ITEM_SIZE: u32 = 4116u32;
pub const WIA_IPA_ITEM_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Item Size");
pub const WIA_IPA_ITEM_TIME: u32 = 4100u32;
pub const WIA_IPA_ITEM_TIME_STR: windows_core::PCWSTR = windows_core::w!("Item Time Stamp");
pub const WIA_IPA_MIN_BUFFER_SIZE: u32 = 4118u32;
pub const WIA_IPA_MIN_BUFFER_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Buffer Size");
pub const WIA_IPA_NUMBER_OF_LINES: u32 = 4114u32;
pub const WIA_IPA_NUMBER_OF_LINES_STR: windows_core::PCWSTR = windows_core::w!("Number of Lines");
pub const WIA_IPA_PIXELS_PER_LINE: u32 = 4112u32;
pub const WIA_IPA_PIXELS_PER_LINE_STR: windows_core::PCWSTR = windows_core::w!("Pixels Per Line");
pub const WIA_IPA_PLANAR: u32 = 4111u32;
pub const WIA_IPA_PLANAR_STR: windows_core::PCWSTR = windows_core::w!("Planar");
pub const WIA_IPA_PREFERRED_FORMAT: u32 = 4105u32;
pub const WIA_IPA_PREFERRED_FORMAT_STR: windows_core::PCWSTR = windows_core::w!("Preferred Format");
pub const WIA_IPA_PROP_STREAM_COMPAT_ID: u32 = 4122u32;
pub const WIA_IPA_PROP_STREAM_COMPAT_ID_STR: windows_core::PCWSTR = windows_core::w!("Stream Compatibility ID");
pub const WIA_IPA_RAW_BITS_PER_CHANNEL: u32 = 4128u32;
pub const WIA_IPA_RAW_BITS_PER_CHANNEL_STR: windows_core::PCWSTR = windows_core::w!("Raw Bits Per Channel");
pub const WIA_IPA_REGION_TYPE: u32 = 4119u32;
pub const WIA_IPA_REGION_TYPE_STR: windows_core::PCWSTR = windows_core::w!("Region Type");
pub const WIA_IPA_SUPPRESS_PROPERTY_PAGE: u32 = 4124u32;
pub const WIA_IPA_SUPPRESS_PROPERTY_PAGE_STR: windows_core::PCWSTR = windows_core::w!("Suppress a property page");
pub const WIA_IPA_TYMED: u32 = 4108u32;
pub const WIA_IPA_TYMED_STR: windows_core::PCWSTR = windows_core::w!("Media Type");
pub const WIA_IPA_UPLOAD_ITEM_SIZE: u32 = 4126u32;
pub const WIA_IPA_UPLOAD_ITEM_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Upload Item Size");
pub const WIA_IPC_AUDIO_AVAILABLE: u32 = 5125u32;
pub const WIA_IPC_AUDIO_AVAILABLE_STR: windows_core::PCWSTR = windows_core::w!("Audio Available");
pub const WIA_IPC_AUDIO_DATA: u32 = 5127u32;
pub const WIA_IPC_AUDIO_DATA_FORMAT: u32 = 5126u32;
pub const WIA_IPC_AUDIO_DATA_FORMAT_STR: windows_core::PCWSTR = windows_core::w!("Audio Format");
pub const WIA_IPC_AUDIO_DATA_STR: windows_core::PCWSTR = windows_core::w!("Audio Data");
pub const WIA_IPC_FIRST: u32 = 5122u32;
pub const WIA_IPC_NUM_PICT_PER_ROW: u32 = 5128u32;
pub const WIA_IPC_NUM_PICT_PER_ROW_STR: windows_core::PCWSTR = windows_core::w!("Pictures per Row");
pub const WIA_IPC_SEQUENCE: u32 = 5129u32;
pub const WIA_IPC_SEQUENCE_STR: windows_core::PCWSTR = windows_core::w!("Sequence Number");
pub const WIA_IPC_THUMBNAIL: u32 = 5122u32;
pub const WIA_IPC_THUMBNAIL_STR: windows_core::PCWSTR = windows_core::w!("Thumbnail Data");
pub const WIA_IPC_THUMB_HEIGHT: u32 = 5124u32;
pub const WIA_IPC_THUMB_HEIGHT_STR: windows_core::PCWSTR = windows_core::w!("Thumbnail Height");
pub const WIA_IPC_THUMB_WIDTH: u32 = 5123u32;
pub const WIA_IPC_THUMB_WIDTH_STR: windows_core::PCWSTR = windows_core::w!("Thumbnail Width");
pub const WIA_IPC_TIMEDELAY: u32 = 5130u32;
pub const WIA_IPC_TIMEDELAY_STR: windows_core::PCWSTR = windows_core::w!("Time Delay");
pub const WIA_IPS_ALARM: u32 = 4185u32;
pub const WIA_IPS_ALARM_STR: windows_core::PCWSTR = windows_core::w!("Alarm");
pub const WIA_IPS_AUTO_CROP: u32 = 4170u32;
pub const WIA_IPS_AUTO_CROP_STR: windows_core::PCWSTR = windows_core::w!("Auto-Crop");
pub const WIA_IPS_AUTO_DESKEW: u32 = 3107u32;
pub const WIA_IPS_AUTO_DESKEW_STR: windows_core::PCWSTR = windows_core::w!("Automatic Deskew");
pub const WIA_IPS_BARCODE_READER: u32 = 4150u32;
pub const WIA_IPS_BARCODE_READER_STR: windows_core::PCWSTR = windows_core::w!("Barcode Reader");
pub const WIA_IPS_BARCODE_SEARCH_DIRECTION: u32 = 4152u32;
pub const WIA_IPS_BARCODE_SEARCH_DIRECTION_STR: windows_core::PCWSTR = windows_core::w!("Barcode Search Direction");
pub const WIA_IPS_BARCODE_SEARCH_TIMEOUT: u32 = 4154u32;
pub const WIA_IPS_BARCODE_SEARCH_TIMEOUT_STR: windows_core::PCWSTR = windows_core::w!("Barcode Search Timeout");
pub const WIA_IPS_BLANK_PAGES: u32 = 4167u32;
pub const WIA_IPS_BLANK_PAGES_SENSITIVITY: u32 = 4192u32;
pub const WIA_IPS_BLANK_PAGES_SENSITIVITY_STR: windows_core::PCWSTR = windows_core::w!("Blank Pages Sensitivity");
pub const WIA_IPS_BLANK_PAGES_STR: windows_core::PCWSTR = windows_core::w!("Blank Pages");
pub const WIA_IPS_BRIGHTNESS: u32 = 6154u32;
pub const WIA_IPS_BRIGHTNESS_STR: windows_core::PCWSTR = windows_core::w!("Brightness");
pub const WIA_IPS_COLOR_DROP: u32 = 4176u32;
pub const WIA_IPS_COLOR_DROP_BLUE: u32 = 4179u32;
pub const WIA_IPS_COLOR_DROP_BLUE_STR: windows_core::PCWSTR = windows_core::w!("Color Drop Blue");
pub const WIA_IPS_COLOR_DROP_GREEN: u32 = 4178u32;
pub const WIA_IPS_COLOR_DROP_GREEN_STR: windows_core::PCWSTR = windows_core::w!("Color Drop Green");
pub const WIA_IPS_COLOR_DROP_MULTI: u32 = 4191u32;
pub const WIA_IPS_COLOR_DROP_MULTI_STR: windows_core::PCWSTR = windows_core::w!("Color Drop Multiple");
pub const WIA_IPS_COLOR_DROP_RED: u32 = 4177u32;
pub const WIA_IPS_COLOR_DROP_RED_STR: windows_core::PCWSTR = windows_core::w!("Color Drop Red");
pub const WIA_IPS_COLOR_DROP_STR: windows_core::PCWSTR = windows_core::w!("Color Drop");
pub const WIA_IPS_CONTRAST: u32 = 6155u32;
pub const WIA_IPS_CONTRAST_STR: windows_core::PCWSTR = windows_core::w!("Contrast");
pub const WIA_IPS_CUR_INTENT: u32 = 6146u32;
pub const WIA_IPS_CUR_INTENT_STR: windows_core::PCWSTR = windows_core::w!("Current Intent");
pub const WIA_IPS_DESKEW_X: u32 = 6162u32;
pub const WIA_IPS_DESKEW_X_STR: windows_core::PCWSTR = windows_core::w!("DeskewX");
pub const WIA_IPS_DESKEW_Y: u32 = 6163u32;
pub const WIA_IPS_DESKEW_Y_STR: windows_core::PCWSTR = windows_core::w!("DeskewY");
pub const WIA_IPS_DOCUMENT_HANDLING_SELECT: u32 = 3088u32;
pub const WIA_IPS_DOCUMENT_HANDLING_SELECT_STR: windows_core::PCWSTR = windows_core::w!("Document Handling Select");
pub const WIA_IPS_ENABLED_BARCODE_TYPES: u32 = 4156u32;
pub const WIA_IPS_ENABLED_BARCODE_TYPES_STR: windows_core::PCWSTR = windows_core::w!("Enabled Barcode Types");
pub const WIA_IPS_ENABLED_PATCH_CODE_TYPES: u32 = 4163u32;
pub const WIA_IPS_ENABLED_PATCH_CODE_TYPES_STR: windows_core::PCWSTR = windows_core::w!("Enabled Path Code Types");
pub const WIA_IPS_FEEDER_CONTROL: u32 = 4182u32;
pub const WIA_IPS_FEEDER_CONTROL_STR: windows_core::PCWSTR = windows_core::w!("Feeder Control");
pub const WIA_IPS_FILM_NODE_NAME: u32 = 4129u32;
pub const WIA_IPS_FILM_NODE_NAME_STR: windows_core::PCWSTR = windows_core::w!("Film Node Name");
pub const WIA_IPS_FILM_SCAN_MODE: u32 = 3104u32;
pub const WIA_IPS_FILM_SCAN_MODE_STR: windows_core::PCWSTR = windows_core::w!("Film Scan Mode");
pub const WIA_IPS_FIRST: u32 = 6146u32;
pub const WIA_IPS_INVERT: u32 = 6160u32;
pub const WIA_IPS_INVERT_STR: windows_core::PCWSTR = windows_core::w!("Invert");
pub const WIA_IPS_JOB_SEPARATORS: u32 = 4165u32;
pub const WIA_IPS_JOB_SEPARATORS_STR: windows_core::PCWSTR = windows_core::w!("Job Separators");
pub const WIA_IPS_LAMP: u32 = 3105u32;
pub const WIA_IPS_LAMP_AUTO_OFF: u32 = 3106u32;
pub const WIA_IPS_LAMP_AUTO_OFF_STR: windows_core::PCWSTR = windows_core::w!("Lamp Auto Off");
pub const WIA_IPS_LAMP_STR: windows_core::PCWSTR = windows_core::w!("Lamp");
pub const WIA_IPS_LONG_DOCUMENT: u32 = 4166u32;
pub const WIA_IPS_LONG_DOCUMENT_STR: windows_core::PCWSTR = windows_core::w!("Long Document");
pub const WIA_IPS_MAXIMUM_BARCODES_PER_PAGE: u32 = 4151u32;
pub const WIA_IPS_MAXIMUM_BARCODES_PER_PAGE_STR: windows_core::PCWSTR = windows_core::w!("Maximum Barcodes Per Page");
pub const WIA_IPS_MAXIMUM_BARCODE_SEARCH_RETRIES: u32 = 4153u32;
pub const WIA_IPS_MAXIMUM_BARCODE_SEARCH_RETRIES_STR: windows_core::PCWSTR = windows_core::w!("Barcode Search Retries");
pub const WIA_IPS_MAX_HORIZONTAL_SIZE: u32 = 6165u32;
pub const WIA_IPS_MAX_HORIZONTAL_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Maximum Horizontal Scan Size");
pub const WIA_IPS_MAX_VERTICAL_SIZE: u32 = 6166u32;
pub const WIA_IPS_MAX_VERTICAL_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Maximum Vertical Scan Size");
pub const WIA_IPS_MICR_READER: u32 = 4164u32;
pub const WIA_IPS_MICR_READER_STR: windows_core::PCWSTR = windows_core::w!("MICR Reader");
pub const WIA_IPS_MIN_HORIZONTAL_SIZE: u32 = 6167u32;
pub const WIA_IPS_MIN_HORIZONTAL_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Minimum Horizontal Scan Size");
pub const WIA_IPS_MIN_VERTICAL_SIZE: u32 = 6168u32;
pub const WIA_IPS_MIN_VERTICAL_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Minimum Vertical Scan Size");
pub const WIA_IPS_MIRROR: u32 = 6158u32;
pub const WIA_IPS_MIRROR_STR: windows_core::PCWSTR = windows_core::w!("Mirror");
pub const WIA_IPS_MULTI_FEED: u32 = 4168u32;
pub const WIA_IPS_MULTI_FEED_DETECT_METHOD: u32 = 4193u32;
pub const WIA_IPS_MULTI_FEED_DETECT_METHOD_STR: windows_core::PCWSTR = windows_core::w!("Multi-Feed Detection Method");
pub const WIA_IPS_MULTI_FEED_SENSITIVITY: u32 = 4169u32;
pub const WIA_IPS_MULTI_FEED_SENSITIVITY_STR: windows_core::PCWSTR = windows_core::w!("Multi-Feed Sensitivity");
pub const WIA_IPS_MULTI_FEED_STR: windows_core::PCWSTR = windows_core::w!("Multi-Feed");
pub const WIA_IPS_OPTICAL_XRES: u32 = 3090u32;
pub const WIA_IPS_OPTICAL_XRES_STR: windows_core::PCWSTR = windows_core::w!("Horizontal Optical Resolution");
pub const WIA_IPS_OPTICAL_YRES: u32 = 3091u32;
pub const WIA_IPS_OPTICAL_YRES_STR: windows_core::PCWSTR = windows_core::w!("Vertical Optical Resolution");
pub const WIA_IPS_ORIENTATION: u32 = 6156u32;
pub const WIA_IPS_ORIENTATION_STR: windows_core::PCWSTR = windows_core::w!("Orientation");
pub const WIA_IPS_OVER_SCAN: u32 = 4171u32;
pub const WIA_IPS_OVER_SCAN_BOTTOM: u32 = 4175u32;
pub const WIA_IPS_OVER_SCAN_BOTTOM_STR: windows_core::PCWSTR = windows_core::w!("Overscan Bottom");
pub const WIA_IPS_OVER_SCAN_LEFT: u32 = 4172u32;
pub const WIA_IPS_OVER_SCAN_LEFT_STR: windows_core::PCWSTR = windows_core::w!("Overscan Left");
pub const WIA_IPS_OVER_SCAN_RIGHT: u32 = 4173u32;
pub const WIA_IPS_OVER_SCAN_RIGHT_STR: windows_core::PCWSTR = windows_core::w!("Overscan Right");
pub const WIA_IPS_OVER_SCAN_STR: windows_core::PCWSTR = windows_core::w!("Overscan");
pub const WIA_IPS_OVER_SCAN_TOP: u32 = 4174u32;
pub const WIA_IPS_OVER_SCAN_TOP_STR: windows_core::PCWSTR = windows_core::w!("Overscan Top");
pub const WIA_IPS_PAGES: u32 = 3096u32;
pub const WIA_IPS_PAGES_STR: windows_core::PCWSTR = windows_core::w!("Pages");
pub const WIA_IPS_PAGE_HEIGHT: u32 = 3099u32;
pub const WIA_IPS_PAGE_HEIGHT_STR: windows_core::PCWSTR = windows_core::w!("Page Height");
pub const WIA_IPS_PAGE_SIZE: u32 = 3097u32;
pub const WIA_IPS_PAGE_SIZE_STR: windows_core::PCWSTR = windows_core::w!("Page Size");
pub const WIA_IPS_PAGE_WIDTH: u32 = 3098u32;
pub const WIA_IPS_PAGE_WIDTH_STR: windows_core::PCWSTR = windows_core::w!("Page Width");
pub const WIA_IPS_PATCH_CODE_READER: u32 = 4157u32;
pub const WIA_IPS_PATCH_CODE_READER_STR: windows_core::PCWSTR = windows_core::w!("Patch Code Reader");
pub const WIA_IPS_PHOTOMETRIC_INTERP: u32 = 6153u32;
pub const WIA_IPS_PHOTOMETRIC_INTERP_STR: windows_core::PCWSTR = windows_core::w!("Photometric Interpretation");
pub const WIA_IPS_PREVIEW: u32 = 3100u32;
pub const WIA_IPS_PREVIEW_STR: windows_core::PCWSTR = windows_core::w!("Preview");
pub const WIA_IPS_PREVIEW_TYPE: u32 = 3111u32;
pub const WIA_IPS_PREVIEW_TYPE_STR: windows_core::PCWSTR = windows_core::w!("Preview Type");
pub const WIA_IPS_PRINTER_ENDORSER: u32 = 4130u32;
pub const WIA_IPS_PRINTER_ENDORSER_CHARACTER_ROTATION: u32 = 4187u32;
pub const WIA_IPS_PRINTER_ENDORSER_CHARACTER_ROTATION_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Character Rotation");
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER: u32 = 4132u32;
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER_DIGITS: u32 = 4190u32;
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER_DIGITS_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Counter Digits");
pub const WIA_IPS_PRINTER_ENDORSER_COUNTER_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Counter");
pub const WIA_IPS_PRINTER_ENDORSER_FONT_TYPE: u32 = 4184u32;
pub const WIA_IPS_PRINTER_ENDORSER_FONT_TYPE_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Font Type");
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS: u32 = 4142u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_DOWNLOAD: u32 = 4149u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_DOWNLOAD_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Graphics Download");
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_HEIGHT: u32 = 4147u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_HEIGHT_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Graphics Maximum Height");
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_WIDTH: u32 = 4145u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MAX_WIDTH_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Graphics Maximum Width");
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_HEIGHT: u32 = 4146u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_HEIGHT_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Graphics Minimum Height");
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_WIDTH: u32 = 4144u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_MIN_WIDTH_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Graphics Minimum Width");
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_POSITION: u32 = 4143u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_POSITION_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Graphics Position");
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Graphics");
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_UPLOAD: u32 = 4148u32;
pub const WIA_IPS_PRINTER_ENDORSER_GRAPHICS_UPLOAD_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Graphics Upload");
pub const WIA_IPS_PRINTER_ENDORSER_INK: u32 = 4186u32;
pub const WIA_IPS_PRINTER_ENDORSER_INK_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Ink");
pub const WIA_IPS_PRINTER_ENDORSER_MAX_CHARACTERS: u32 = 4188u32;
pub const WIA_IPS_PRINTER_ENDORSER_MAX_CHARACTERS_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Maximum Characters");
pub const WIA_IPS_PRINTER_ENDORSER_MAX_GRAPHICS: u32 = 4189u32;
pub const WIA_IPS_PRINTER_ENDORSER_MAX_GRAPHICS_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Maximum Graphics");
pub const WIA_IPS_PRINTER_ENDORSER_NUM_LINES: u32 = 4136u32;
pub const WIA_IPS_PRINTER_ENDORSER_NUM_LINES_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Lines");
pub const WIA_IPS_PRINTER_ENDORSER_ORDER: u32 = 4131u32;
pub const WIA_IPS_PRINTER_ENDORSER_ORDER_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Order");
pub const WIA_IPS_PRINTER_ENDORSER_PADDING: u32 = 4183u32;
pub const WIA_IPS_PRINTER_ENDORSER_PADDING_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Padding");
pub const WIA_IPS_PRINTER_ENDORSER_STEP: u32 = 4133u32;
pub const WIA_IPS_PRINTER_ENDORSER_STEP_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Step");
pub const WIA_IPS_PRINTER_ENDORSER_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser");
pub const WIA_IPS_PRINTER_ENDORSER_STRING: u32 = 4137u32;
pub const WIA_IPS_PRINTER_ENDORSER_STRING_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser String");
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_DOWNLOAD: u32 = 4141u32;
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_DOWNLOAD_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Text Download");
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_UPLOAD: u32 = 4140u32;
pub const WIA_IPS_PRINTER_ENDORSER_TEXT_UPLOAD_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Text Upload");
pub const WIA_IPS_PRINTER_ENDORSER_VALID_CHARACTERS: u32 = 4138u32;
pub const WIA_IPS_PRINTER_ENDORSER_VALID_CHARACTERS_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Valid Characters");
pub const WIA_IPS_PRINTER_ENDORSER_VALID_FORMAT_SPECIFIERS: u32 = 4139u32;
pub const WIA_IPS_PRINTER_ENDORSER_VALID_FORMAT_SPECIFIERS_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Valid Format Specifiers");
pub const WIA_IPS_PRINTER_ENDORSER_XOFFSET: u32 = 4134u32;
pub const WIA_IPS_PRINTER_ENDORSER_XOFFSET_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Horizontal Offset");
pub const WIA_IPS_PRINTER_ENDORSER_YOFFSET: u32 = 4135u32;
pub const WIA_IPS_PRINTER_ENDORSER_YOFFSET_STR: windows_core::PCWSTR = windows_core::w!("Printer/Endorser Vertical Offset");
pub const WIA_IPS_ROTATION: u32 = 6157u32;
pub const WIA_IPS_ROTATION_STR: windows_core::PCWSTR = windows_core::w!("Rotation");
pub const WIA_IPS_SCAN_AHEAD: u32 = 4180u32;
pub const WIA_IPS_SCAN_AHEAD_CAPACITY: u32 = 4181u32;
pub const WIA_IPS_SCAN_AHEAD_CAPACITY_STR: windows_core::PCWSTR = windows_core::w!("Scan Ahead Capacity");
pub const WIA_IPS_SCAN_AHEAD_STR: windows_core::PCWSTR = windows_core::w!("Scan Ahead");
pub const WIA_IPS_SEGMENTATION: u32 = 6164u32;
pub const WIA_IPS_SEGMENTATION_STR: windows_core::PCWSTR = windows_core::w!("Segmentation");
pub const WIA_IPS_SHEET_FEEDER_REGISTRATION: u32 = 3078u32;
pub const WIA_IPS_SHEET_FEEDER_REGISTRATION_STR: windows_core::PCWSTR = windows_core::w!("Sheet Feeder Registration");
pub const WIA_IPS_SHOW_PREVIEW_CONTROL: u32 = 3103u32;
pub const WIA_IPS_SHOW_PREVIEW_CONTROL_STR: windows_core::PCWSTR = windows_core::w!("Show preview control");
pub const WIA_IPS_SUPPORTED_BARCODE_TYPES: u32 = 4155u32;
pub const WIA_IPS_SUPPORTED_BARCODE_TYPES_STR: windows_core::PCWSTR = windows_core::w!("Supported Barcode Types");
pub const WIA_IPS_SUPPORTED_PATCH_CODE_TYPES: u32 = 4162u32;
pub const WIA_IPS_SUPPORTED_PATCH_CODE_TYPES_STR: windows_core::PCWSTR = windows_core::w!("Supported Patch Code Types");
pub const WIA_IPS_SUPPORTS_CHILD_ITEM_CREATION: u32 = 3108u32;
pub const WIA_IPS_SUPPORTS_CHILD_ITEM_CREATION_STR: windows_core::PCWSTR = windows_core::w!("Supports Child Item Creation");
pub const WIA_IPS_THRESHOLD: u32 = 6159u32;
pub const WIA_IPS_THRESHOLD_STR: windows_core::PCWSTR = windows_core::w!("Threshold");
pub const WIA_IPS_TRANSFER_CAPABILITIES: u32 = 6169u32;
pub const WIA_IPS_TRANSFER_CAPABILITIES_STR: windows_core::PCWSTR = windows_core::w!("Transfer Capabilities");
pub const WIA_IPS_WARM_UP_TIME: u32 = 6161u32;
pub const WIA_IPS_WARM_UP_TIME_STR: windows_core::PCWSTR = windows_core::w!("Lamp Warm up Time");
pub const WIA_IPS_XEXTENT: u32 = 6151u32;
pub const WIA_IPS_XEXTENT_STR: windows_core::PCWSTR = windows_core::w!("Horizontal Extent");
pub const WIA_IPS_XPOS: u32 = 6149u32;
pub const WIA_IPS_XPOS_STR: windows_core::PCWSTR = windows_core::w!("Horizontal Start Position");
pub const WIA_IPS_XRES: u32 = 6147u32;
pub const WIA_IPS_XRES_STR: windows_core::PCWSTR = windows_core::w!("Horizontal Resolution");
pub const WIA_IPS_XSCALING: u32 = 3109u32;
pub const WIA_IPS_XSCALING_STR: windows_core::PCWSTR = windows_core::w!("Horizontal Scaling");
pub const WIA_IPS_YEXTENT: u32 = 6152u32;
pub const WIA_IPS_YEXTENT_STR: windows_core::PCWSTR = windows_core::w!("Vertical Extent");
pub const WIA_IPS_YPOS: u32 = 6150u32;
pub const WIA_IPS_YPOS_STR: windows_core::PCWSTR = windows_core::w!("Vertical Start Position");
pub const WIA_IPS_YRES: u32 = 6148u32;
pub const WIA_IPS_YRES_STR: windows_core::PCWSTR = windows_core::w!("Vertical Resolution");
pub const WIA_IPS_YSCALING: u32 = 3110u32;
pub const WIA_IPS_YSCALING_STR: windows_core::PCWSTR = windows_core::w!("Vertical Scaling");
pub const WIA_IS_DEFAULT_HANDLER: u32 = 1u32;
pub const WIA_ITEM_CAN_BE_DELETED: u32 = 128u32;
pub const WIA_ITEM_READ: u32 = 1u32;
pub const WIA_ITEM_WRITE: u32 = 2u32;
pub const WIA_LAMP_OFF: u32 = 1u32;
pub const WIA_LAMP_ON: u32 = 0u32;
pub const WIA_LINE_ORDER_BOTTOM_TO_TOP: u32 = 2u32;
pub const WIA_LINE_ORDER_TOP_TO_BOTTOM: u32 = 1u32;
pub const WIA_LIST_COUNT: u32 = 0u32;
pub const WIA_LIST_NOM: u32 = 1u32;
pub const WIA_LIST_NUM_ELEMS: u32 = 2u32;
pub const WIA_LIST_VALUES: u32 = 2u32;
pub const WIA_LONG_DOCUMENT_DISABLED: u32 = 0u32;
pub const WIA_LONG_DOCUMENT_ENABLED: u32 = 1u32;
pub const WIA_LONG_DOCUMENT_SPLIT: u32 = 2u32;
pub const WIA_MAJOR_EVENT_DEVICE_CONNECT: u32 = 1u32;
pub const WIA_MAJOR_EVENT_DEVICE_DISCONNECT: u32 = 2u32;
pub const WIA_MAJOR_EVENT_PICTURE_DELETED: u32 = 4u32;
pub const WIA_MAJOR_EVENT_PICTURE_TAKEN: u32 = 3u32;
pub const WIA_MAX_CTX_SIZE: u32 = 16777216u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIA_MICR {
    pub Tag: u32,
    pub Version: u32,
    pub Size: u32,
    pub Placeholder: u16,
    pub Reserved: u16,
    pub Count: u32,
    pub Micr: [WIA_MICR_INFO; 1],
}
impl Default for WIA_MICR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIA_MICR_INFO {
    pub Size: u32,
    pub Page: u32,
    pub Length: u32,
    pub Text: [u16; 1],
}
impl Default for WIA_MICR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WIA_MICR_READER_AUTO: u32 = 1u32;
pub const WIA_MICR_READER_DISABLED: u32 = 0u32;
pub const WIA_MICR_READER_FEEDER_BACK: u32 = 4u32;
pub const WIA_MICR_READER_FEEDER_DUPLEX: u32 = 5u32;
pub const WIA_MICR_READER_FEEDER_FRONT: u32 = 3u32;
pub const WIA_MICR_READER_FLATBED: u32 = 2u32;
pub const WIA_MULTI_FEED_DETECT_CONTINUE: u32 = 3u32;
pub const WIA_MULTI_FEED_DETECT_DISABLED: u32 = 0u32;
pub const WIA_MULTI_FEED_DETECT_METHOD_LENGTH: u32 = 0u32;
pub const WIA_MULTI_FEED_DETECT_METHOD_OVERLAP: u32 = 1u32;
pub const WIA_MULTI_FEED_DETECT_STOP_ERROR: u32 = 1u32;
pub const WIA_MULTI_FEED_DETECT_STOP_SUCCESS: u32 = 2u32;
pub const WIA_NOTIFICATION_EVENT: u32 = 1u32;
pub const WIA_NUM_DIP: u32 = 16u32;
pub const WIA_NUM_IPC: u32 = 9u32;
pub const WIA_ORDER_BGR: u32 = 1u32;
pub const WIA_ORDER_RGB: u32 = 0u32;
pub const WIA_OVER_SCAN_ALL: u32 = 3u32;
pub const WIA_OVER_SCAN_DISABLED: u32 = 0u32;
pub const WIA_OVER_SCAN_LEFT_RIGHT: u32 = 2u32;
pub const WIA_OVER_SCAN_TOP_BOTTOM: u32 = 1u32;
pub const WIA_PACKED_PIXEL: u32 = 0u32;
pub const WIA_PAGE_A4: u32 = 0u32;
pub const WIA_PAGE_AUTO: u32 = 100u32;
pub const WIA_PAGE_BUSINESSCARD: u32 = 6u32;
pub const WIA_PAGE_CUSTOM: u32 = 2u32;
pub const WIA_PAGE_CUSTOM_BASE: u32 = 32768u32;
pub const WIA_PAGE_DIN_2B: u32 = 52u32;
pub const WIA_PAGE_DIN_4B: u32 = 53u32;
pub const WIA_PAGE_ISO_A0: u32 = 7u32;
pub const WIA_PAGE_ISO_A1: u32 = 8u32;
pub const WIA_PAGE_ISO_A10: u32 = 16u32;
pub const WIA_PAGE_ISO_A2: u32 = 9u32;
pub const WIA_PAGE_ISO_A3: u32 = 10u32;
pub const WIA_PAGE_ISO_A4: u32 = 0u32;
pub const WIA_PAGE_ISO_A5: u32 = 11u32;
pub const WIA_PAGE_ISO_A6: u32 = 12u32;
pub const WIA_PAGE_ISO_A7: u32 = 13u32;
pub const WIA_PAGE_ISO_A8: u32 = 14u32;
pub const WIA_PAGE_ISO_A9: u32 = 15u32;
pub const WIA_PAGE_ISO_B0: u32 = 17u32;
pub const WIA_PAGE_ISO_B1: u32 = 18u32;
pub const WIA_PAGE_ISO_B10: u32 = 27u32;
pub const WIA_PAGE_ISO_B2: u32 = 19u32;
pub const WIA_PAGE_ISO_B3: u32 = 20u32;
pub const WIA_PAGE_ISO_B4: u32 = 21u32;
pub const WIA_PAGE_ISO_B5: u32 = 22u32;
pub const WIA_PAGE_ISO_B6: u32 = 23u32;
pub const WIA_PAGE_ISO_B7: u32 = 24u32;
pub const WIA_PAGE_ISO_B8: u32 = 25u32;
pub const WIA_PAGE_ISO_B9: u32 = 26u32;
pub const WIA_PAGE_ISO_C0: u32 = 28u32;
pub const WIA_PAGE_ISO_C1: u32 = 29u32;
pub const WIA_PAGE_ISO_C10: u32 = 38u32;
pub const WIA_PAGE_ISO_C2: u32 = 30u32;
pub const WIA_PAGE_ISO_C3: u32 = 31u32;
pub const WIA_PAGE_ISO_C4: u32 = 32u32;
pub const WIA_PAGE_ISO_C5: u32 = 33u32;
pub const WIA_PAGE_ISO_C6: u32 = 34u32;
pub const WIA_PAGE_ISO_C7: u32 = 35u32;
pub const WIA_PAGE_ISO_C8: u32 = 36u32;
pub const WIA_PAGE_ISO_C9: u32 = 37u32;
pub const WIA_PAGE_JIS_2A: u32 = 50u32;
pub const WIA_PAGE_JIS_4A: u32 = 51u32;
pub const WIA_PAGE_JIS_B0: u32 = 39u32;
pub const WIA_PAGE_JIS_B1: u32 = 40u32;
pub const WIA_PAGE_JIS_B10: u32 = 49u32;
pub const WIA_PAGE_JIS_B2: u32 = 41u32;
pub const WIA_PAGE_JIS_B3: u32 = 42u32;
pub const WIA_PAGE_JIS_B4: u32 = 43u32;
pub const WIA_PAGE_JIS_B5: u32 = 44u32;
pub const WIA_PAGE_JIS_B6: u32 = 45u32;
pub const WIA_PAGE_JIS_B7: u32 = 46u32;
pub const WIA_PAGE_JIS_B8: u32 = 47u32;
pub const WIA_PAGE_JIS_B9: u32 = 48u32;
pub const WIA_PAGE_LETTER: u32 = 1u32;
pub const WIA_PAGE_USLEDGER: u32 = 4u32;
pub const WIA_PAGE_USLEGAL: u32 = 3u32;
pub const WIA_PAGE_USLETTER: u32 = 1u32;
pub const WIA_PAGE_USSTATEMENT: u32 = 5u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIA_PATCH_CODES {
    pub Tag: u32,
    pub Version: u32,
    pub Size: u32,
    pub Count: u32,
    pub PatchCodes: [WIA_PATCH_CODE_INFO; 1],
}
impl Default for WIA_PATCH_CODES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WIA_PATCH_CODE_1: u32 = 1u32;
pub const WIA_PATCH_CODE_10: u32 = 10u32;
pub const WIA_PATCH_CODE_11: u32 = 11u32;
pub const WIA_PATCH_CODE_12: u32 = 12u32;
pub const WIA_PATCH_CODE_13: u32 = 13u32;
pub const WIA_PATCH_CODE_14: u32 = 14u32;
pub const WIA_PATCH_CODE_2: u32 = 2u32;
pub const WIA_PATCH_CODE_3: u32 = 3u32;
pub const WIA_PATCH_CODE_4: u32 = 4u32;
pub const WIA_PATCH_CODE_6: u32 = 6u32;
pub const WIA_PATCH_CODE_7: u32 = 7u32;
pub const WIA_PATCH_CODE_8: u32 = 8u32;
pub const WIA_PATCH_CODE_9: u32 = 9u32;
pub const WIA_PATCH_CODE_CUSTOM_BASE: u32 = 32768u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIA_PATCH_CODE_INFO {
    pub Type: u32,
}
pub const WIA_PATCH_CODE_READER_AUTO: u32 = 1u32;
pub const WIA_PATCH_CODE_READER_DISABLED: u32 = 0u32;
pub const WIA_PATCH_CODE_READER_FEEDER_BACK: u32 = 4u32;
pub const WIA_PATCH_CODE_READER_FEEDER_DUPLEX: u32 = 5u32;
pub const WIA_PATCH_CODE_READER_FEEDER_FRONT: u32 = 3u32;
pub const WIA_PATCH_CODE_READER_FLATBED: u32 = 2u32;
pub const WIA_PATCH_CODE_T: u32 = 5u32;
pub const WIA_PATCH_CODE_UNKNOWN: u32 = 0u32;
pub const WIA_PHOTO_WHITE_0: u32 = 1u32;
pub const WIA_PHOTO_WHITE_1: u32 = 0u32;
pub const WIA_PLANAR: u32 = 1u32;
pub const WIA_PREVIEW_SCAN: u32 = 1u32;
pub const WIA_PRINTER_ENDORSER_AFTER_SCAN: u32 = 1u32;
pub const WIA_PRINTER_ENDORSER_AUTO: u32 = 1u32;
pub const WIA_PRINTER_ENDORSER_BEFORE_SCAN: u32 = 0u32;
pub const WIA_PRINTER_ENDORSER_DIGITAL: u32 = 6u32;
pub const WIA_PRINTER_ENDORSER_DISABLED: u32 = 0u32;
pub const WIA_PRINTER_ENDORSER_FEEDER_BACK: u32 = 4u32;
pub const WIA_PRINTER_ENDORSER_FEEDER_DUPLEX: u32 = 5u32;
pub const WIA_PRINTER_ENDORSER_FEEDER_FRONT: u32 = 3u32;
pub const WIA_PRINTER_ENDORSER_FLATBED: u32 = 2u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BACKGROUND: u32 = 8u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM: u32 = 3u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM_LEFT: u32 = 6u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_BOTTOM_RIGHT: u32 = 7u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_DEVICE_DEFAULT: u32 = 9u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_LEFT: u32 = 0u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_RIGHT: u32 = 1u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_TOP: u32 = 2u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_TOP_LEFT: u32 = 4u32;
pub const WIA_PRINTER_ENDORSER_GRAPHICS_TOP_RIGHT: u32 = 5u32;
pub const WIA_PRINT_AM_PM: u32 = 9u32;
pub const WIA_PRINT_DATE: u32 = 0u32;
pub const WIA_PRINT_DAY: u32 = 3u32;
pub const WIA_PRINT_FONT_BOLD: u32 = 1u32;
pub const WIA_PRINT_FONT_EXTRA_BOLD: u32 = 2u32;
pub const WIA_PRINT_FONT_ITALIC: u32 = 5u32;
pub const WIA_PRINT_FONT_ITALIC_BOLD: u32 = 3u32;
pub const WIA_PRINT_FONT_ITALIC_EXTRA_BOLD: u32 = 4u32;
pub const WIA_PRINT_FONT_LARGE: u32 = 12u32;
pub const WIA_PRINT_FONT_LARGE_BOLD: u32 = 13u32;
pub const WIA_PRINT_FONT_LARGE_EXTRA_BOLD: u32 = 14u32;
pub const WIA_PRINT_FONT_LARGE_ITALIC: u32 = 17u32;
pub const WIA_PRINT_FONT_LARGE_ITALIC_BOLD: u32 = 15u32;
pub const WIA_PRINT_FONT_LARGE_ITALIC_EXTRA_BOLD: u32 = 16u32;
pub const WIA_PRINT_FONT_NORMAL: u32 = 0u32;
pub const WIA_PRINT_FONT_SMALL: u32 = 6u32;
pub const WIA_PRINT_FONT_SMALL_BOLD: u32 = 7u32;
pub const WIA_PRINT_FONT_SMALL_EXTRA_BOLD: u32 = 8u32;
pub const WIA_PRINT_FONT_SMALL_ITALIC: u32 = 11u32;
pub const WIA_PRINT_FONT_SMALL_ITALIC_BOLD: u32 = 9u32;
pub const WIA_PRINT_FONT_SMALL_ITALIC_EXTRA_BOLD: u32 = 10u32;
pub const WIA_PRINT_HOUR_12H: u32 = 8u32;
pub const WIA_PRINT_HOUR_24H: u32 = 7u32;
pub const WIA_PRINT_IMAGE: u32 = 13u32;
pub const WIA_PRINT_MILLISECOND: u32 = 14u32;
pub const WIA_PRINT_MINUTE: u32 = 10u32;
pub const WIA_PRINT_MONTH: u32 = 2u32;
pub const WIA_PRINT_MONTH_NAME: u32 = 15u32;
pub const WIA_PRINT_MONTH_SHORT: u32 = 16u32;
pub const WIA_PRINT_PADDING_BLANK: u32 = 2u32;
pub const WIA_PRINT_PADDING_NONE: u32 = 0u32;
pub const WIA_PRINT_PADDING_ZERO: u32 = 1u32;
pub const WIA_PRINT_PAGE_COUNT: u32 = 12u32;
pub const WIA_PRINT_SECOND: u32 = 11u32;
pub const WIA_PRINT_TIME_12H: u32 = 6u32;
pub const WIA_PRINT_TIME_24H: u32 = 5u32;
pub const WIA_PRINT_WEEK_DAY: u32 = 4u32;
pub const WIA_PRINT_WEEK_DAY_SHORT: u32 = 17u32;
pub const WIA_PRINT_YEAR: u32 = 1u32;
pub const WIA_PRIVATE_DEVPROP: u32 = 38914u32;
pub const WIA_PRIVATE_ITEMPROP: u32 = 71682u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIA_PROPERTY_CONTEXT {
    pub cProps: u32,
    pub pProps: *mut u32,
    pub pChanged: *mut windows_core::BOOL,
}
impl Default for WIA_PROPERTY_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
pub struct WIA_PROPERTY_INFO {
    pub lAccessFlags: u32,
    pub vt: super::super::System::Variant::VARENUM,
    pub ValidVal: WIA_PROPERTY_INFO_0,
}
#[cfg(feature = "Win32_System_Variant")]
impl Clone for WIA_PROPERTY_INFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Variant")]
impl Default for WIA_PROPERTY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
pub union WIA_PROPERTY_INFO_0 {
    pub Range: WIA_PROPERTY_INFO_0_0,
    pub RangeFloat: WIA_PROPERTY_INFO_0_1,
    pub List: WIA_PROPERTY_INFO_0_2,
    pub ListFloat: WIA_PROPERTY_INFO_0_3,
    pub ListGuid: WIA_PROPERTY_INFO_0_4,
    pub ListBStr: core::mem::ManuallyDrop<WIA_PROPERTY_INFO_0_5>,
    pub Flag: WIA_PROPERTY_INFO_0_6,
    pub None: WIA_PROPERTY_INFO_0_7,
}
#[cfg(feature = "Win32_System_Variant")]
impl Clone for WIA_PROPERTY_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Variant")]
impl Default for WIA_PROPERTY_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIA_PROPERTY_INFO_0_6 {
    pub Nom: i32,
    pub ValidBits: i32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Debug, PartialEq)]
pub struct WIA_PROPERTY_INFO_0_5 {
    pub cNumList: i32,
    pub Nom: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub pList: *mut windows_core::BSTR,
}
#[cfg(feature = "Win32_System_Variant")]
impl Default for WIA_PROPERTY_INFO_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIA_PROPERTY_INFO_0_3 {
    pub cNumList: i32,
    pub Nom: f64,
    pub pList: *mut u8,
}
#[cfg(feature = "Win32_System_Variant")]
impl Default for WIA_PROPERTY_INFO_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIA_PROPERTY_INFO_0_4 {
    pub cNumList: i32,
    pub Nom: windows_core::GUID,
    pub pList: *mut windows_core::GUID,
}
#[cfg(feature = "Win32_System_Variant")]
impl Default for WIA_PROPERTY_INFO_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIA_PROPERTY_INFO_0_2 {
    pub cNumList: i32,
    pub Nom: i32,
    pub pList: *mut u8,
}
#[cfg(feature = "Win32_System_Variant")]
impl Default for WIA_PROPERTY_INFO_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIA_PROPERTY_INFO_0_7 {
    pub Dummy: i32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIA_PROPERTY_INFO_0_1 {
    pub Min: f64,
    pub Nom: f64,
    pub Max: f64,
    pub Inc: f64,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIA_PROPERTY_INFO_0_0 {
    pub Min: i32,
    pub Nom: i32,
    pub Max: i32,
    pub Inc: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIA_PROPID_TO_NAME {
    pub propid: u32,
    pub pszName: windows_core::PWSTR,
}
pub const WIA_PROPPAGE_CAMERA_ITEM_GENERAL: u32 = 2u32;
pub const WIA_PROPPAGE_DEVICE_GENERAL: u32 = 4u32;
pub const WIA_PROPPAGE_SCANNER_ITEM_GENERAL: u32 = 1u32;
pub const WIA_PROP_CACHEABLE: u32 = 65536u32;
pub const WIA_PROP_FLAG: u32 = 64u32;
pub const WIA_PROP_LIST: u32 = 32u32;
pub const WIA_PROP_NONE: u32 = 8u32;
pub const WIA_PROP_RANGE: u32 = 16u32;
pub const WIA_PROP_READ: u32 = 1u32;
pub const WIA_PROP_SYNC_REQUIRED: u32 = 4u32;
pub const WIA_PROP_WRITE: u32 = 2u32;
pub const WIA_RANGE_MAX: u32 = 2u32;
pub const WIA_RANGE_MIN: u32 = 0u32;
pub const WIA_RANGE_NOM: u32 = 1u32;
pub const WIA_RANGE_NUM_ELEMS: u32 = 4u32;
pub const WIA_RANGE_STEP: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIA_RAW_HEADER {
    pub Tag: u32,
    pub Version: u32,
    pub HeaderSize: u32,
    pub XRes: u32,
    pub YRes: u32,
    pub XExtent: u32,
    pub YExtent: u32,
    pub BytesPerLine: u32,
    pub BitsPerPixel: u32,
    pub ChannelsPerPixel: u32,
    pub DataType: u32,
    pub BitsPerChannel: [u8; 8],
    pub Compression: u32,
    pub PhotometricInterp: u32,
    pub LineOrder: u32,
    pub RawDataOffset: u32,
    pub RawDataSize: u32,
    pub PaletteOffset: u32,
    pub PaletteSize: u32,
}
impl Default for WIA_RAW_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WIA_REGISTER_EVENT_CALLBACK: u32 = 1u32;
pub const WIA_RESERVED_FOR_NEW_PROPS: u32 = 1024u32;
pub const WIA_SCAN_AHEAD_ALL: u32 = 0u32;
pub const WIA_SCAN_AHEAD_DISABLED: u32 = 0u32;
pub const WIA_SCAN_AHEAD_ENABLED: u32 = 1u32;
pub const WIA_SEGMENTATION_FILTER_STR: windows_core::PCWSTR = windows_core::w!("SegmentationFilter");
pub const WIA_SELECT_DEVICE_NODEFAULT: u32 = 1u32;
pub const WIA_SEPARATOR_DETECT_NOSCAN_CONTINUE: u32 = 3u32;
pub const WIA_SEPARATOR_DETECT_NOSCAN_STOP: u32 = 4u32;
pub const WIA_SEPARATOR_DETECT_SCAN_CONTINUE: u32 = 1u32;
pub const WIA_SEPARATOR_DETECT_SCAN_STOP: u32 = 2u32;
pub const WIA_SEPARATOR_DISABLED: u32 = 0u32;
pub const WIA_SET_DEFAULT_HANDLER: u32 = 4u32;
pub const WIA_SHOW_PREVIEW_CONTROL: u32 = 0u32;
pub const WIA_STATUS_CALIBRATING: windows_core::HRESULT = windows_core::HRESULT(0x210003_u32 as _);
pub const WIA_STATUS_CLEAR: windows_core::HRESULT = windows_core::HRESULT(0x210008_u32 as _);
pub const WIA_STATUS_END_OF_MEDIA: windows_core::HRESULT = windows_core::HRESULT(0x210001_u32 as _);
pub const WIA_STATUS_NETWORK_DEVICE_RESERVED: windows_core::HRESULT = windows_core::HRESULT(0x210007_u32 as _);
pub const WIA_STATUS_NOT_HANDLED: windows_core::HRESULT = windows_core::HRESULT(0x21000A_u32 as _);
pub const WIA_STATUS_RESERVING_NETWORK_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0x210006_u32 as _);
pub const WIA_STATUS_SKIP_ITEM: windows_core::HRESULT = windows_core::HRESULT(0x210009_u32 as _);
pub const WIA_STATUS_WARMING_UP: windows_core::HRESULT = windows_core::HRESULT(0x210002_u32 as _);
pub const WIA_S_CHANGE_DEVICE: windows_core::HRESULT = windows_core::HRESULT(0x21000B_u32 as _);
pub const WIA_S_NO_DEVICE_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80210015_u32 as _);
pub const WIA_TRANSFER_ACQUIRE_CHILDREN: u32 = 1u32;
pub const WIA_TRANSFER_CHILDREN_SINGLE_SCAN: u32 = 1u32;
pub const WIA_TRANSFER_MSG_DEVICE_STATUS: u32 = 5u32;
pub const WIA_TRANSFER_MSG_END_OF_STREAM: u32 = 2u32;
pub const WIA_TRANSFER_MSG_END_OF_TRANSFER: u32 = 3u32;
pub const WIA_TRANSFER_MSG_NEW_PAGE: u32 = 6u32;
pub const WIA_TRANSFER_MSG_STATUS: u32 = 1u32;
pub const WIA_UNREGISTER_EVENT_CALLBACK: u32 = 2u32;
pub const WIA_USE_SEGMENTATION_FILTER: u32 = 0u32;
pub const WIA_WSD_FRIENDLY_NAME: u32 = 38920u32;
pub const WIA_WSD_FRIENDLY_NAME_STR: windows_core::PCWSTR = windows_core::w!("Friendly name");
pub const WIA_WSD_MANUFACTURER: u32 = 38914u32;
pub const WIA_WSD_MANUFACTURER_STR: windows_core::PCWSTR = windows_core::w!("Device manufacturer");
pub const WIA_WSD_MANUFACTURER_URL: u32 = 38915u32;
pub const WIA_WSD_MANUFACTURER_URL_STR: windows_core::PCWSTR = windows_core::w!("Manufacurer URL");
pub const WIA_WSD_MODEL_NAME: u32 = 38916u32;
pub const WIA_WSD_MODEL_NAME_STR: windows_core::PCWSTR = windows_core::w!("Model name");
pub const WIA_WSD_MODEL_NUMBER: u32 = 38917u32;
pub const WIA_WSD_MODEL_NUMBER_STR: windows_core::PCWSTR = windows_core::w!("Model number");
pub const WIA_WSD_MODEL_URL: u32 = 38918u32;
pub const WIA_WSD_MODEL_URL_STR: windows_core::PCWSTR = windows_core::w!("Model URL");
pub const WIA_WSD_PRESENTATION_URL: u32 = 38919u32;
pub const WIA_WSD_PRESENTATION_URL_STR: windows_core::PCWSTR = windows_core::w!("Presentation URL");
pub const WIA_WSD_SCAN_AVAILABLE_ITEM: u32 = 38922u32;
pub const WIA_WSD_SCAN_AVAILABLE_ITEM_STR: windows_core::PCWSTR = windows_core::w!("Scan Available Item");
pub const WIA_WSD_SERIAL_NUMBER: u32 = 38921u32;
pub const WIA_WSD_SERIAL_NUMBER_STR: windows_core::PCWSTR = windows_core::w!("Serial number");
pub const WiaAudFmt_AIFF: windows_core::GUID = windows_core::GUID::from_u128(0x66e2bf4f_b6fc_443f_94c8_2f33c8a65aaf);
pub const WiaAudFmt_MP3: windows_core::GUID = windows_core::GUID::from_u128(0x0fbc71fb_43bf_49f2_9190_e6fecff37e54);
pub const WiaAudFmt_WAV: windows_core::GUID = windows_core::GUID::from_u128(0xf818e146_07af_40ff_ae55_be8f2c065dbe);
pub const WiaAudFmt_WMA: windows_core::GUID = windows_core::GUID::from_u128(0xd61d6413_8bc2_438f_93ad_21bd484db6a1);
pub const WiaDevMgr: windows_core::GUID = windows_core::GUID::from_u128(0xa1f4e726_8cf1_11d1_bf92_0060081ed811);
pub const WiaDevMgr2: windows_core::GUID = windows_core::GUID::from_u128(0xb6c292bc_7c88_41ee_8b54_8ec92617e599);
pub const WiaImgFmt_ASF: windows_core::GUID = windows_core::GUID::from_u128(0x8d948ee9_d0aa_4a12_9d9a_9cc5de36199b);
pub const WiaImgFmt_AVI: windows_core::GUID = windows_core::GUID::from_u128(0x32f8ca14_087c_4908_b7c4_6757fe7e90ab);
pub const WiaImgFmt_BMP: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cab_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_CIFF: windows_core::GUID = windows_core::GUID::from_u128(0x9821a8ab_3a7e_4215_94e0_d27a460c03b2);
pub const WiaImgFmt_CSV: windows_core::GUID = windows_core::GUID::from_u128(0x355bda24_5a9f_4494_80dc_be752cecbc8c);
pub const WiaImgFmt_DPOF: windows_core::GUID = windows_core::GUID::from_u128(0x369eeeab_a0e8_45ca_86a6_a83ce5697e28);
pub const WiaImgFmt_EMF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cac_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_EXEC: windows_core::GUID = windows_core::GUID::from_u128(0x485da097_141e_4aa5_bb3b_a5618d95d02b);
pub const WiaImgFmt_EXIF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb2_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_FLASHPIX: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb4_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_GIF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb0_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_HTML: windows_core::GUID = windows_core::GUID::from_u128(0xc99a4e62_99de_4a94_acca_71956ac2977d);
pub const WiaImgFmt_ICO: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb5_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_JBIG: windows_core::GUID = windows_core::GUID::from_u128(0x41e8dd92_2f0a_43d4_8636_f1614ba11e46);
pub const WiaImgFmt_JBIG2: windows_core::GUID = windows_core::GUID::from_u128(0xbb8e7e67_283c_4235_9e59_0b9bf94ca687);
pub const WiaImgFmt_JPEG: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cae_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_JPEG2K: windows_core::GUID = windows_core::GUID::from_u128(0x344ee2b2_39db_4dde_8173_c4b75f8f1e49);
pub const WiaImgFmt_JPEG2KX: windows_core::GUID = windows_core::GUID::from_u128(0x43e14614_c80a_4850_baf3_4b152dc8da27);
pub const WiaImgFmt_MEMORYBMP: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3caa_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_MPG: windows_core::GUID = windows_core::GUID::from_u128(0xecd757e4_d2ec_4f57_955d_bcf8a97c4e52);
pub const WiaImgFmt_OXPS: windows_core::GUID = windows_core::GUID::from_u128(0x2c7b1240_c14d_4109_9755_04b89025153a);
pub const WiaImgFmt_PDFA: windows_core::GUID = windows_core::GUID::from_u128(0x9980bd5b_3463_43c7_bdca_3caa146f229f);
pub const WiaImgFmt_PHOTOCD: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb3_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_PICT: windows_core::GUID = windows_core::GUID::from_u128(0xa6bc85d8_6b3e_40ee_a95c_25d482e41adc);
pub const WiaImgFmt_PNG: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3caf_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_RAW: windows_core::GUID = windows_core::GUID::from_u128(0x6f120719_f1a8_4e07_9ade_9b64c63a3dcc);
pub const WiaImgFmt_RAWBAR: windows_core::GUID = windows_core::GUID::from_u128(0xda63f833_d26e_451e_90d2_ea55a1365d62);
pub const WiaImgFmt_RAWMIC: windows_core::GUID = windows_core::GUID::from_u128(0x22c4f058_0d88_409c_ac1c_eec12b0ea680);
pub const WiaImgFmt_RAWPAT: windows_core::GUID = windows_core::GUID::from_u128(0x7760507c_5064_400c_9a17_575624d8824b);
pub const WiaImgFmt_RAWRGB: windows_core::GUID = windows_core::GUID::from_u128(0xbca48b55_f272_4371_b0f1_4a150d057bb4);
pub const WiaImgFmt_RTF: windows_core::GUID = windows_core::GUID::from_u128(0x573dd6a3_4834_432d_a9b5_e198dd9e890d);
pub const WiaImgFmt_SCRIPT: windows_core::GUID = windows_core::GUID::from_u128(0xfe7d6c53_2dac_446a_b0bd_d73e21e924c9);
pub const WiaImgFmt_TIFF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb1_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_TXT: windows_core::GUID = windows_core::GUID::from_u128(0xfafd4d82_723f_421f_9318_30501ac44b59);
pub const WiaImgFmt_UNDEFINED: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3ca9_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_UNICODE16: windows_core::GUID = windows_core::GUID::from_u128(0x1b7639b6_6357_47d1_9a07_12452dc073e9);
pub const WiaImgFmt_WMF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cad_0728_11d3_9d7b_0000f81ef32e);
pub const WiaImgFmt_XML: windows_core::GUID = windows_core::GUID::from_u128(0xb9171457_dac8_4884_b393_15b471d5f07e);
pub const WiaImgFmt_XMLBAR: windows_core::GUID = windows_core::GUID::from_u128(0x6235701c_3a98_484c_b2a8_fdffd87e6b16);
pub const WiaImgFmt_XMLMIC: windows_core::GUID = windows_core::GUID::from_u128(0x2d164c61_b9ae_4b23_8973_c7067e1fbd31);
pub const WiaImgFmt_XMLPAT: windows_core::GUID = windows_core::GUID::from_u128(0xf8986f55_f052_460d_9523_3a7dfedbb33c);
pub const WiaImgFmt_XPS: windows_core::GUID = windows_core::GUID::from_u128(0x700b4a0f_2011_411c_b430_d1e0b2e10b28);
pub const WiaItemTypeAnalyze: u32 = 16u32;
pub const WiaItemTypeAudio: u32 = 32u32;
pub const WiaItemTypeBurst: u32 = 2048u32;
pub const WiaItemTypeDeleted: u32 = 128u32;
pub const WiaItemTypeDevice: u32 = 64u32;
pub const WiaItemTypeDisconnected: u32 = 256u32;
pub const WiaItemTypeDocument: u32 = 262144u32;
pub const WiaItemTypeFile: u32 = 2u32;
pub const WiaItemTypeFolder: u32 = 4u32;
pub const WiaItemTypeFree: u32 = 0u32;
pub const WiaItemTypeGenerated: u32 = 16384u32;
pub const WiaItemTypeHPanorama: u32 = 512u32;
pub const WiaItemTypeHasAttachments: u32 = 32768u32;
pub const WiaItemTypeImage: u32 = 1u32;
pub const WiaItemTypeMask: u32 = 2148532223u32;
pub const WiaItemTypeProgrammableDataSource: u32 = 524288u32;
pub const WiaItemTypeRemoved: u32 = 2147483648u32;
pub const WiaItemTypeRoot: u32 = 8u32;
pub const WiaItemTypeStorage: u32 = 4096u32;
pub const WiaItemTypeTransfer: u32 = 8192u32;
pub const WiaItemTypeTwainCapabilityPassThrough: u32 = 131072u32;
pub const WiaItemTypeVPanorama: u32 = 1024u32;
pub const WiaItemTypeVideo: u32 = 65536u32;
pub const WiaLog: windows_core::GUID = windows_core::GUID::from_u128(0xa1e75357_881a_419e_83e2_bb16db197c68);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WiaTransferParams {
    pub lMessage: i32,
    pub lPercentComplete: i32,
    pub ulTransferredBytes: u64,
    pub hrErrorStatus: windows_core::HRESULT,
}
pub const WiaVideo: windows_core::GUID = windows_core::GUID::from_u128(0x3908c3cd_4478_4536_af2f_10c25d4ef89a);
pub const g_dwDebugFlags: u32 = 0u32;
