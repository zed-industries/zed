windows_core::imp::define_interface!(IAcousticEchoCancellationConfiguration, IAcousticEchoCancellationConfiguration_Vtbl, 0x587e735b_175b_5177_a407_2e33bafe33a5);
impl windows_core::RuntimeType for IAcousticEchoCancellationConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAcousticEchoCancellationConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetEchoCancellationRenderEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioCaptureEffectsManager, IAudioCaptureEffectsManager_Vtbl, 0x8f85c271_038d_4393_8298_540110608eef);
impl windows_core::RuntimeType for IAudioCaptureEffectsManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioCaptureEffectsManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AudioCaptureEffectsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAudioCaptureEffectsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAudioCaptureEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAudioCaptureEffects: usize,
}
windows_core::imp::define_interface!(IAudioEffect, IAudioEffect_Vtbl, 0x34aafa51_9207_4055_be93_6e5734a86ae4);
impl windows_core::RuntimeType for IAudioEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AudioEffectType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioEffectType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioEffect2, IAudioEffect2_Vtbl, 0x06703cb0_757e_5757_8af0_6ba58a8b2990);
impl windows_core::RuntimeType for IAudioEffect2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioEffect2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AcousticEchoCancellationConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanSetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioEffectState) -> windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, AudioEffectState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioEffectDefinition, IAudioEffectDefinition_Vtbl, 0xe4d7f974_7d80_4f73_9089_e31c9db9c294);
impl core::ops::Deref for IAudioEffectDefinition {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioEffectDefinition, windows_core::IUnknown, windows_core::IInspectable);
impl IAudioEffectDefinition {
    pub fn ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatableClassId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IAudioEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioEffectDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivatableClassId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IAudioEffectDefinitionFactory, IAudioEffectDefinitionFactory_Vtbl, 0x8e1da646_e705_45ed_8a2b_fc4e4f405a97);
impl windows_core::RuntimeType for IAudioEffectDefinitionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioEffectDefinitionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
}
windows_core::imp::define_interface!(IAudioEffectsManagerStatics, IAudioEffectsManagerStatics_Vtbl, 0x66406c04_86fa_47cc_a315_f489d8c3fe10);
impl windows_core::RuntimeType for IAudioEffectsManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioEffectsManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Render")]
    pub CreateAudioRenderEffectsManager: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::Render::AudioRenderCategory, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateAudioRenderEffectsManager: usize,
    #[cfg(feature = "Media_Render")]
    pub CreateAudioRenderEffectsManagerWithMode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::Render::AudioRenderCategory, super::AudioProcessing, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateAudioRenderEffectsManagerWithMode: usize,
    #[cfg(feature = "Media_Capture")]
    pub CreateAudioCaptureEffectsManager: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::Capture::MediaCategory, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateAudioCaptureEffectsManager: usize,
    #[cfg(feature = "Media_Capture")]
    pub CreateAudioCaptureEffectsManagerWithMode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::Capture::MediaCategory, super::AudioProcessing, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateAudioCaptureEffectsManagerWithMode: usize,
}
windows_core::imp::define_interface!(IAudioRenderEffectsManager, IAudioRenderEffectsManager_Vtbl, 0x4dc98966_8751_42b2_bfcb_39ca7864bd47);
impl windows_core::RuntimeType for IAudioRenderEffectsManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioRenderEffectsManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AudioRenderEffectsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAudioRenderEffectsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAudioRenderEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAudioRenderEffects: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IAudioRenderEffectsManager2, IAudioRenderEffectsManager2_Vtbl, 0xa844cd09_5ecc_44b3_bb4e_1db07287139c);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IAudioRenderEffectsManager2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAudioRenderEffectsManager2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub EffectsProviderThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    EffectsProviderThumbnail: usize,
    #[cfg(feature = "deprecated")]
    pub EffectsProviderSettingsLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    EffectsProviderSettingsLabel: usize,
    #[cfg(feature = "deprecated")]
    pub ShowSettingsUI: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShowSettingsUI: usize,
}
windows_core::imp::define_interface!(IBasicAudioEffect, IBasicAudioEffect_Vtbl, 0x8c062c53_6bc0_48b8_a99a_4b41550f1359);
impl core::ops::Deref for IBasicAudioEffect {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBasicAudioEffect, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IBasicAudioEffect, super::IMediaExtension);
impl IBasicAudioEffect {
    pub fn UseInputFrameForOutput(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseInputFrameForOutput)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn SupportedEncodingProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::AudioEncodingProperties>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedEncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetEncodingProperties<P0>(&self, encodingproperties: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::MediaProperties::AudioEncodingProperties>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEncodingProperties)(windows_core::Interface::as_raw(this), encodingproperties.param().abi()).ok() }
    }
    pub fn ProcessFrame<P0>(&self, context: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ProcessAudioFrameContext>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ProcessFrame)(windows_core::Interface::as_raw(this), context.param().abi()).ok() }
    }
    pub fn Close(&self, reason: MediaEffectClosedReason) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this), reason).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DiscardQueuedFrames)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<P0>(&self, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &windows_core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProperties)(windows_core::Interface::as_raw(this), configuration.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for IBasicAudioEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBasicAudioEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UseInputFrameForOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub SupportedEncodingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    SupportedEncodingProperties: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetEncodingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetEncodingProperties: usize,
    pub ProcessFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, MediaEffectClosedReason) -> windows_core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBasicVideoEffect, IBasicVideoEffect_Vtbl, 0x8262c7ef_b360_40be_949b_2ff42ff35693);
