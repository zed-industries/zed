windows_core::imp::define_interface!(IAudioEndpointFormatControl, IAudioEndpointFormatControl_Vtbl, 0x784cfd40_9f89_456e_a1a6_873b006a664e);
impl core::ops::Deref for IAudioEndpointFormatControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioEndpointFormatControl, windows_core::IUnknown);
impl IAudioEndpointFormatControl {
    pub unsafe fn ResetToDefault(&self, resetflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetToDefault)(windows_core::Interface::as_raw(self), resetflags).ok()
    }
}
#[repr(C)]
pub struct IAudioEndpointFormatControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ResetToDefault: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioEndpointLastBufferControl, IAudioEndpointLastBufferControl_Vtbl, 0xf8520dd3_8f9d_4437_9861_62f584c33dd6);
impl core::ops::Deref for IAudioEndpointLastBufferControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioEndpointLastBufferControl, windows_core::IUnknown);
impl IAudioEndpointLastBufferControl {
    pub unsafe fn IsLastBufferControlSupported(&self) -> super::super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsLastBufferControlSupported)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub unsafe fn ReleaseOutputDataPointerForLastBuffer(&self, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY) {
        (windows_core::Interface::vtable(self).ReleaseOutputDataPointerForLastBuffer)(windows_core::Interface::as_raw(self), pconnectionproperty)
    }
}
#[repr(C)]
pub struct IAudioEndpointLastBufferControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsLastBufferControlSupported: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub ReleaseOutputDataPointerForLastBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Apo::APO_CONNECTION_PROPERTY),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))]
    ReleaseOutputDataPointerForLastBuffer: usize,
}
windows_core::imp::define_interface!(IAudioEndpointOffloadStreamMeter, IAudioEndpointOffloadStreamMeter_Vtbl, 0xe1546dce_9dd1_418b_9ab2_348ced161c86);
impl core::ops::Deref for IAudioEndpointOffloadStreamMeter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioEndpointOffloadStreamMeter, windows_core::IUnknown);
impl IAudioEndpointOffloadStreamMeter {
    pub unsafe fn GetMeterChannelCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMeterChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMeteringData(&self, u32channelcount: u32) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMeteringData)(windows_core::Interface::as_raw(self), u32channelcount, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAudioEndpointOffloadStreamMeter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMeterChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMeteringData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioEndpointOffloadStreamMute, IAudioEndpointOffloadStreamMute_Vtbl, 0xdfe21355_5ec2_40e0_8d6b_710ac3c00249);
impl core::ops::Deref for IAudioEndpointOffloadStreamMute {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioEndpointOffloadStreamMute, windows_core::IUnknown);
impl IAudioEndpointOffloadStreamMute {
    pub unsafe fn SetMute(&self, bmuted: u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMute)(windows_core::Interface::as_raw(self), bmuted).ok()
    }
    pub unsafe fn GetMute(&self) -> windows_core::Result<u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMute)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAudioEndpointOffloadStreamMute_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMute: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub GetMute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioEndpointOffloadStreamVolume, IAudioEndpointOffloadStreamVolume_Vtbl, 0x64f1dd49_71ca_4281_8672_3a9eddd1d0b6);
impl core::ops::Deref for IAudioEndpointOffloadStreamVolume {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioEndpointOffloadStreamVolume, windows_core::IUnknown);
impl IAudioEndpointOffloadStreamVolume {
    pub unsafe fn GetVolumeChannelCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVolumeChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Media_KernelStreaming")]
    pub unsafe fn SetChannelVolumes(&self, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetChannelVolumes)(windows_core::Interface::as_raw(self), u32channelcount, pf32volumes, u32curvetype, pcurveduration).ok()
    }
    pub unsafe fn GetChannelVolumes(&self, u32channelcount: u32) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChannelVolumes)(windows_core::Interface::as_raw(self), u32channelcount, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAudioEndpointOffloadStreamVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVolumeChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Media_KernelStreaming")]
    pub SetChannelVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, super::super::KernelStreaming::AUDIO_CURVE_TYPE, *const i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_KernelStreaming"))]
    SetChannelVolumes: usize,
    pub GetChannelVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioEndpointVolume, IAudioEndpointVolume_Vtbl, 0x5cdf2c82_841e_4546_9722_0cf74078229a);
