windows_core::imp::define_interface!(IApoAcousticEchoCancellation, IApoAcousticEchoCancellation_Vtbl, 0x25385759_3236_4101_a943_25693dfb5d2d);
impl core::ops::Deref for IApoAcousticEchoCancellation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IApoAcousticEchoCancellation, windows_core::IUnknown);
impl IApoAcousticEchoCancellation {}
#[repr(C)]
pub struct IApoAcousticEchoCancellation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(IApoAuxiliaryInputConfiguration, IApoAuxiliaryInputConfiguration_Vtbl, 0x4ceb0aab_fa19_48ed_a857_87771ae1b768);
impl core::ops::Deref for IApoAuxiliaryInputConfiguration {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IApoAuxiliaryInputConfiguration, windows_core::IUnknown);
impl IApoAuxiliaryInputConfiguration {
    pub unsafe fn AddAuxiliaryInput(&self, dwinputid: u32, pbydata: &[u8], pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddAuxiliaryInput)(windows_core::Interface::as_raw(self), dwinputid, pbydata.len().try_into().unwrap(), core::mem::transmute(pbydata.as_ptr()), pinputconnection).ok()
    }
    pub unsafe fn RemoveAuxiliaryInput(&self, dwinputid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAuxiliaryInput)(windows_core::Interface::as_raw(self), dwinputid).ok()
    }
    pub unsafe fn IsInputFormatSupported<P0>(&self, prequestedinputformat: P0) -> windows_core::Result<IAudioMediaType>
    where
        P0: windows_core::Param<IAudioMediaType>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsInputFormatSupported)(windows_core::Interface::as_raw(self), prequestedinputformat.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IApoAuxiliaryInputConfiguration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddAuxiliaryInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8, *const APO_CONNECTION_DESCRIPTOR) -> windows_core::HRESULT,
    pub RemoveAuxiliaryInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsInputFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApoAuxiliaryInputRT, IApoAuxiliaryInputRT_Vtbl, 0xf851809c_c177_49a0_b1b2_b66f017943ab);
impl core::ops::Deref for IApoAuxiliaryInputRT {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IApoAuxiliaryInputRT, windows_core::IUnknown);
impl IApoAuxiliaryInputRT {
    pub unsafe fn AcceptInput(&self, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY) {
        (windows_core::Interface::vtable(self).AcceptInput)(windows_core::Interface::as_raw(self), dwinputid, pinputconnection)
    }
}
#[repr(C)]
pub struct IApoAuxiliaryInputRT_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AcceptInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const APO_CONNECTION_PROPERTY),
}
windows_core::imp::define_interface!(IAudioDeviceModulesClient, IAudioDeviceModulesClient_Vtbl, 0x98f37dac_d0b6_49f5_896a_aa4d169a4c48);
impl core::ops::Deref for IAudioDeviceModulesClient {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioDeviceModulesClient, windows_core::IUnknown);
impl IAudioDeviceModulesClient {
    pub unsafe fn SetAudioDeviceModulesManager<P0>(&self, paudiodevicemodulesmanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetAudioDeviceModulesManager)(windows_core::Interface::as_raw(self), paudiodevicemodulesmanager.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAudioDeviceModulesClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAudioDeviceModulesManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioMediaType, IAudioMediaType_Vtbl, 0x4e997f73_b71f_4798_873b_ed7dfcf15b4d);
impl core::ops::Deref for IAudioMediaType {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioMediaType, windows_core::IUnknown);
impl IAudioMediaType {
    pub unsafe fn IsCompressedFormat(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsCompressedFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsEqual<P0>(&self, piaudiotype: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IAudioMediaType>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), piaudiotype.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAudioFormat(&self) -> *mut super::WAVEFORMATEX {
        (windows_core::Interface::vtable(self).GetAudioFormat)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetUncompressedAudioFormat(&self, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUncompressedAudioFormat)(windows_core::Interface::as_raw(self), puncompressedaudioformat).ok()
    }
}
#[repr(C)]
pub struct IAudioMediaType_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsCompressedFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAudioFormat: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut super::WAVEFORMATEX,
    pub GetUncompressedAudioFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UNCOMPRESSEDAUDIOFORMAT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioProcessingObject, IAudioProcessingObject_Vtbl, 0xfd7f2b29_24d0_4b5c_b177_592c39f9ca10);
impl core::ops::Deref for IAudioProcessingObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioProcessingObject, windows_core::IUnknown);
impl IAudioProcessingObject {
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetLatency(&self) -> windows_core::Result<i64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetRegistrationProperties(&self) -> windows_core::Result<*mut APO_REG_PROPERTIES> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRegistrationProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Initialize(&self, pbydata: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pbydata.len().try_into().unwrap(), core::mem::transmute(pbydata.as_ptr())).ok()
    }
    pub unsafe fn IsInputFormatSupported<P0, P1>(&self, poppositeformat: P0, prequestedinputformat: P1) -> windows_core::Result<IAudioMediaType>
    where
        P0: windows_core::Param<IAudioMediaType>,
        P1: windows_core::Param<IAudioMediaType>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsInputFormatSupported)(windows_core::Interface::as_raw(self), poppositeformat.param().abi(), prequestedinputformat.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsOutputFormatSupported<P0, P1>(&self, poppositeformat: P0, prequestedoutputformat: P1) -> windows_core::Result<IAudioMediaType>
    where
        P0: windows_core::Param<IAudioMediaType>,
        P1: windows_core::Param<IAudioMediaType>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsOutputFormatSupported)(windows_core::Interface::as_raw(self), poppositeformat.param().abi(), prequestedoutputformat.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetInputChannelCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInputChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAudioProcessingObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub GetRegistrationProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut APO_REG_PROPERTIES) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub IsInputFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOutputFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInputChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioProcessingObjectConfiguration, IAudioProcessingObjectConfiguration_Vtbl, 0x0e5ed805_aba6_49c3_8f9a_2b8c889c4fa8);