impl core::ops::Deref for IBasicVideoEffect {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBasicVideoEffect, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IBasicVideoEffect, super::IMediaExtension);
impl IBasicVideoEffect {
    pub fn IsReadOnly(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReadOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SupportedMemoryTypes(&self) -> windows_core::Result<MediaMemoryTypes> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedMemoryTypes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TimeIndependent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeIndependent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn SupportedEncodingProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::VideoEncodingProperties>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedEncodingProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub fn SetEncodingProperties<P0, P1>(&self, encodingproperties: P0, device: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::MediaProperties::VideoEncodingProperties>,
        P1: windows_core::Param<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEncodingProperties)(windows_core::Interface::as_raw(this), encodingproperties.param().abi(), device.param().abi()).ok() }
    }
    pub fn ProcessFrame<P0>(&self, context: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ProcessVideoFrameContext>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ProcessFrame)(windows_core::Interface::as_raw(this), context.param().abi()).ok() }
    }
    pub fn Close(&self, reason: MediaEffectClosedReason) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this), reason).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DiscardQueuedFrames)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<P0>(&self, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &windows_core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProperties)(windows_core::Interface::as_raw(this), configuration.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for IBasicVideoEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBasicVideoEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SupportedMemoryTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MediaMemoryTypes) -> windows_core::HRESULT,
    pub TimeIndependent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub SupportedEncodingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    SupportedEncodingProperties: usize,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub SetEncodingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties")))]
    SetEncodingProperties: usize,
    pub ProcessFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, MediaEffectClosedReason) -> windows_core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompositeVideoFrameContext, ICompositeVideoFrameContext_Vtbl, 0x6c30024b_f514_4278_a5f7_b9188049d110);
