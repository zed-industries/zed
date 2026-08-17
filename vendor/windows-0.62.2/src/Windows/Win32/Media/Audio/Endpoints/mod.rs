#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    pub u32Size: u32,
    pub u32TSSessionId: u32,
    pub targetEndpointConnectorType: EndpointConnectorType,
    pub wfxDeviceFormat: super::WAVEFORMATEX,
}
pub const DEVINTERFACE_AUDIOENDPOINTPLUGIN: windows_core::GUID = windows_core::GUID::from_u128(0x9f2f7b66_65ac_4fa6_8ae4_123c78b89313);
pub const DEVPKEY_AudioEndpointPlugin2_FactoryCLSID: super::super::super::Foundation::PROPERTYKEY = super::super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 4 };
pub const DEVPKEY_AudioEndpointPlugin_DataFlow: super::super::super::Foundation::PROPERTYKEY = super::super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 2 };
pub const DEVPKEY_AudioEndpointPlugin_FactoryCLSID: super::super::super::Foundation::PROPERTYKEY = super::super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 1 };
pub const DEVPKEY_AudioEndpointPlugin_PnPInterface: super::super::super::Foundation::PROPERTYKEY = super::super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 3 };
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EndpointConnectorType(pub i32);
windows_core::imp::define_interface!(IAudioEndpointFormatControl, IAudioEndpointFormatControl_Vtbl, 0x784cfd40_9f89_456e_a1a6_873b006a664e);
windows_core::imp::interface_hierarchy!(IAudioEndpointFormatControl, windows_core::IUnknown);
impl IAudioEndpointFormatControl {
    pub unsafe fn ResetToDefault(&self, resetflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResetToDefault)(windows_core::Interface::as_raw(self), resetflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointFormatControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ResetToDefault: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IAudioEndpointFormatControl_Impl: windows_core::IUnknownImpl {
    fn ResetToDefault(&self, resetflags: u32) -> windows_core::Result<()>;
}
impl IAudioEndpointFormatControl_Vtbl {
    pub const fn new<Identity: IAudioEndpointFormatControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResetToDefault<Identity: IAudioEndpointFormatControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resetflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointFormatControl_Impl::ResetToDefault(this, core::mem::transmute_copy(&resetflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ResetToDefault: ResetToDefault::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointFormatControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioEndpointFormatControl {}
windows_core::imp::define_interface!(IAudioEndpointLastBufferControl, IAudioEndpointLastBufferControl_Vtbl, 0xf8520dd3_8f9d_4437_9861_62f584c33dd6);
windows_core::imp::interface_hierarchy!(IAudioEndpointLastBufferControl, windows_core::IUnknown);
impl IAudioEndpointLastBufferControl {
    pub unsafe fn IsLastBufferControlSupported(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsLastBufferControlSupported)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub unsafe fn ReleaseOutputDataPointerForLastBuffer(&self, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY) {
        unsafe { (windows_core::Interface::vtable(self).ReleaseOutputDataPointerForLastBuffer)(windows_core::Interface::as_raw(self), pconnectionproperty) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointLastBufferControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsLastBufferControlSupported: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub ReleaseOutputDataPointerForLastBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Apo::APO_CONNECTION_PROPERTY),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))]
    ReleaseOutputDataPointerForLastBuffer: usize,
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
pub trait IAudioEndpointLastBufferControl_Impl: windows_core::IUnknownImpl {
    fn IsLastBufferControlSupported(&self) -> windows_core::BOOL;
    fn ReleaseOutputDataPointerForLastBuffer(&self, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY);
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl IAudioEndpointLastBufferControl_Vtbl {
    pub const fn new<Identity: IAudioEndpointLastBufferControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsLastBufferControlSupported<Identity: IAudioEndpointLastBufferControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointLastBufferControl_Impl::IsLastBufferControlSupported(this)
            }
        }
        unsafe extern "system" fn ReleaseOutputDataPointerForLastBuffer<Identity: IAudioEndpointLastBufferControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointLastBufferControl_Impl::ReleaseOutputDataPointerForLastBuffer(this, core::mem::transmute_copy(&pconnectionproperty))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsLastBufferControlSupported: IsLastBufferControlSupported::<Identity, OFFSET>,
            ReleaseOutputDataPointerForLastBuffer: ReleaseOutputDataPointerForLastBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointLastBufferControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl windows_core::RuntimeName for IAudioEndpointLastBufferControl {}
windows_core::imp::define_interface!(IAudioEndpointOffloadStreamMeter, IAudioEndpointOffloadStreamMeter_Vtbl, 0xe1546dce_9dd1_418b_9ab2_348ced161c86);
windows_core::imp::interface_hierarchy!(IAudioEndpointOffloadStreamMeter, windows_core::IUnknown);
impl IAudioEndpointOffloadStreamMeter {
    pub unsafe fn GetMeterChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMeterChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMeteringData(&self, u32channelcount: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMeteringData)(windows_core::Interface::as_raw(self), u32channelcount, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointOffloadStreamMeter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMeterChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMeteringData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
}
pub trait IAudioEndpointOffloadStreamMeter_Impl: windows_core::IUnknownImpl {
    fn GetMeterChannelCount(&self) -> windows_core::Result<u32>;
    fn GetMeteringData(&self, u32channelcount: u32) -> windows_core::Result<f32>;
}
impl IAudioEndpointOffloadStreamMeter_Vtbl {
    pub const fn new<Identity: IAudioEndpointOffloadStreamMeter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMeterChannelCount<Identity: IAudioEndpointOffloadStreamMeter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pu32channelcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointOffloadStreamMeter_Impl::GetMeterChannelCount(this) {
                    Ok(ok__) => {
                        pu32channelcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMeteringData<Identity: IAudioEndpointOffloadStreamMeter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32channelcount: u32, pf32peakvalues: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointOffloadStreamMeter_Impl::GetMeteringData(this, core::mem::transmute_copy(&u32channelcount)) {
                    Ok(ok__) => {
                        pf32peakvalues.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMeterChannelCount: GetMeterChannelCount::<Identity, OFFSET>,
            GetMeteringData: GetMeteringData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointOffloadStreamMeter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioEndpointOffloadStreamMeter {}
windows_core::imp::define_interface!(IAudioEndpointOffloadStreamMute, IAudioEndpointOffloadStreamMute_Vtbl, 0xdfe21355_5ec2_40e0_8d6b_710ac3c00249);
windows_core::imp::interface_hierarchy!(IAudioEndpointOffloadStreamMute, windows_core::IUnknown);
impl IAudioEndpointOffloadStreamMute {
    pub unsafe fn SetMute(&self, bmuted: u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMute)(windows_core::Interface::as_raw(self), bmuted).ok() }
    }
    pub unsafe fn GetMute(&self) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMute)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointOffloadStreamMute_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMute: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub GetMute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
pub trait IAudioEndpointOffloadStreamMute_Impl: windows_core::IUnknownImpl {
    fn SetMute(&self, bmuted: u8) -> windows_core::Result<()>;
    fn GetMute(&self) -> windows_core::Result<u8>;
}
impl IAudioEndpointOffloadStreamMute_Vtbl {
    pub const fn new<Identity: IAudioEndpointOffloadStreamMute_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMute<Identity: IAudioEndpointOffloadStreamMute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmuted: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointOffloadStreamMute_Impl::SetMute(this, core::mem::transmute_copy(&bmuted)).into()
            }
        }
        unsafe extern "system" fn GetMute<Identity: IAudioEndpointOffloadStreamMute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmuted: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointOffloadStreamMute_Impl::GetMute(this) {
                    Ok(ok__) => {
                        pbmuted.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetMute: SetMute::<Identity, OFFSET>, GetMute: GetMute::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointOffloadStreamMute as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioEndpointOffloadStreamMute {}
windows_core::imp::define_interface!(IAudioEndpointOffloadStreamVolume, IAudioEndpointOffloadStreamVolume_Vtbl, 0x64f1dd49_71ca_4281_8672_3a9eddd1d0b6);
windows_core::imp::interface_hierarchy!(IAudioEndpointOffloadStreamVolume, windows_core::IUnknown);
impl IAudioEndpointOffloadStreamVolume {
    pub unsafe fn GetVolumeChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVolumeChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Media_KernelStreaming")]
    pub unsafe fn SetChannelVolumes(&self, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetChannelVolumes)(windows_core::Interface::as_raw(self), u32channelcount, pf32volumes, u32curvetype, pcurveduration).ok() }
    }
    pub unsafe fn GetChannelVolumes(&self, u32channelcount: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelVolumes)(windows_core::Interface::as_raw(self), u32channelcount, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointOffloadStreamVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVolumeChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_KernelStreaming")]
    pub SetChannelVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, super::super::KernelStreaming::AUDIO_CURVE_TYPE, *const i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_KernelStreaming"))]
    SetChannelVolumes: usize,
    pub GetChannelVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Media_KernelStreaming")]
pub trait IAudioEndpointOffloadStreamVolume_Impl: windows_core::IUnknownImpl {
    fn GetVolumeChannelCount(&self) -> windows_core::Result<u32>;
    fn SetChannelVolumes(&self, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> windows_core::Result<()>;
    fn GetChannelVolumes(&self, u32channelcount: u32) -> windows_core::Result<f32>;
}
#[cfg(feature = "Win32_Media_KernelStreaming")]
impl IAudioEndpointOffloadStreamVolume_Vtbl {
    pub const fn new<Identity: IAudioEndpointOffloadStreamVolume_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVolumeChannelCount<Identity: IAudioEndpointOffloadStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pu32channelcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointOffloadStreamVolume_Impl::GetVolumeChannelCount(this) {
                    Ok(ok__) => {
                        pu32channelcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetChannelVolumes<Identity: IAudioEndpointOffloadStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointOffloadStreamVolume_Impl::SetChannelVolumes(this, core::mem::transmute_copy(&u32channelcount), core::mem::transmute_copy(&pf32volumes), core::mem::transmute_copy(&u32curvetype), core::mem::transmute_copy(&pcurveduration)).into()
            }
        }
        unsafe extern "system" fn GetChannelVolumes<Identity: IAudioEndpointOffloadStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32channelcount: u32, pf32volumes: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointOffloadStreamVolume_Impl::GetChannelVolumes(this, core::mem::transmute_copy(&u32channelcount)) {
                    Ok(ok__) => {
                        pf32volumes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVolumeChannelCount: GetVolumeChannelCount::<Identity, OFFSET>,
            SetChannelVolumes: SetChannelVolumes::<Identity, OFFSET>,
            GetChannelVolumes: GetChannelVolumes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointOffloadStreamVolume as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_KernelStreaming")]
impl windows_core::RuntimeName for IAudioEndpointOffloadStreamVolume {}
windows_core::imp::define_interface!(IAudioEndpointVolume, IAudioEndpointVolume_Vtbl, 0x5cdf2c82_841e_4546_9722_0cf74078229a);
windows_core::imp::interface_hierarchy!(IAudioEndpointVolume, windows_core::IUnknown);
impl IAudioEndpointVolume {
    pub unsafe fn RegisterControlChangeNotify<P0>(&self, pnotify: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioEndpointVolumeCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterControlChangeNotify)(windows_core::Interface::as_raw(self), pnotify.param().abi()).ok() }
    }
    pub unsafe fn UnregisterControlChangeNotify<P0>(&self, pnotify: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioEndpointVolumeCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterControlChangeNotify)(windows_core::Interface::as_raw(self), pnotify.param().abi()).ok() }
    }
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMasterVolumeLevel(&self, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMasterVolumeLevel)(windows_core::Interface::as_raw(self), fleveldb, pguideventcontext).ok() }
    }
    pub unsafe fn SetMasterVolumeLevelScalar(&self, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMasterVolumeLevelScalar)(windows_core::Interface::as_raw(self), flevel, pguideventcontext).ok() }
    }
    pub unsafe fn GetMasterVolumeLevel(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMasterVolumeLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMasterVolumeLevelScalar(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMasterVolumeLevelScalar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetChannelVolumeLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetChannelVolumeLevel)(windows_core::Interface::as_raw(self), nchannel, fleveldb, pguideventcontext).ok() }
    }
    pub unsafe fn SetChannelVolumeLevelScalar(&self, nchannel: u32, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetChannelVolumeLevelScalar)(windows_core::Interface::as_raw(self), nchannel, flevel, pguideventcontext).ok() }
    }
    pub unsafe fn GetChannelVolumeLevel(&self, nchannel: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelVolumeLevel)(windows_core::Interface::as_raw(self), nchannel, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChannelVolumeLevelScalar(&self, nchannel: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelVolumeLevelScalar)(windows_core::Interface::as_raw(self), nchannel, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMute(&self, bmute: bool, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMute)(windows_core::Interface::as_raw(self), bmute.into(), pguideventcontext).ok() }
    }
    pub unsafe fn GetMute(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMute)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVolumeStepInfo(&self, pnstep: *mut u32, pnstepcount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetVolumeStepInfo)(windows_core::Interface::as_raw(self), pnstep as _, pnstepcount as _).ok() }
    }
    pub unsafe fn VolumeStepUp(&self, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).VolumeStepUp)(windows_core::Interface::as_raw(self), pguideventcontext).ok() }
    }
    pub unsafe fn VolumeStepDown(&self, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).VolumeStepDown)(windows_core::Interface::as_raw(self), pguideventcontext).ok() }
    }
    pub unsafe fn QueryHardwareSupport(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryHardwareSupport)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVolumeRange(&self, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetVolumeRange)(windows_core::Interface::as_raw(self), pflvolumemindb as _, pflvolumemaxdb as _, pflvolumeincrementdb as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterControlChangeNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterControlChangeNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMasterVolumeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetMasterVolumeLevelScalar: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetMasterVolumeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetMasterVolumeLevelScalar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetChannelVolumeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetChannelVolumeLevelScalar: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetChannelVolumeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub GetChannelVolumeLevelScalar: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub SetMute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetMute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetVolumeStepInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub VolumeStepUp: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub VolumeStepDown: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub QueryHardwareSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetVolumeRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32, *mut f32) -> windows_core::HRESULT,
}
pub trait IAudioEndpointVolume_Impl: windows_core::IUnknownImpl {
    fn RegisterControlChangeNotify(&self, pnotify: windows_core::Ref<IAudioEndpointVolumeCallback>) -> windows_core::Result<()>;
    fn UnregisterControlChangeNotify(&self, pnotify: windows_core::Ref<IAudioEndpointVolumeCallback>) -> windows_core::Result<()>;
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn SetMasterVolumeLevel(&self, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetMasterVolumeLevelScalar(&self, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetMasterVolumeLevel(&self) -> windows_core::Result<f32>;
    fn GetMasterVolumeLevelScalar(&self) -> windows_core::Result<f32>;
    fn SetChannelVolumeLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetChannelVolumeLevelScalar(&self, nchannel: u32, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetChannelVolumeLevel(&self, nchannel: u32) -> windows_core::Result<f32>;
    fn GetChannelVolumeLevelScalar(&self, nchannel: u32) -> windows_core::Result<f32>;
    fn SetMute(&self, bmute: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetMute(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetVolumeStepInfo(&self, pnstep: *mut u32, pnstepcount: *mut u32) -> windows_core::Result<()>;
    fn VolumeStepUp(&self, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn VolumeStepDown(&self, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn QueryHardwareSupport(&self) -> windows_core::Result<u32>;
    fn GetVolumeRange(&self, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::Result<()>;
}
impl IAudioEndpointVolume_Vtbl {
    pub const fn new<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterControlChangeNotify<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotify: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::RegisterControlChangeNotify(this, core::mem::transmute_copy(&pnotify)).into()
            }
        }
        unsafe extern "system" fn UnregisterControlChangeNotify<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotify: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::UnregisterControlChangeNotify(this, core::mem::transmute_copy(&pnotify)).into()
            }
        }
        unsafe extern "system" fn GetChannelCount<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnchannelcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointVolume_Impl::GetChannelCount(this) {
                    Ok(ok__) => {
                        pnchannelcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMasterVolumeLevel<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::SetMasterVolumeLevel(this, core::mem::transmute_copy(&fleveldb), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn SetMasterVolumeLevelScalar<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::SetMasterVolumeLevelScalar(this, core::mem::transmute_copy(&flevel), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn GetMasterVolumeLevel<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfleveldb: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointVolume_Impl::GetMasterVolumeLevel(this) {
                    Ok(ok__) => {
                        pfleveldb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMasterVolumeLevelScalar<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointVolume_Impl::GetMasterVolumeLevelScalar(this) {
                    Ok(ok__) => {
                        pflevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetChannelVolumeLevel<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::SetChannelVolumeLevel(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&fleveldb), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn SetChannelVolumeLevelScalar<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::SetChannelVolumeLevelScalar(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&flevel), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn GetChannelVolumeLevel<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointVolume_Impl::GetChannelVolumeLevel(this, core::mem::transmute_copy(&nchannel)) {
                    Ok(ok__) => {
                        pfleveldb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChannelVolumeLevelScalar<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointVolume_Impl::GetChannelVolumeLevelScalar(this, core::mem::transmute_copy(&nchannel)) {
                    Ok(ok__) => {
                        pflevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMute<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmute: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::SetMute(this, core::mem::transmute_copy(&bmute), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn GetMute<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmute: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointVolume_Impl::GetMute(this) {
                    Ok(ok__) => {
                        pbmute.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVolumeStepInfo<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnstep: *mut u32, pnstepcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::GetVolumeStepInfo(this, core::mem::transmute_copy(&pnstep), core::mem::transmute_copy(&pnstepcount)).into()
            }
        }
        unsafe extern "system" fn VolumeStepUp<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::VolumeStepUp(this, core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn VolumeStepDown<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::VolumeStepDown(this, core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn QueryHardwareSupport<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointVolume_Impl::QueryHardwareSupport(this) {
                    Ok(ok__) => {
                        pdwhardwaresupportmask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVolumeRange<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::GetVolumeRange(this, core::mem::transmute_copy(&pflvolumemindb), core::mem::transmute_copy(&pflvolumemaxdb), core::mem::transmute_copy(&pflvolumeincrementdb)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterControlChangeNotify: RegisterControlChangeNotify::<Identity, OFFSET>,
            UnregisterControlChangeNotify: UnregisterControlChangeNotify::<Identity, OFFSET>,
            GetChannelCount: GetChannelCount::<Identity, OFFSET>,
            SetMasterVolumeLevel: SetMasterVolumeLevel::<Identity, OFFSET>,
            SetMasterVolumeLevelScalar: SetMasterVolumeLevelScalar::<Identity, OFFSET>,
            GetMasterVolumeLevel: GetMasterVolumeLevel::<Identity, OFFSET>,
            GetMasterVolumeLevelScalar: GetMasterVolumeLevelScalar::<Identity, OFFSET>,
            SetChannelVolumeLevel: SetChannelVolumeLevel::<Identity, OFFSET>,
            SetChannelVolumeLevelScalar: SetChannelVolumeLevelScalar::<Identity, OFFSET>,
            GetChannelVolumeLevel: GetChannelVolumeLevel::<Identity, OFFSET>,
            GetChannelVolumeLevelScalar: GetChannelVolumeLevelScalar::<Identity, OFFSET>,
            SetMute: SetMute::<Identity, OFFSET>,
            GetMute: GetMute::<Identity, OFFSET>,
            GetVolumeStepInfo: GetVolumeStepInfo::<Identity, OFFSET>,
            VolumeStepUp: VolumeStepUp::<Identity, OFFSET>,
            VolumeStepDown: VolumeStepDown::<Identity, OFFSET>,
            QueryHardwareSupport: QueryHardwareSupport::<Identity, OFFSET>,
            GetVolumeRange: GetVolumeRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointVolume as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioEndpointVolume {}
windows_core::imp::define_interface!(IAudioEndpointVolumeCallback, IAudioEndpointVolumeCallback_Vtbl, 0x657804fa_d6ad_4496_8a60_352752af4f89);
windows_core::imp::interface_hierarchy!(IAudioEndpointVolumeCallback, windows_core::IUnknown);
impl IAudioEndpointVolumeCallback {
    pub unsafe fn OnNotify(&self, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnNotify)(windows_core::Interface::as_raw(self), pnotify as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointVolumeCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> windows_core::HRESULT,
}
pub trait IAudioEndpointVolumeCallback_Impl: windows_core::IUnknownImpl {
    fn OnNotify(&self, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> windows_core::Result<()>;
}
impl IAudioEndpointVolumeCallback_Vtbl {
    pub const fn new<Identity: IAudioEndpointVolumeCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnNotify<Identity: IAudioEndpointVolumeCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolumeCallback_Impl::OnNotify(this, core::mem::transmute_copy(&pnotify)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNotify: OnNotify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointVolumeCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioEndpointVolumeCallback {}
windows_core::imp::define_interface!(IAudioEndpointVolumeEx, IAudioEndpointVolumeEx_Vtbl, 0x66e11784_f695_4f28_a505_a7080081a78f);
impl core::ops::Deref for IAudioEndpointVolumeEx {
    type Target = IAudioEndpointVolume;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioEndpointVolumeEx, windows_core::IUnknown, IAudioEndpointVolume);
impl IAudioEndpointVolumeEx {
    pub unsafe fn GetVolumeRangeChannel(&self, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetVolumeRangeChannel)(windows_core::Interface::as_raw(self), ichannel, pflvolumemindb as _, pflvolumemaxdb as _, pflvolumeincrementdb as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointVolumeEx_Vtbl {
    pub base__: IAudioEndpointVolume_Vtbl,
    pub GetVolumeRangeChannel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, *mut f32, *mut f32) -> windows_core::HRESULT,
}
pub trait IAudioEndpointVolumeEx_Impl: IAudioEndpointVolume_Impl {
    fn GetVolumeRangeChannel(&self, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::Result<()>;
}
impl IAudioEndpointVolumeEx_Vtbl {
    pub const fn new<Identity: IAudioEndpointVolumeEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVolumeRangeChannel<Identity: IAudioEndpointVolumeEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolumeEx_Impl::GetVolumeRangeChannel(this, core::mem::transmute_copy(&ichannel), core::mem::transmute_copy(&pflvolumemindb), core::mem::transmute_copy(&pflvolumemaxdb), core::mem::transmute_copy(&pflvolumeincrementdb)).into()
            }
        }
        Self { base__: IAudioEndpointVolume_Vtbl::new::<Identity, OFFSET>(), GetVolumeRangeChannel: GetVolumeRangeChannel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointVolumeEx as windows_core::Interface>::IID || iid == &<IAudioEndpointVolume as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioEndpointVolumeEx {}
windows_core::imp::define_interface!(IAudioLfxControl, IAudioLfxControl_Vtbl, 0x076a6922_d802_4f83_baf6_409d9ca11bfe);
windows_core::imp::interface_hierarchy!(IAudioLfxControl, windows_core::IUnknown);
impl IAudioLfxControl {
    pub unsafe fn SetLocalEffectsState(&self, benabled: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLocalEffectsState)(windows_core::Interface::as_raw(self), benabled.into()).ok() }
    }
    pub unsafe fn GetLocalEffectsState(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalEffectsState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioLfxControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLocalEffectsState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLocalEffectsState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAudioLfxControl_Impl: windows_core::IUnknownImpl {
    fn SetLocalEffectsState(&self, benabled: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLocalEffectsState(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAudioLfxControl_Vtbl {
    pub const fn new<Identity: IAudioLfxControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLocalEffectsState<Identity: IAudioLfxControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioLfxControl_Impl::SetLocalEffectsState(this, core::mem::transmute_copy(&benabled)).into()
            }
        }
        unsafe extern "system" fn GetLocalEffectsState<Identity: IAudioLfxControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioLfxControl_Impl::GetLocalEffectsState(this) {
                    Ok(ok__) => {
                        pbenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLocalEffectsState: SetLocalEffectsState::<Identity, OFFSET>,
            GetLocalEffectsState: GetLocalEffectsState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioLfxControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioLfxControl {}
windows_core::imp::define_interface!(IAudioMeterInformation, IAudioMeterInformation_Vtbl, 0xc02216f6_8c67_4b5b_9d00_d008e73e0064);
windows_core::imp::interface_hierarchy!(IAudioMeterInformation, windows_core::IUnknown);
impl IAudioMeterInformation {
    pub unsafe fn GetPeakValue(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPeakValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMeteringChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMeteringChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChannelsPeakValues(&self, afpeakvalues: &mut [f32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetChannelsPeakValues)(windows_core::Interface::as_raw(self), afpeakvalues.len().try_into().unwrap(), core::mem::transmute(afpeakvalues.as_ptr())).ok() }
    }
    pub unsafe fn QueryHardwareSupport(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryHardwareSupport)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMeterInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPeakValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetMeteringChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetChannelsPeakValues: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub QueryHardwareSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IAudioMeterInformation_Impl: windows_core::IUnknownImpl {
    fn GetPeakValue(&self) -> windows_core::Result<f32>;
    fn GetMeteringChannelCount(&self) -> windows_core::Result<u32>;
    fn GetChannelsPeakValues(&self, u32channelcount: u32, afpeakvalues: *mut f32) -> windows_core::Result<()>;
    fn QueryHardwareSupport(&self) -> windows_core::Result<u32>;
}
impl IAudioMeterInformation_Vtbl {
    pub const fn new<Identity: IAudioMeterInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPeakValue<Identity: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfpeak: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioMeterInformation_Impl::GetPeakValue(this) {
                    Ok(ok__) => {
                        pfpeak.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMeteringChannelCount<Identity: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnchannelcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioMeterInformation_Impl::GetMeteringChannelCount(this) {
                    Ok(ok__) => {
                        pnchannelcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChannelsPeakValues<Identity: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32channelcount: u32, afpeakvalues: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioMeterInformation_Impl::GetChannelsPeakValues(this, core::mem::transmute_copy(&u32channelcount), core::mem::transmute_copy(&afpeakvalues)).into()
            }
        }
        unsafe extern "system" fn QueryHardwareSupport<Identity: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioMeterInformation_Impl::QueryHardwareSupport(this) {
                    Ok(ok__) => {
                        pdwhardwaresupportmask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPeakValue: GetPeakValue::<Identity, OFFSET>,
            GetMeteringChannelCount: GetMeteringChannelCount::<Identity, OFFSET>,
            GetChannelsPeakValues: GetChannelsPeakValues::<Identity, OFFSET>,
            QueryHardwareSupport: QueryHardwareSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioMeterInformation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioMeterInformation {}
windows_core::imp::define_interface!(IHardwareAudioEngineBase, IHardwareAudioEngineBase_Vtbl, 0xeddce3e4_f3c1_453a_b461_223563cbd886);
windows_core::imp::interface_hierarchy!(IHardwareAudioEngineBase, windows_core::IUnknown);
impl IHardwareAudioEngineBase {
    pub unsafe fn GetAvailableOffloadConnectorCount<P0>(&self, _pwstrdeviceid: P0, _uconnectorid: u32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAvailableOffloadConnectorCount)(windows_core::Interface::as_raw(self), _pwstrdeviceid.param().abi(), _uconnectorid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEngineFormat<P0>(&self, pdevice: P0, _brequestdeviceformat: bool, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IMMDevice>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetEngineFormat)(windows_core::Interface::as_raw(self), pdevice.param().abi(), _brequestdeviceformat.into(), _ppwfxformat as _).ok() }
    }
    pub unsafe fn SetEngineDeviceFormat<P0>(&self, pdevice: P0, _pwfxformat: *mut super::WAVEFORMATEX) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IMMDevice>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEngineDeviceFormat)(windows_core::Interface::as_raw(self), pdevice.param().abi(), _pwfxformat as _).ok() }
    }
    pub unsafe fn SetGfxState<P0>(&self, pdevice: P0, _benable: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IMMDevice>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetGfxState)(windows_core::Interface::as_raw(self), pdevice.param().abi(), _benable.into()).ok() }
    }
    pub unsafe fn GetGfxState<P0>(&self, pdevice: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<super::IMMDevice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGfxState)(windows_core::Interface::as_raw(self), pdevice.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHardwareAudioEngineBase_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAvailableOffloadConnectorCount: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetEngineFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT,
    pub SetEngineDeviceFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::WAVEFORMATEX) -> windows_core::HRESULT,
    pub SetGfxState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetGfxState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IHardwareAudioEngineBase_Impl: windows_core::IUnknownImpl {
    fn GetAvailableOffloadConnectorCount(&self, _pwstrdeviceid: &windows_core::PCWSTR, _uconnectorid: u32) -> windows_core::Result<u32>;
    fn GetEngineFormat(&self, pdevice: windows_core::Ref<super::IMMDevice>, _brequestdeviceformat: windows_core::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> windows_core::Result<()>;
    fn SetEngineDeviceFormat(&self, pdevice: windows_core::Ref<super::IMMDevice>, _pwfxformat: *mut super::WAVEFORMATEX) -> windows_core::Result<()>;
    fn SetGfxState(&self, pdevice: windows_core::Ref<super::IMMDevice>, _benable: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetGfxState(&self, pdevice: windows_core::Ref<super::IMMDevice>) -> windows_core::Result<windows_core::BOOL>;
}
impl IHardwareAudioEngineBase_Vtbl {
    pub const fn new<Identity: IHardwareAudioEngineBase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAvailableOffloadConnectorCount<Identity: IHardwareAudioEngineBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, _pwstrdeviceid: windows_core::PCWSTR, _uconnectorid: u32, _pavailableconnectorinstancecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHardwareAudioEngineBase_Impl::GetAvailableOffloadConnectorCount(this, core::mem::transmute(&_pwstrdeviceid), core::mem::transmute_copy(&_uconnectorid)) {
                    Ok(ok__) => {
                        _pavailableconnectorinstancecount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEngineFormat<Identity: IHardwareAudioEngineBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, _brequestdeviceformat: windows_core::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHardwareAudioEngineBase_Impl::GetEngineFormat(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&_brequestdeviceformat), core::mem::transmute_copy(&_ppwfxformat)).into()
            }
        }
        unsafe extern "system" fn SetEngineDeviceFormat<Identity: IHardwareAudioEngineBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, _pwfxformat: *mut super::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHardwareAudioEngineBase_Impl::SetEngineDeviceFormat(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&_pwfxformat)).into()
            }
        }
        unsafe extern "system" fn SetGfxState<Identity: IHardwareAudioEngineBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, _benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHardwareAudioEngineBase_Impl::SetGfxState(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&_benable)).into()
            }
        }
        unsafe extern "system" fn GetGfxState<Identity: IHardwareAudioEngineBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, _pbenable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHardwareAudioEngineBase_Impl::GetGfxState(this, core::mem::transmute_copy(&pdevice)) {
                    Ok(ok__) => {
                        _pbenable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAvailableOffloadConnectorCount: GetAvailableOffloadConnectorCount::<Identity, OFFSET>,
            GetEngineFormat: GetEngineFormat::<Identity, OFFSET>,
            SetEngineDeviceFormat: SetEngineDeviceFormat::<Identity, OFFSET>,
            SetGfxState: SetGfxState::<Identity, OFFSET>,
            GetGfxState: GetGfxState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHardwareAudioEngineBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHardwareAudioEngineBase {}
pub const eConnectorCount: EndpointConnectorType = EndpointConnectorType(4i32);
pub const eHostProcessConnector: EndpointConnectorType = EndpointConnectorType(0i32);
pub const eKeywordDetectorConnector: EndpointConnectorType = EndpointConnectorType(3i32);
pub const eLoopbackConnector: EndpointConnectorType = EndpointConnectorType(2i32);
pub const eOffloadConnector: EndpointConnectorType = EndpointConnectorType(1i32);