impl core::ops::Deref for IAudioProcessingObjectConfiguration {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioProcessingObjectConfiguration, windows_core::IUnknown);
impl IAudioProcessingObjectConfiguration {
    pub unsafe fn LockForProcess(&self, ppinputconnections: &[*const APO_CONNECTION_DESCRIPTOR], ppoutputconnections: &[*const APO_CONNECTION_DESCRIPTOR]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LockForProcess)(windows_core::Interface::as_raw(self), ppinputconnections.len().try_into().unwrap(), core::mem::transmute(ppinputconnections.as_ptr()), ppoutputconnections.len().try_into().unwrap(), core::mem::transmute(ppoutputconnections.as_ptr())).ok()
    }
    pub unsafe fn UnlockForProcess(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnlockForProcess)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IAudioProcessingObjectConfiguration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LockForProcess: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *const APO_CONNECTION_DESCRIPTOR, u32, *const *const APO_CONNECTION_DESCRIPTOR) -> windows_core::HRESULT,
    pub UnlockForProcess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioProcessingObjectLoggingService, IAudioProcessingObjectLoggingService_Vtbl, 0x698f0107_1745_4708_95a5_d84478a62a65);
impl core::ops::Deref for IAudioProcessingObjectLoggingService {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioProcessingObjectLoggingService, windows_core::IUnknown);
impl IAudioProcessingObjectLoggingService {
    pub unsafe fn ApoLog<P0>(&self, level: APO_LOG_LEVEL, format: P0)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ApoLog)(windows_core::Interface::as_raw(self), level, format.param().abi())
    }
}
#[repr(C)]
pub struct IAudioProcessingObjectLoggingService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ApoLog: unsafe extern "system" fn(*mut core::ffi::c_void, APO_LOG_LEVEL, windows_core::PCWSTR),
}
windows_core::imp::define_interface!(IAudioProcessingObjectNotifications, IAudioProcessingObjectNotifications_Vtbl, 0x56b0c76f_02fd_4b21_a52e_9f8219fc86e4);
impl core::ops::Deref for IAudioProcessingObjectNotifications {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioProcessingObjectNotifications, windows_core::IUnknown);
impl IAudioProcessingObjectNotifications {
    pub unsafe fn GetApoNotificationRegistrationInfo(&self, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetApoNotificationRegistrationInfo)(windows_core::Interface::as_raw(self), aponotifications, count).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn HandleNotification(&self, aponotification: *const APO_NOTIFICATION) {
        (windows_core::Interface::vtable(self).HandleNotification)(windows_core::Interface::as_raw(self), aponotification)
    }
}
#[repr(C)]
pub struct IAudioProcessingObjectNotifications_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetApoNotificationRegistrationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut APO_NOTIFICATION_DESCRIPTOR, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub HandleNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *const APO_NOTIFICATION),
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    HandleNotification: usize,
}
windows_core::imp::define_interface!(IAudioProcessingObjectNotifications2, IAudioProcessingObjectNotifications2_Vtbl, 0xca2cfbde_a9d6_4eb0_bc95_c4d026b380f0);
impl core::ops::Deref for IAudioProcessingObjectNotifications2 {
    type Target = IAudioProcessingObjectNotifications;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioProcessingObjectNotifications2, windows_core::IUnknown, IAudioProcessingObjectNotifications);