impl windows_core::RuntimeType for ICompositeVideoFrameContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICompositeVideoFrameContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))]
    pub SurfacesToOverlay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11")))]
    SurfacesToOverlay: usize,
    pub BackgroundFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutputFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing"))]
    pub GetOverlayForSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing")))]
    GetOverlayForSurface: usize,
}
windows_core::imp::define_interface!(IProcessAudioFrameContext, IProcessAudioFrameContext_Vtbl, 0x4cd92946_1222_4a27_a586_fb3e20273255);
impl windows_core::RuntimeType for IProcessAudioFrameContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProcessAudioFrameContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InputFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutputFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProcessVideoFrameContext, IProcessVideoFrameContext_Vtbl, 0x276f0e2b_6461_401e_ba78_0fdad6114eec);
impl windows_core::RuntimeType for IProcessVideoFrameContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProcessVideoFrameContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InputFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutputFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISlowMotionEffectDefinition, ISlowMotionEffectDefinition_Vtbl, 0x35053cd0_176c_4763_82c4_1b02dbe31737);
impl windows_core::RuntimeType for ISlowMotionEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISlowMotionEffectDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TimeStretchRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetTimeStretchRate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVideoCompositor, IVideoCompositor_Vtbl, 0x8510b43e_420c_420f_96c7_7c98bba1fc55);
impl core::ops::Deref for IVideoCompositor {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVideoCompositor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IVideoCompositor, super::IMediaExtension);
impl IVideoCompositor {
    pub fn TimeIndependent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeIndependent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub fn SetEncodingProperties<P0, P1>(&self, backgroundproperties: P0, device: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::MediaProperties::VideoEncodingProperties>,
        P1: windows_core::Param<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEncodingProperties)(windows_core::Interface::as_raw(this), backgroundproperties.param().abi(), device.param().abi()).ok() }
    }
    pub fn CompositeFrame<P0>(&self, context: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CompositeVideoFrameContext>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CompositeFrame)(windows_core::Interface::as_raw(this), context.param().abi()).ok() }
    }
    pub fn Close(&self, reason: MediaEffectClosedReason) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this), reason).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DiscardQueuedFrames)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<P0>(&self, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &windows_core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProperties)(windows_core::Interface::as_raw(this), configuration.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for IVideoCompositor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoCompositor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TimeIndependent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties"))]
    pub SetEncodingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_MediaProperties")))]
    SetEncodingProperties: usize,
    pub CompositeFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, MediaEffectClosedReason) -> windows_core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVideoCompositorDefinition, IVideoCompositorDefinition_Vtbl, 0x7946b8d0_2010_4ae3_9ab2_2cef42edd4d2);
impl core::ops::Deref for IVideoCompositorDefinition {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVideoCompositorDefinition, windows_core::IUnknown, windows_core::IInspectable);
impl IVideoCompositorDefinition {
    pub fn ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatableClassId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IVideoCompositorDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoCompositorDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivatableClassId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IVideoCompositorDefinitionFactory, IVideoCompositorDefinitionFactory_Vtbl, 0x4366fd10_68b8_4d52_89b6_02a968cca899);
impl windows_core::RuntimeType for IVideoCompositorDefinitionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoCompositorDefinitionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
}
windows_core::imp::define_interface!(IVideoEffectDefinition, IVideoEffectDefinition_Vtbl, 0x39f38cf0_8d0f_4f3e_84fc_2d46a5297943);
impl core::ops::Deref for IVideoEffectDefinition {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVideoEffectDefinition, windows_core::IUnknown, windows_core::IInspectable);
impl IVideoEffectDefinition {
    pub fn ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatableClassId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IVideoEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoEffectDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivatableClassId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IVideoEffectDefinitionFactory, IVideoEffectDefinitionFactory_Vtbl, 0x81439b4e_6e33_428f_9d21_b5aafef7617c);