impl core::ops::Deref for IAudioEndpointVolume {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioEndpointVolume, windows_core::IUnknown);
impl IAudioEndpointVolume {
    pub unsafe fn RegisterControlChangeNotify<P0>(&self, pnotify: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioEndpointVolumeCallback>,
    {
        (windows_core::Interface::vtable(self).RegisterControlChangeNotify)(windows_core::Interface::as_raw(self), pnotify.param().abi()).ok()
    }
    pub unsafe fn UnregisterControlChangeNotify<P0>(&self, pnotify: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioEndpointVolumeCallback>,
    {
        (windows_core::Interface::vtable(self).UnregisterControlChangeNotify)(windows_core::Interface::as_raw(self), pnotify.param().abi()).ok()
    }
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMasterVolumeLevel(&self, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMasterVolumeLevel)(windows_core::Interface::as_raw(self), fleveldb, pguideventcontext).ok()
    }
    pub unsafe fn SetMasterVolumeLevelScalar(&self, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMasterVolumeLevelScalar)(windows_core::Interface::as_raw(self), flevel, pguideventcontext).ok()
    }
    pub unsafe fn GetMasterVolumeLevel(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMasterVolumeLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMasterVolumeLevelScalar(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMasterVolumeLevelScalar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetChannelVolumeLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetChannelVolumeLevel)(windows_core::Interface::as_raw(self), nchannel, fleveldb, pguideventcontext).ok()
    }
    pub unsafe fn SetChannelVolumeLevelScalar(&self, nchannel: u32, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetChannelVolumeLevelScalar)(windows_core::Interface::as_raw(self), nchannel, flevel, pguideventcontext).ok()
    }
    pub unsafe fn GetChannelVolumeLevel(&self, nchannel: u32) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChannelVolumeLevel)(windows_core::Interface::as_raw(self), nchannel, &mut result__).map(|| result__)
    }
    pub unsafe fn GetChannelVolumeLevelScalar(&self, nchannel: u32) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChannelVolumeLevelScalar)(windows_core::Interface::as_raw(self), nchannel, &mut result__).map(|| result__)
    }
    pub unsafe fn SetMute<P0>(&self, bmute: P0, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetMute)(windows_core::Interface::as_raw(self), bmute.param().abi(), pguideventcontext).ok()
    }
    pub unsafe fn GetMute(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMute)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetVolumeStepInfo(&self, pnstep: *mut u32, pnstepcount: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVolumeStepInfo)(windows_core::Interface::as_raw(self), pnstep, pnstepcount).ok()
    }
    pub unsafe fn VolumeStepUp(&self, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).VolumeStepUp)(windows_core::Interface::as_raw(self), pguideventcontext).ok()
    }
    pub unsafe fn VolumeStepDown(&self, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).VolumeStepDown)(windows_core::Interface::as_raw(self), pguideventcontext).ok()
    }
    pub unsafe fn QueryHardwareSupport(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryHardwareSupport)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetVolumeRange(&self, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetVolumeRange)(windows_core::Interface::as_raw(self), pflvolumemindb, pflvolumemaxdb, pflvolumeincrementdb).ok()
    }
}
#[repr(C)]
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
    pub SetMute: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetMute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetVolumeStepInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub VolumeStepUp: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub VolumeStepDown: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub QueryHardwareSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetVolumeRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioEndpointVolumeCallback, IAudioEndpointVolumeCallback_Vtbl, 0x657804fa_d6ad_4496_8a60_352752af4f89);
impl core::ops::Deref for IAudioEndpointVolumeCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioEndpointVolumeCallback, windows_core::IUnknown);
impl IAudioEndpointVolumeCallback {
    pub unsafe fn OnNotify(&self, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnNotify)(windows_core::Interface::as_raw(self), pnotify).ok()
    }
}
#[repr(C)]
pub struct IAudioEndpointVolumeCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).GetVolumeRangeChannel)(windows_core::Interface::as_raw(self), ichannel, pflvolumemindb, pflvolumemaxdb, pflvolumeincrementdb).ok()
    }
}
#[repr(C)]
pub struct IAudioEndpointVolumeEx_Vtbl {
    pub base__: IAudioEndpointVolume_Vtbl,
    pub GetVolumeRangeChannel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, *mut f32, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioLfxControl, IAudioLfxControl_Vtbl, 0x076a6922_d802_4f83_baf6_409d9ca11bfe);