impl IAudioProcessingObjectNotifications2 {
    pub unsafe fn GetApoNotificationRegistrationInfo2(&self, maxaponotificationtypesupported: APO_NOTIFICATION_TYPE, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetApoNotificationRegistrationInfo2)(windows_core::Interface::as_raw(self), maxaponotificationtypesupported, aponotifications, count).ok()
    }
}
#[repr(C)]
pub struct IAudioProcessingObjectNotifications2_Vtbl {
    pub base__: IAudioProcessingObjectNotifications_Vtbl,
    pub GetApoNotificationRegistrationInfo2: unsafe extern "system" fn(*mut core::ffi::c_void, APO_NOTIFICATION_TYPE, *mut *mut APO_NOTIFICATION_DESCRIPTOR, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioProcessingObjectRT, IAudioProcessingObjectRT_Vtbl, 0x9e1d6a6d_ddbc_4e95_a4c7_ad64ba37846c);
impl core::ops::Deref for IAudioProcessingObjectRT {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioProcessingObjectRT, windows_core::IUnknown);
impl IAudioProcessingObjectRT {
    pub unsafe fn APOProcess(&self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY) {
        (windows_core::Interface::vtable(self).APOProcess)(windows_core::Interface::as_raw(self), u32numinputconnections, ppinputconnections, u32numoutputconnections, ppoutputconnections)
    }
    pub unsafe fn CalcInputFrames(&self, u32outputframecount: u32) -> u32 {
        (windows_core::Interface::vtable(self).CalcInputFrames)(windows_core::Interface::as_raw(self), u32outputframecount)
    }
    pub unsafe fn CalcOutputFrames(&self, u32inputframecount: u32) -> u32 {
        (windows_core::Interface::vtable(self).CalcOutputFrames)(windows_core::Interface::as_raw(self), u32inputframecount)
    }
}
#[repr(C)]
pub struct IAudioProcessingObjectRT_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub APOProcess: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *const APO_CONNECTION_PROPERTY, u32, *mut *mut APO_CONNECTION_PROPERTY),
    pub CalcInputFrames: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub CalcOutputFrames: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
}
windows_core::imp::define_interface!(IAudioProcessingObjectRTQueueService, IAudioProcessingObjectRTQueueService_Vtbl, 0xacd65e2f_955b_4b57_b9bf_ac297bb752c9);
impl core::ops::Deref for IAudioProcessingObjectRTQueueService {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioProcessingObjectRTQueueService, windows_core::IUnknown);
impl IAudioProcessingObjectRTQueueService {
    pub unsafe fn GetRealTimeWorkQueue(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRealTimeWorkQueue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAudioProcessingObjectRTQueueService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRealTimeWorkQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioProcessingObjectVBR, IAudioProcessingObjectVBR_Vtbl, 0x7ba1db8f_78ad_49cd_9591_f79d80a17c81);
impl core::ops::Deref for IAudioProcessingObjectVBR {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioProcessingObjectVBR, windows_core::IUnknown);
impl IAudioProcessingObjectVBR {
    pub unsafe fn CalcMaxInputFrames(&self, u32maxoutputframecount: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CalcMaxInputFrames)(windows_core::Interface::as_raw(self), u32maxoutputframecount, &mut result__).map(|| result__)
    }
    pub unsafe fn CalcMaxOutputFrames(&self, u32maxinputframecount: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CalcMaxOutputFrames)(windows_core::Interface::as_raw(self), u32maxinputframecount, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAudioProcessingObjectVBR_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CalcMaxInputFrames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub CalcMaxOutputFrames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioSystemEffects, IAudioSystemEffects_Vtbl, 0x5fa00f27_add6_499a_8a9d_6b98521fa75b);
impl core::ops::Deref for IAudioSystemEffects {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSystemEffects, windows_core::IUnknown);
impl IAudioSystemEffects {}
#[repr(C)]
pub struct IAudioSystemEffects_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(IAudioSystemEffects2, IAudioSystemEffects2_Vtbl, 0xbafe99d2_7436_44ce_9e0e_4d89afbfff56);
impl core::ops::Deref for IAudioSystemEffects2 {
    type Target = IAudioSystemEffects;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSystemEffects2, windows_core::IUnknown, IAudioSystemEffects);
impl IAudioSystemEffects2 {
    pub unsafe fn GetEffectsList<P0>(&self, ppeffectsids: *mut *mut windows_core::GUID, pceffects: *mut u32, event: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).GetEffectsList)(windows_core::Interface::as_raw(self), ppeffectsids, pceffects, event.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAudioSystemEffects2_Vtbl {
    pub base__: IAudioSystemEffects_Vtbl,
    pub GetEffectsList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut windows_core::GUID, *mut u32, super::super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioSystemEffects3, IAudioSystemEffects3_Vtbl, 0xc58b31cd_fc6a_4255_bc1f_ad29bb0a4a17);
impl core::ops::Deref for IAudioSystemEffects3 {
    type Target = IAudioSystemEffects2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSystemEffects3, windows_core::IUnknown, IAudioSystemEffects, IAudioSystemEffects2);
impl IAudioSystemEffects3 {
    pub unsafe fn GetControllableSystemEffectsList<P0>(&self, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).GetControllableSystemEffectsList)(windows_core::Interface::as_raw(self), effects, numeffects, event.param().abi()).ok()
    }
    pub unsafe fn SetAudioSystemEffectState(&self, effectid: windows_core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAudioSystemEffectState)(windows_core::Interface::as_raw(self), core::mem::transmute(effectid), state).ok()
    }
}
#[repr(C)]
pub struct IAudioSystemEffects3_Vtbl {
    pub base__: IAudioSystemEffects2_Vtbl,
    pub GetControllableSystemEffectsList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut AUDIO_SYSTEMEFFECT, *mut u32, super::super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub SetAudioSystemEffectState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, AUDIO_SYSTEMEFFECT_STATE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioSystemEffectsCustomFormats, IAudioSystemEffectsCustomFormats_Vtbl, 0xb1176e34_bb7f_4f05_bebd_1b18a534e097);