impl windows_core::RuntimeType for IVideoEffectDefinitionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoEffectDefinitionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
}
windows_core::imp::define_interface!(IVideoTransformEffectDefinition, IVideoTransformEffectDefinition_Vtbl, 0x9664bb6a_1ea6_4aa6_8074_abe8851ecae2);
impl windows_core::RuntimeType for IVideoTransformEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoTransformEffectDefinition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub PaddingColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    PaddingColor: usize,
    #[cfg(feature = "UI")]
    pub SetPaddingColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetPaddingColor: usize,
    pub OutputSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub SetOutputSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size) -> windows_core::HRESULT,
    pub CropRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub SetCropRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Rect) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub Rotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::MediaProperties::MediaRotation) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Rotation: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, super::MediaProperties::MediaRotation) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetRotation: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub Mirror: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::MediaProperties::MediaMirroringOptions) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Mirror: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetMirror: unsafe extern "system" fn(*mut core::ffi::c_void, super::MediaProperties::MediaMirroringOptions) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetMirror: usize,
    #[cfg(feature = "Media_Transcoding")]
    pub SetProcessingAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, super::Transcoding::MediaVideoProcessingAlgorithm) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Transcoding"))]
    SetProcessingAlgorithm: usize,
    #[cfg(feature = "Media_Transcoding")]
    pub ProcessingAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Transcoding::MediaVideoProcessingAlgorithm) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Transcoding"))]
    ProcessingAlgorithm: usize,
}
windows_core::imp::define_interface!(IVideoTransformEffectDefinition2, IVideoTransformEffectDefinition2_Vtbl, 0xf0a8089f_66c8_4694_9fd9_1136abf7444a);
impl windows_core::RuntimeType for IVideoTransformEffectDefinition2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoTransformEffectDefinition2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SphericalProjection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVideoTransformSphericalProjection, IVideoTransformSphericalProjection_Vtbl, 0xcf4401f0_9bf2_4c39_9f41_e022514a8468);
impl windows_core::RuntimeType for IVideoTransformSphericalProjection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVideoTransformSphericalProjection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub FrameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::MediaProperties::SphericalVideoFrameFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    FrameFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetFrameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::MediaProperties::SphericalVideoFrameFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetFrameFormat: usize,
    #[cfg(feature = "Media_Playback")]
    pub ProjectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Playback::SphericalVideoProjectionMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    ProjectionMode: usize,
    #[cfg(feature = "Media_Playback")]
    pub SetProjectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::Playback::SphericalVideoProjectionMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    SetProjectionMode: usize,
    pub HorizontalFieldOfViewInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetHorizontalFieldOfViewInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub ViewOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ViewOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetViewOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetViewOrientation: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AcousticEchoCancellationConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AcousticEchoCancellationConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl AcousticEchoCancellationConfiguration {
    pub fn SetEchoCancellationRenderEndpoint(&self, deviceid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEchoCancellationRenderEndpoint)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid)).ok() }
    }
}
impl windows_core::RuntimeType for AcousticEchoCancellationConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAcousticEchoCancellationConfiguration>();
}
unsafe impl windows_core::Interface for AcousticEchoCancellationConfiguration {
    type Vtable = IAcousticEchoCancellationConfiguration_Vtbl;
    const IID: windows_core::GUID = <IAcousticEchoCancellationConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AcousticEchoCancellationConfiguration {
    const NAME: &'static str = "Windows.Media.Effects.AcousticEchoCancellationConfiguration";
}
unsafe impl Send for AcousticEchoCancellationConfiguration {}
unsafe impl Sync for AcousticEchoCancellationConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AudioCaptureEffectsManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioCaptureEffectsManager, windows_core::IUnknown, windows_core::IInspectable);
impl AudioCaptureEffectsManager {
    pub fn AudioCaptureEffectsChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AudioCaptureEffectsManager, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioCaptureEffectsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAudioCaptureEffectsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAudioCaptureEffectsChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAudioCaptureEffects(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AudioEffect>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAudioCaptureEffects)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AudioCaptureEffectsManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioCaptureEffectsManager>();
}
unsafe impl windows_core::Interface for AudioCaptureEffectsManager {
    type Vtable = IAudioCaptureEffectsManager_Vtbl;
    const IID: windows_core::GUID = <IAudioCaptureEffectsManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioCaptureEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.AudioCaptureEffectsManager";
}
unsafe impl Send for AudioCaptureEffectsManager {}
unsafe impl Sync for AudioCaptureEffectsManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AudioEffect(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioEffect, windows_core::IUnknown, windows_core::IInspectable);
impl AudioEffect {
    pub fn AudioEffectType(&self) -> windows_core::Result<AudioEffectType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioEffectType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AcousticEchoCancellationConfiguration(&self) -> windows_core::Result<AcousticEchoCancellationConfiguration> {
        let this = &windows_core::Interface::cast::<IAudioEffect2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcousticEchoCancellationConfiguration)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanSetState(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAudioEffect2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanSetState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn State(&self) -> windows_core::Result<AudioEffectState> {
        let this = &windows_core::Interface::cast::<IAudioEffect2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetState(&self, newstate: AudioEffectState) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioEffect2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetState)(windows_core::Interface::as_raw(this), newstate).ok() }
    }
}
impl windows_core::RuntimeType for AudioEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioEffect>();
}
unsafe impl windows_core::Interface for AudioEffect {
    type Vtable = IAudioEffect_Vtbl;
    const IID: windows_core::GUID = <IAudioEffect as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioEffect {
    const NAME: &'static str = "Windows.Media.Effects.AudioEffect";
}
unsafe impl Send for AudioEffect {}
unsafe impl Sync for AudioEffect {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AudioEffectDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioEffectDefinition, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AudioEffectDefinition, IAudioEffectDefinition);
impl AudioEffectDefinition {
    pub fn ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatableClassId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(activatableclassid: &windows_core::HSTRING) -> windows_core::Result<AudioEffectDefinition> {
        Self::IAudioEffectDefinitionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithProperties<P0>(activatableclassid: &windows_core::HSTRING, props: P0) -> windows_core::Result<AudioEffectDefinition>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        Self::IAudioEffectDefinitionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithProperties)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), props.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAudioEffectDefinitionFactory<R, F: FnOnce(&IAudioEffectDefinitionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AudioEffectDefinition, IAudioEffectDefinitionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AudioEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioEffectDefinition>();
}
unsafe impl windows_core::Interface for AudioEffectDefinition {
    type Vtable = IAudioEffectDefinition_Vtbl;
    const IID: windows_core::GUID = <IAudioEffectDefinition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.AudioEffectDefinition";
}
unsafe impl Send for AudioEffectDefinition {}
unsafe impl Sync for AudioEffectDefinition {}
pub struct AudioEffectsManager;
impl AudioEffectsManager {
    #[cfg(feature = "Media_Render")]
    pub fn CreateAudioRenderEffectsManager(deviceid: &windows_core::HSTRING, category: super::Render::AudioRenderCategory) -> windows_core::Result<AudioRenderEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAudioRenderEffectsManager)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Media_Render")]
    pub fn CreateAudioRenderEffectsManagerWithMode(deviceid: &windows_core::HSTRING, category: super::Render::AudioRenderCategory, mode: super::AudioProcessing) -> windows_core::Result<AudioRenderEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAudioRenderEffectsManagerWithMode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), category, mode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Media_Capture")]
    pub fn CreateAudioCaptureEffectsManager(deviceid: &windows_core::HSTRING, category: super::Capture::MediaCategory) -> windows_core::Result<AudioCaptureEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAudioCaptureEffectsManager)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Media_Capture")]
    pub fn CreateAudioCaptureEffectsManagerWithMode(deviceid: &windows_core::HSTRING, category: super::Capture::MediaCategory, mode: super::AudioProcessing) -> windows_core::Result<AudioCaptureEffectsManager> {
        Self::IAudioEffectsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAudioCaptureEffectsManagerWithMode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), category, mode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAudioEffectsManagerStatics<R, F: FnOnce(&IAudioEffectsManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AudioEffectsManager, IAudioEffectsManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for AudioEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.AudioEffectsManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AudioRenderEffectsManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioRenderEffectsManager, windows_core::IUnknown, windows_core::IInspectable);