impl core::ops::Deref for IAudioLfxControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioLfxControl, windows_core::IUnknown);
impl IAudioLfxControl {
    pub unsafe fn SetLocalEffectsState<P0>(&self, benabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLocalEffectsState)(windows_core::Interface::as_raw(self), benabled.param().abi()).ok()
    }
    pub unsafe fn GetLocalEffectsState(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLocalEffectsState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAudioLfxControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLocalEffectsState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetLocalEffectsState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioMeterInformation, IAudioMeterInformation_Vtbl, 0xc02216f6_8c67_4b5b_9d00_d008e73e0064);
impl core::ops::Deref for IAudioMeterInformation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioMeterInformation, windows_core::IUnknown);
impl IAudioMeterInformation {
    pub unsafe fn GetPeakValue(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPeakValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMeteringChannelCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMeteringChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetChannelsPeakValues(&self, afpeakvalues: &mut [f32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetChannelsPeakValues)(windows_core::Interface::as_raw(self), afpeakvalues.len().try_into().unwrap(), core::mem::transmute(afpeakvalues.as_ptr())).ok()
    }
    pub unsafe fn QueryHardwareSupport(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryHardwareSupport)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAudioMeterInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPeakValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetMeteringChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetChannelsPeakValues: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub QueryHardwareSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHardwareAudioEngineBase, IHardwareAudioEngineBase_Vtbl, 0xeddce3e4_f3c1_453a_b461_223563cbd886);
impl core::ops::Deref for IHardwareAudioEngineBase {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHardwareAudioEngineBase, windows_core::IUnknown);
impl IHardwareAudioEngineBase {
    pub unsafe fn GetAvailableOffloadConnectorCount<P0>(&self, _pwstrdeviceid: P0, _uconnectorid: u32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAvailableOffloadConnectorCount)(windows_core::Interface::as_raw(self), _pwstrdeviceid.param().abi(), _uconnectorid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetEngineFormat<P0, P1>(&self, pdevice: P0, _brequestdeviceformat: P1, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IMMDevice>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetEngineFormat)(windows_core::Interface::as_raw(self), pdevice.param().abi(), _brequestdeviceformat.param().abi(), _ppwfxformat).ok()
    }
    pub unsafe fn SetEngineDeviceFormat<P0>(&self, pdevice: P0, _pwfxformat: *mut super::WAVEFORMATEX) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IMMDevice>,
    {
        (windows_core::Interface::vtable(self).SetEngineDeviceFormat)(windows_core::Interface::as_raw(self), pdevice.param().abi(), _pwfxformat).ok()
    }
    pub unsafe fn SetGfxState<P0, P1>(&self, pdevice: P0, _benable: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IMMDevice>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetGfxState)(windows_core::Interface::as_raw(self), pdevice.param().abi(), _benable.param().abi()).ok()
    }
    pub unsafe fn GetGfxState<P0>(&self, pdevice: P0) -> windows_core::Result<super::super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<super::IMMDevice>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGfxState)(windows_core::Interface::as_raw(self), pdevice.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IHardwareAudioEngineBase_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAvailableOffloadConnectorCount: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetEngineFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::BOOL, *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT,
    pub SetEngineDeviceFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::WAVEFORMATEX) -> windows_core::HRESULT,
    pub SetGfxState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetGfxState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin2_FactoryCLSID: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_DataFlow: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_FactoryCLSID: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 1 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_AudioEndpointPlugin_PnPInterface: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x12d83bd7_cf12_46be_8540_812710d3021c), pid: 3 };
pub const eConnectorCount: EndpointConnectorType = EndpointConnectorType(4i32);
pub const eHostProcessConnector: EndpointConnectorType = EndpointConnectorType(0i32);
pub const eKeywordDetectorConnector: EndpointConnectorType = EndpointConnectorType(3i32);
pub const eLoopbackConnector: EndpointConnectorType = EndpointConnectorType(2i32);
pub const eOffloadConnector: EndpointConnectorType = EndpointConnectorType(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EndpointConnectorType(pub i32);
impl windows_core::TypeKind for EndpointConnectorType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EndpointConnectorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EndpointConnectorType").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    pub u32Size: u32,
    pub u32TSSessionId: u32,
    pub targetEndpointConnectorType: EndpointConnectorType,
    pub wfxDeviceFormat: super::WAVEFORMATEX,
}
impl windows_core::TypeKind for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIO_ENDPOINT_SHARED_CREATE_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEVINTERFACE_AUDIOENDPOINTPLUGIN: windows_core::GUID = windows_core::GUID::from_u128(0x9f2f7b66_65ac_4fa6_8ae4_123c78b89313);
#[cfg(feature = "implement")]
core::include!("impl.rs");