impl core::ops::Deref for IAudioSystemEffectsCustomFormats {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSystemEffectsCustomFormats, windows_core::IUnknown);
impl IAudioSystemEffectsCustomFormats {
    pub unsafe fn GetFormatCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFormatCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFormat(&self, nformat: u32) -> windows_core::Result<IAudioMediaType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), nformat, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFormatRepresentation(&self, nformat: u32) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFormatRepresentation)(windows_core::Interface::as_raw(self), nformat, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAudioSystemEffectsCustomFormats_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFormatCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFormatRepresentation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub const APOERR_ALREADY_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x887D0001_u32 as _);
pub const APOERR_ALREADY_UNLOCKED: windows_core::HRESULT = windows_core::HRESULT(0x887D0006_u32 as _);
pub const APOERR_APO_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x887D000A_u32 as _);
pub const APOERR_BUFFERS_OVERLAP: windows_core::HRESULT = windows_core::HRESULT(0x887D0005_u32 as _);
pub const APOERR_FORMAT_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x887D0003_u32 as _);
pub const APOERR_INVALID_APO_CLSID: windows_core::HRESULT = windows_core::HRESULT(0x887D0004_u32 as _);
pub const APOERR_INVALID_COEFFCOUNT: windows_core::HRESULT = windows_core::HRESULT(0x887D000B_u32 as _);
pub const APOERR_INVALID_COEFFICIENT: windows_core::HRESULT = windows_core::HRESULT(0x887D000C_u32 as _);
pub const APOERR_INVALID_CONNECTION_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x887D0009_u32 as _);
pub const APOERR_INVALID_CURVE_PARAM: windows_core::HRESULT = windows_core::HRESULT(0x887D000D_u32 as _);
pub const APOERR_INVALID_INPUTID: windows_core::HRESULT = windows_core::HRESULT(0x887D000E_u32 as _);
pub const APOERR_INVALID_OUTPUT_MAXFRAMECOUNT: windows_core::HRESULT = windows_core::HRESULT(0x887D0008_u32 as _);
pub const APOERR_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x887D0002_u32 as _);
pub const APOERR_NUM_CONNECTIONS_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x887D0007_u32 as _);
pub const APO_CONNECTION_BUFFER_TYPE_ALLOCATED: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(0i32);
pub const APO_CONNECTION_BUFFER_TYPE_DEPENDANT: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(2i32);
pub const APO_CONNECTION_BUFFER_TYPE_EXTERNAL: APO_CONNECTION_BUFFER_TYPE = APO_CONNECTION_BUFFER_TYPE(1i32);
pub const APO_FLAG_BITSPERSAMPLE_MUST_MATCH: APO_FLAG = APO_FLAG(8i32);
pub const APO_FLAG_DEFAULT: APO_FLAG = APO_FLAG(14i32);
pub const APO_FLAG_FRAMESPERSECOND_MUST_MATCH: APO_FLAG = APO_FLAG(4i32);
pub const APO_FLAG_INPLACE: APO_FLAG = APO_FLAG(1i32);
pub const APO_FLAG_MIXER: APO_FLAG = APO_FLAG(16i32);
pub const APO_FLAG_NONE: APO_FLAG = APO_FLAG(0i32);
pub const APO_FLAG_SAMPLESPERFRAME_MUST_MATCH: APO_FLAG = APO_FLAG(2i32);
pub const APO_LOG_LEVEL_ALWAYS: APO_LOG_LEVEL = APO_LOG_LEVEL(0i32);
pub const APO_LOG_LEVEL_CRITICAL: APO_LOG_LEVEL = APO_LOG_LEVEL(1i32);
pub const APO_LOG_LEVEL_ERROR: APO_LOG_LEVEL = APO_LOG_LEVEL(2i32);
pub const APO_LOG_LEVEL_INFO: APO_LOG_LEVEL = APO_LOG_LEVEL(4i32);
pub const APO_LOG_LEVEL_VERBOSE: APO_LOG_LEVEL = APO_LOG_LEVEL(5i32);
pub const APO_LOG_LEVEL_WARNING: APO_LOG_LEVEL = APO_LOG_LEVEL(3i32);
pub const APO_NOTIFICATION_TYPE_DEVICE_ORIENTATION: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(5i32);
pub const APO_NOTIFICATION_TYPE_ENDPOINT_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(2i32);
pub const APO_NOTIFICATION_TYPE_ENDPOINT_VOLUME: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(1i32);
pub const APO_NOTIFICATION_TYPE_ENDPOINT_VOLUME2: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(4i32);
pub const APO_NOTIFICATION_TYPE_MICROPHONE_BOOST: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(6i32);
pub const APO_NOTIFICATION_TYPE_NONE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(0i32);
pub const APO_NOTIFICATION_TYPE_SYSTEM_EFFECTS_PROPERTY_CHANGE: APO_NOTIFICATION_TYPE = APO_NOTIFICATION_TYPE(3i32);
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_DATA: u32 = 4u32;
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_TYPES: u32 = 2u32;
pub const AUDIOMEDIATYPE_EQUAL_FORMAT_USER_DATA: u32 = 8u32;
pub const AUDIO_FLOW_PULL: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(0i32);
pub const AUDIO_FLOW_PUSH: AUDIO_FLOW_TYPE = AUDIO_FLOW_TYPE(1i32);
pub const AUDIO_MAX_CHANNELS: u32 = 4096u32;
pub const AUDIO_MAX_FRAMERATE: f64 = 384000f64;
pub const AUDIO_MIN_CHANNELS: u32 = 1u32;
pub const AUDIO_MIN_FRAMERATE: f64 = 10f64;
pub const AUDIO_SYSTEMEFFECT_STATE_OFF: AUDIO_SYSTEMEFFECT_STATE = AUDIO_SYSTEMEFFECT_STATE(0i32);
pub const AUDIO_SYSTEMEFFECT_STATE_ON: AUDIO_SYSTEMEFFECT_STATE = AUDIO_SYSTEMEFFECT_STATE(1i32);
pub const BUFFER_INVALID: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(0i32);
pub const BUFFER_SILENT: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(2i32);
pub const BUFFER_VALID: APO_BUFFER_FLAGS = APO_BUFFER_FLAGS(1i32);
pub const DEVICE_NOT_ROTATED: DEVICE_ORIENTATION_TYPE = DEVICE_ORIENTATION_TYPE(0i32);
pub const DEVICE_ROTATED_180_DEGREES_CLOCKWISE: DEVICE_ORIENTATION_TYPE = DEVICE_ORIENTATION_TYPE(2i32);
pub const DEVICE_ROTATED_270_DEGREES_CLOCKWISE: DEVICE_ORIENTATION_TYPE = DEVICE_ORIENTATION_TYPE(3i32);
pub const DEVICE_ROTATED_90_DEGREES_CLOCKWISE: DEVICE_ORIENTATION_TYPE = DEVICE_ORIENTATION_TYPE(1i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_APO_SWFallback_ProcessingModes: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 13 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 15 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 18 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 17 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 16 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 14 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 20 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 19 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CompositeFX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 13 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_EFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ApplyToBluetooth: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 30 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ApplyToCapture: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 33 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ApplyToRender: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ApplyToUsb: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 31 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Association: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 0 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Author: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 26 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_EffectPackSchema_Version: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 29 };
pub const PKEY_FX_EffectPack_Schema_V1: windows_core::GUID = windows_core::GUID::from_u128(0x7abf23d9_727e_4d0b_86a3_dd501d260001);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Enumerator: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 23 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_FriendlyName: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_EndpointEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 10 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_KeywordDetector_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_ObjectId: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 27 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_ModeEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 12 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_Offload_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 11 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PostMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_PreMixEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 1 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_State: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 28 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_StreamEffectClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_SupportAppLauncher: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 21 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_SupportedFormats: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 22 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_UserInterfaceClsid: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_VersionMajor: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 24 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FX_VersionMinor: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd04e05a6_594b_4fb6_a80d_01af5eed7d1d), pid: 25 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 12 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_MFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_KeywordDetector_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_Offload_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 11 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SFX_ProcessingModes_Supported_For_Streaming: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xd3993a3f_99c2_4402_b5ec_a92a0367664b), pid: 5 };
pub const SID_AudioProcessingObjectLoggingService: windows_core::GUID = windows_core::GUID::from_u128(0x8b8008af_09f9_456e_a173_bdb58499bce7);
pub const SID_AudioProcessingObjectRTQueue: windows_core::GUID = windows_core::GUID::from_u128(0x458c1a1f_6899_4c12_99ac_e2e6ac253104);
pub const eAudioConstriction14_14: EAudioConstriction = EAudioConstriction(3i32);
pub const eAudioConstriction44_16: EAudioConstriction = EAudioConstriction(2i32);
pub const eAudioConstriction48_16: EAudioConstriction = EAudioConstriction(1i32);
pub const eAudioConstrictionMute: EAudioConstriction = EAudioConstriction(4i32);
pub const eAudioConstrictionOff: EAudioConstriction = EAudioConstriction(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APO_BUFFER_FLAGS(pub i32);
impl windows_core::TypeKind for APO_BUFFER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APO_BUFFER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APO_BUFFER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APO_CONNECTION_BUFFER_TYPE(pub i32);
impl windows_core::TypeKind for APO_CONNECTION_BUFFER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APO_CONNECTION_BUFFER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APO_CONNECTION_BUFFER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APO_FLAG(pub i32);
impl windows_core::TypeKind for APO_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APO_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APO_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APO_LOG_LEVEL(pub i32);
impl windows_core::TypeKind for APO_LOG_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APO_LOG_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APO_LOG_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APO_NOTIFICATION_TYPE(pub i32);
impl windows_core::TypeKind for APO_NOTIFICATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APO_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APO_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUDIO_FLOW_TYPE(pub i32);
impl windows_core::TypeKind for AUDIO_FLOW_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUDIO_FLOW_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUDIO_FLOW_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUDIO_SYSTEMEFFECT_STATE(pub i32);
impl windows_core::TypeKind for AUDIO_SYSTEMEFFECT_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUDIO_SYSTEMEFFECT_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUDIO_SYSTEMEFFECT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DEVICE_ORIENTATION_TYPE(pub i32);
impl windows_core::TypeKind for DEVICE_ORIENTATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DEVICE_ORIENTATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DEVICE_ORIENTATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EAudioConstriction(pub i32);
impl windows_core::TypeKind for EAudioConstriction {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EAudioConstriction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EAudioConstriction").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct APOInitBaseStruct {
    pub cbSize: u32,
    pub clsid: windows_core::GUID,
}
impl windows_core::TypeKind for APOInitBaseStruct {
    type TypeKind = windows_core::CopyType;
}
impl Default for APOInitBaseStruct {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[derive(Debug, Eq, PartialEq)]
pub struct APOInitSystemEffects {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: core::mem::ManuallyDrop<Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    pub pAPOSystemEffectsProperties: core::mem::ManuallyDrop<Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    pub pReserved: *mut core::ffi::c_void,
    pub pDeviceCollection: core::mem::ManuallyDrop<Option<super::IMMDeviceCollection>>,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Clone for APOInitSystemEffects {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::TypeKind for APOInitSystemEffects {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Default for APOInitSystemEffects {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[derive(Debug, Eq, PartialEq)]
pub struct APOInitSystemEffects2 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: core::mem::ManuallyDrop<Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    pub pAPOSystemEffectsProperties: core::mem::ManuallyDrop<Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    pub pReserved: *mut core::ffi::c_void,
    pub pDeviceCollection: core::mem::ManuallyDrop<Option<super::IMMDeviceCollection>>,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: windows_core::GUID,
    pub InitializeForDiscoveryOnly: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Clone for APOInitSystemEffects2 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::TypeKind for APOInitSystemEffects2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Default for APOInitSystemEffects2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
#[derive(Debug, Eq, PartialEq)]
pub struct APOInitSystemEffects3 {
    pub APOInit: APOInitBaseStruct,
    pub pAPOEndpointProperties: core::mem::ManuallyDrop<Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    pub pServiceProvider: core::mem::ManuallyDrop<Option<super::super::super::System::Com::IServiceProvider>>,
    pub pDeviceCollection: core::mem::ManuallyDrop<Option<super::IMMDeviceCollection>>,
    pub nSoftwareIoDeviceInCollection: u32,
    pub nSoftwareIoConnectorIndex: u32,
    pub AudioProcessingMode: windows_core::GUID,
    pub InitializeForDiscoveryOnly: super::super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl Clone for APOInitSystemEffects3 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::TypeKind for APOInitSystemEffects3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl Default for APOInitSystemEffects3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct APO_CONNECTION_DESCRIPTOR {
    pub Type: APO_CONNECTION_BUFFER_TYPE,
    pub pBuffer: usize,
    pub u32MaxFrameCount: u32,
    pub pFormat: core::mem::ManuallyDrop<Option<IAudioMediaType>>,
    pub u32Signature: u32,
}
impl Clone for APO_CONNECTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for APO_CONNECTION_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for APO_CONNECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct APO_CONNECTION_PROPERTY {
    pub pBuffer: usize,
    pub u32ValidFrameCount: u32,
    pub u32BufferFlags: APO_BUFFER_FLAGS,
    pub u32Signature: u32,
}
impl windows_core::TypeKind for APO_CONNECTION_PROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl Default for APO_CONNECTION_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct APO_CONNECTION_PROPERTY_V2 {
    pub property: APO_CONNECTION_PROPERTY,
    pub u64QPCTime: u64,
}
impl windows_core::TypeKind for APO_CONNECTION_PROPERTY_V2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for APO_CONNECTION_PROPERTY_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub struct APO_NOTIFICATION {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_0,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Clone for APO_NOTIFICATION {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::TypeKind for APO_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Default for APO_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub union APO_NOTIFICATION_0 {
    pub audioEndpointVolumeChange: core::mem::ManuallyDrop<AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION>,
    pub audioEndpointPropertyChange: core::mem::ManuallyDrop<AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION>,
    pub audioSystemEffectsPropertyChange: core::mem::ManuallyDrop<AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION>,
    pub audioEndpointVolumeChange2: core::mem::ManuallyDrop<AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION2>,
    pub deviceOrientation: DEVICE_ORIENTATION_TYPE,
    pub audioMicrophoneBoostChange: core::mem::ManuallyDrop<AUDIO_MICROPHONE_BOOST_NOTIFICATION>,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Clone for APO_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::TypeKind for APO_NOTIFICATION_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Default for APO_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct APO_NOTIFICATION_DESCRIPTOR {
    pub r#type: APO_NOTIFICATION_TYPE,
    pub Anonymous: APO_NOTIFICATION_DESCRIPTOR_0,
}
impl Clone for APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for APO_NOTIFICATION_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union APO_NOTIFICATION_DESCRIPTOR_0 {
    pub audioEndpointVolume: core::mem::ManuallyDrop<AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR>,
    pub audioEndpointPropertyChange: core::mem::ManuallyDrop<AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR>,
    pub audioSystemEffectsPropertyChange: core::mem::ManuallyDrop<AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR>,
    pub audioMicrophoneBoost: core::mem::ManuallyDrop<AUDIO_MICROPHONE_BOOST_APO_NOTIFICATION_DESCRIPTOR>,
}
impl Clone for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for APO_NOTIFICATION_DESCRIPTOR_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for APO_NOTIFICATION_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct APO_REG_PROPERTIES {
    pub clsid: windows_core::GUID,
    pub Flags: APO_FLAG,
    pub szFriendlyName: [u16; 256],
    pub szCopyrightInfo: [u16; 256],
    pub u32MajorVersion: u32,
    pub u32MinorVersion: u32,
    pub u32MinInputConnections: u32,
    pub u32MaxInputConnections: u32,
    pub u32MinOutputConnections: u32,
    pub u32MaxOutputConnections: u32,
    pub u32MaxInstances: u32,
    pub u32NumAPOInterfaces: u32,
    pub iidAPOInterfaceList: [windows_core::GUID; 1],
}
impl windows_core::TypeKind for APO_REG_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for APO_REG_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: core::mem::ManuallyDrop<Option<super::IMMDevice>>,
}
impl Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIO_ENDPOINT_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[derive(Debug, Eq, PartialEq)]
pub struct AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: core::mem::ManuallyDrop<Option<super::IMMDevice>>,
    pub propertyStore: core::mem::ManuallyDrop<Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    pub propertyKey: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Clone for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::TypeKind for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Default for AUDIO_ENDPOINT_PROPERTY_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    pub device: core::mem::ManuallyDrop<Option<super::IMMDevice>>,
}
impl Clone for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIO_ENDPOINT_VOLUME_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    pub endpoint: core::mem::ManuallyDrop<Option<super::IMMDevice>>,
    pub volume: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA,
}
impl Clone for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION2 {
    pub endpoint: core::mem::ManuallyDrop<Option<super::IMMDevice>>,
    pub volume: *mut AUDIO_VOLUME_NOTIFICATION_DATA2,
}
impl Clone for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION2 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIO_ENDPOINT_VOLUME_CHANGE_NOTIFICATION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct AUDIO_MICROPHONE_BOOST_APO_NOTIFICATION_DESCRIPTOR {
    pub device: core::mem::ManuallyDrop<Option<super::IMMDevice>>,
}
impl Clone for AUDIO_MICROPHONE_BOOST_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for AUDIO_MICROPHONE_BOOST_APO_NOTIFICATION_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIO_MICROPHONE_BOOST_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct AUDIO_MICROPHONE_BOOST_NOTIFICATION {
    pub endpoint: core::mem::ManuallyDrop<Option<super::IMMDevice>>,
    pub eventContext: windows_core::GUID,
    pub microphoneBoostEnabled: super::super::super::Foundation::BOOL,
    pub levelInDb: f32,
    pub levelMinInDb: f32,
    pub levelMaxInDb: f32,
    pub levelStepInDb: f32,
    pub muteSupported: super::super::super::Foundation::BOOL,
    pub mute: super::super::super::Foundation::BOOL,
}
impl Clone for AUDIO_MICROPHONE_BOOST_NOTIFICATION {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for AUDIO_MICROPHONE_BOOST_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIO_MICROPHONE_BOOST_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUDIO_SYSTEMEFFECT {
    pub id: windows_core::GUID,
    pub canSetState: super::super::super::Foundation::BOOL,
    pub state: AUDIO_SYSTEMEFFECT_STATE,
}
impl windows_core::TypeKind for AUDIO_SYSTEMEFFECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIO_SYSTEMEFFECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    pub device: core::mem::ManuallyDrop<Option<super::IMMDevice>>,
    pub propertyStoreContext: windows_core::GUID,
}
impl Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_APO_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[derive(Debug, Eq, PartialEq)]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    pub endpoint: core::mem::ManuallyDrop<Option<super::IMMDevice>>,
    pub propertyStoreContext: windows_core::GUID,
    pub propertyStoreType: super::AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE,
    pub propertyStore: core::mem::ManuallyDrop<Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    pub propertyKey: super::super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Clone for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::TypeKind for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Default for AUDIO_SYSTEMEFFECTS_PROPERTY_CHANGE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUDIO_VOLUME_NOTIFICATION_DATA2 {
    pub notificationData: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA,
    pub masterVolumeInDb: f32,
    pub volumeMinInDb: f32,
    pub volumeMaxInDb: f32,
    pub volumeIncrementInDb: f32,
    pub step: u32,
    pub stepCount: u32,
    pub channelVolumesInDb: [f32; 1],
}
impl windows_core::TypeKind for AUDIO_VOLUME_NOTIFICATION_DATA2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIO_VOLUME_NOTIFICATION_DATA2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[derive(Debug, Eq, PartialEq)]
pub struct AudioFXExtensionParams {
    pub AddPageParam: super::super::super::Foundation::LPARAM,
    pub pwstrEndpointID: windows_core::PWSTR,
    pub pFxProperties: core::mem::ManuallyDrop<Option<super::super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Clone for AudioFXExtensionParams {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::TypeKind for AudioFXExtensionParams {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl Default for AudioFXExtensionParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UNCOMPRESSEDAUDIOFORMAT {
    pub guidFormatType: windows_core::GUID,
    pub dwSamplesPerFrame: u32,
    pub dwBytesPerSampleContainer: u32,
    pub dwValidBitsPerSample: u32,
    pub fFramesPerSecond: f32,
    pub dwChannelMask: u32,
}
impl windows_core::TypeKind for UNCOMPRESSEDAUDIOFORMAT {
    type TypeKind = windows_core::CopyType;
}
impl Default for UNCOMPRESSEDAUDIOFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FNAPONOTIFICATIONCALLBACK = Option<unsafe extern "system" fn(pproperties: *mut APO_REG_PROPERTIES, pvrefdata: *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