impl AudioRenderEffectsManager {
    pub fn AudioRenderEffectsChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<AudioRenderEffectsManager, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioRenderEffectsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAudioRenderEffectsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAudioRenderEffectsChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAudioRenderEffects(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<AudioEffect>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAudioRenderEffects)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn EffectsProviderThumbnail(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = &windows_core::Interface::cast::<IAudioRenderEffectsManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectsProviderThumbnail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn EffectsProviderSettingsLabel(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAudioRenderEffectsManager2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectsProviderSettingsLabel)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ShowSettingsUI(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAudioRenderEffectsManager2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ShowSettingsUI)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AudioRenderEffectsManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioRenderEffectsManager>();
}
unsafe impl windows_core::Interface for AudioRenderEffectsManager {
    type Vtable = IAudioRenderEffectsManager_Vtbl;
    const IID: windows_core::GUID = <IAudioRenderEffectsManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioRenderEffectsManager {
    const NAME: &'static str = "Windows.Media.Effects.AudioRenderEffectsManager";
}
unsafe impl Send for AudioRenderEffectsManager {}
unsafe impl Sync for AudioRenderEffectsManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CompositeVideoFrameContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompositeVideoFrameContext, windows_core::IUnknown, windows_core::IInspectable);
impl CompositeVideoFrameContext {
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn SurfacesToOverlay(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SurfacesToOverlay)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BackgroundFrame(&self) -> windows_core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OutputFrame(&self) -> windows_core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Media_Editing"))]
    pub fn GetOverlayForSurface<P0>(&self, surfacetooverlay: P0) -> windows_core::Result<super::Editing::MediaOverlay>
    where
        P0: windows_core::Param<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOverlayForSurface)(windows_core::Interface::as_raw(this), surfacetooverlay.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CompositeVideoFrameContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompositeVideoFrameContext>();
}
unsafe impl windows_core::Interface for CompositeVideoFrameContext {
    type Vtable = ICompositeVideoFrameContext_Vtbl;
    const IID: windows_core::GUID = <ICompositeVideoFrameContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompositeVideoFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.CompositeVideoFrameContext";
}
unsafe impl Send for CompositeVideoFrameContext {}
unsafe impl Sync for CompositeVideoFrameContext {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProcessAudioFrameContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProcessAudioFrameContext, windows_core::IUnknown, windows_core::IInspectable);
impl ProcessAudioFrameContext {
    pub fn InputFrame(&self) -> windows_core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OutputFrame(&self) -> windows_core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ProcessAudioFrameContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProcessAudioFrameContext>();
}
unsafe impl windows_core::Interface for ProcessAudioFrameContext {
    type Vtable = IProcessAudioFrameContext_Vtbl;
    const IID: windows_core::GUID = <IProcessAudioFrameContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProcessAudioFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.ProcessAudioFrameContext";
}
unsafe impl Send for ProcessAudioFrameContext {}
unsafe impl Sync for ProcessAudioFrameContext {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProcessVideoFrameContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProcessVideoFrameContext, windows_core::IUnknown, windows_core::IInspectable);
impl ProcessVideoFrameContext {
    pub fn InputFrame(&self) -> windows_core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OutputFrame(&self) -> windows_core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ProcessVideoFrameContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProcessVideoFrameContext>();
}
unsafe impl windows_core::Interface for ProcessVideoFrameContext {
    type Vtable = IProcessVideoFrameContext_Vtbl;
    const IID: windows_core::GUID = <IProcessVideoFrameContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProcessVideoFrameContext {
    const NAME: &'static str = "Windows.Media.Effects.ProcessVideoFrameContext";
}
unsafe impl Send for ProcessVideoFrameContext {}
unsafe impl Sync for ProcessVideoFrameContext {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SlowMotionEffectDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SlowMotionEffectDefinition, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SlowMotionEffectDefinition, IVideoEffectDefinition);
impl SlowMotionEffectDefinition {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SlowMotionEffectDefinition, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TimeStretchRate(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeStretchRate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTimeStretchRate(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTimeStretchRate)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IVideoEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatableClassId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &windows_core::Interface::cast::<IVideoEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SlowMotionEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISlowMotionEffectDefinition>();
}
unsafe impl windows_core::Interface for SlowMotionEffectDefinition {
    type Vtable = ISlowMotionEffectDefinition_Vtbl;
    const IID: windows_core::GUID = <ISlowMotionEffectDefinition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SlowMotionEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.SlowMotionEffectDefinition";
}
unsafe impl Send for SlowMotionEffectDefinition {}
unsafe impl Sync for SlowMotionEffectDefinition {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VideoCompositorDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VideoCompositorDefinition, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VideoCompositorDefinition, IVideoCompositorDefinition);
impl VideoCompositorDefinition {
    pub fn ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatableClassId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(activatableclassid: &windows_core::HSTRING) -> windows_core::Result<VideoCompositorDefinition> {
        Self::IVideoCompositorDefinitionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithProperties<P0>(activatableclassid: &windows_core::HSTRING, props: P0) -> windows_core::Result<VideoCompositorDefinition>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        Self::IVideoCompositorDefinitionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithProperties)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), props.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVideoCompositorDefinitionFactory<R, F: FnOnce(&IVideoCompositorDefinitionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VideoCompositorDefinition, IVideoCompositorDefinitionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VideoCompositorDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVideoCompositorDefinition>();
}
unsafe impl windows_core::Interface for VideoCompositorDefinition {
    type Vtable = IVideoCompositorDefinition_Vtbl;
    const IID: windows_core::GUID = <IVideoCompositorDefinition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VideoCompositorDefinition {
    const NAME: &'static str = "Windows.Media.Effects.VideoCompositorDefinition";
}
unsafe impl Send for VideoCompositorDefinition {}
unsafe impl Sync for VideoCompositorDefinition {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VideoEffectDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VideoEffectDefinition, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VideoEffectDefinition, IVideoEffectDefinition);
impl VideoEffectDefinition {
    pub fn ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatableClassId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(activatableclassid: &windows_core::HSTRING) -> windows_core::Result<VideoEffectDefinition> {
        Self::IVideoEffectDefinitionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithProperties<P0>(activatableclassid: &windows_core::HSTRING, props: P0) -> windows_core::Result<VideoEffectDefinition>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        Self::IVideoEffectDefinitionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithProperties)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), props.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVideoEffectDefinitionFactory<R, F: FnOnce(&IVideoEffectDefinitionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VideoEffectDefinition, IVideoEffectDefinitionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VideoEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVideoEffectDefinition>();
}
unsafe impl windows_core::Interface for VideoEffectDefinition {
    type Vtable = IVideoEffectDefinition_Vtbl;
    const IID: windows_core::GUID = <IVideoEffectDefinition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VideoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.VideoEffectDefinition";
}
unsafe impl Send for VideoEffectDefinition {}
unsafe impl Sync for VideoEffectDefinition {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VideoTransformEffectDefinition(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VideoTransformEffectDefinition, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(VideoTransformEffectDefinition, IVideoEffectDefinition);
impl VideoTransformEffectDefinition {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VideoTransformEffectDefinition, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivatableClassId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI")]
    pub fn PaddingColor(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = &windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PaddingColor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetPaddingColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPaddingColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutputSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        let this = &windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOutputSize(&self, value: super::super::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutputSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CropRectangle(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CropRectangle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCropRectangle(&self, value: super::super::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCropRectangle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Rotation(&self) -> windows_core::Result<super::MediaProperties::MediaRotation> {
        let this = &windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetRotation(&self, value: super::MediaProperties::MediaRotation) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Mirror(&self) -> windows_core::Result<super::MediaProperties::MediaMirroringOptions> {
        let this = &windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mirror)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetMirror(&self, value: super::MediaProperties::MediaMirroringOptions) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMirror)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_Transcoding")]
    pub fn SetProcessingAlgorithm(&self, value: super::Transcoding::MediaVideoProcessingAlgorithm) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProcessingAlgorithm)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_Transcoding")]
    pub fn ProcessingAlgorithm(&self) -> windows_core::Result<super::Transcoding::MediaVideoProcessingAlgorithm> {
        let this = &windows_core::Interface::cast::<IVideoTransformEffectDefinition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProcessingAlgorithm)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SphericalProjection(&self) -> windows_core::Result<VideoTransformSphericalProjection> {
        let this = &windows_core::Interface::cast::<IVideoTransformEffectDefinition2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SphericalProjection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for VideoTransformEffectDefinition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVideoEffectDefinition>();
}
unsafe impl windows_core::Interface for VideoTransformEffectDefinition {
    type Vtable = IVideoEffectDefinition_Vtbl;
    const IID: windows_core::GUID = <IVideoEffectDefinition as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VideoTransformEffectDefinition {
    const NAME: &'static str = "Windows.Media.Effects.VideoTransformEffectDefinition";
}
unsafe impl Send for VideoTransformEffectDefinition {}
unsafe impl Sync for VideoTransformEffectDefinition {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VideoTransformSphericalProjection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VideoTransformSphericalProjection, windows_core::IUnknown, windows_core::IInspectable);
impl VideoTransformSphericalProjection {
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn FrameFormat(&self) -> windows_core::Result<super::MediaProperties::SphericalVideoFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetFrameFormat(&self, value: super::MediaProperties::SphericalVideoFrameFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetFrameFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Media_Playback")]
    pub fn ProjectionMode(&self) -> windows_core::Result<super::Playback::SphericalVideoProjectionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProjectionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_Playback")]
    pub fn SetProjectionMode(&self, value: super::Playback::SphericalVideoProjectionMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProjectionMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalFieldOfViewInDegrees(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalFieldOfViewInDegrees)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalFieldOfViewInDegrees(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalFieldOfViewInDegrees)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ViewOrientation(&self) -> windows_core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ViewOrientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetViewOrientation(&self, value: super::super::Foundation::Numerics::Quaternion) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetViewOrientation)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for VideoTransformSphericalProjection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVideoTransformSphericalProjection>();
}
unsafe impl windows_core::Interface for VideoTransformSphericalProjection {
    type Vtable = IVideoTransformSphericalProjection_Vtbl;
    const IID: windows_core::GUID = <IVideoTransformSphericalProjection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VideoTransformSphericalProjection {
    const NAME: &'static str = "Windows.Media.Effects.VideoTransformSphericalProjection";
}
unsafe impl Send for VideoTransformSphericalProjection {}
unsafe impl Sync for VideoTransformSphericalProjection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioEffectState(pub i32);
impl AudioEffectState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
}
impl windows_core::TypeKind for AudioEffectState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioEffectState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioEffectState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AudioEffectState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.AudioEffectState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioEffectType(pub i32);
impl AudioEffectType {
    pub const Other: Self = Self(0i32);
    pub const AcousticEchoCancellation: Self = Self(1i32);
    pub const NoiseSuppression: Self = Self(2i32);
    pub const AutomaticGainControl: Self = Self(3i32);
    pub const BeamForming: Self = Self(4i32);
    pub const ConstantToneRemoval: Self = Self(5i32);
    pub const Equalizer: Self = Self(6i32);
    pub const LoudnessEqualizer: Self = Self(7i32);
    pub const BassBoost: Self = Self(8i32);
    pub const VirtualSurround: Self = Self(9i32);
    pub const VirtualHeadphones: Self = Self(10i32);
    pub const SpeakerFill: Self = Self(11i32);
    pub const RoomCorrection: Self = Self(12i32);
    pub const BassManagement: Self = Self(13i32);
    pub const EnvironmentalEffects: Self = Self(14i32);
    pub const SpeakerProtection: Self = Self(15i32);
    pub const SpeakerCompensation: Self = Self(16i32);
    pub const DynamicRangeCompression: Self = Self(17i32);
    pub const FarFieldBeamForming: Self = Self(18i32);
    pub const DeepNoiseSuppression: Self = Self(19i32);
}
impl windows_core::TypeKind for AudioEffectType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioEffectType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioEffectType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AudioEffectType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.AudioEffectType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaEffectClosedReason(pub i32);
impl MediaEffectClosedReason {
    pub const Done: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const UnsupportedEncodingFormat: Self = Self(2i32);
    pub const EffectCurrentlyUnloaded: Self = Self(3i32);
}
impl windows_core::TypeKind for MediaEffectClosedReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaEffectClosedReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaEffectClosedReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaEffectClosedReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.MediaEffectClosedReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MediaMemoryTypes(pub i32);
impl MediaMemoryTypes {
    pub const Gpu: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const GpuAndCpu: Self = Self(2i32);
}
impl windows_core::TypeKind for MediaMemoryTypes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MediaMemoryTypes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MediaMemoryTypes").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MediaMemoryTypes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Effects.MediaMemoryTypes;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
