windows_core::imp::define_interface!(ISceneLightingEffect, ISceneLightingEffect_Vtbl, 0x91bb5e52_95d1_4f8b_9a5a_6408b24b8c6a);
impl windows_core::RuntimeType for ISceneLightingEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneLightingEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AmbientAmount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetAmbientAmount: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub DiffuseAmount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDiffuseAmount: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Effects")]
    pub NormalMapSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))]
    NormalMapSource: usize,
    #[cfg(feature = "Graphics_Effects")]
    pub SetNormalMapSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))]
    SetNormalMapSource: usize,
    pub SpecularAmount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpecularAmount: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SpecularShine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetSpecularShine: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISceneLightingEffect2, ISceneLightingEffect2_Vtbl, 0x9e270e81_72f0_4c5c_95f8_8a6e0024f409);
impl windows_core::RuntimeType for ISceneLightingEffect2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISceneLightingEffect2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReflectanceModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SceneLightingEffectReflectanceModel) -> windows_core::HRESULT,
    pub SetReflectanceModel: unsafe extern "system" fn(*mut core::ffi::c_void, SceneLightingEffectReflectanceModel) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SceneLightingEffect(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SceneLightingEffect, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Graphics_Effects")]
windows_core::imp::required_hierarchy!(SceneLightingEffect, super::super::super::Graphics::Effects::IGraphicsEffect, super::super::super::Graphics::Effects::IGraphicsEffectSource);
impl SceneLightingEffect {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SceneLightingEffect, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Graphics_Effects")]
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::super::Graphics::Effects::IGraphicsEffect>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Effects")]
    pub fn SetName(&self, name: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Graphics::Effects::IGraphicsEffect>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name)).ok() }
    }
    pub fn AmbientAmount(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AmbientAmount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAmbientAmount(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAmbientAmount)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DiffuseAmount(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DiffuseAmount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDiffuseAmount(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDiffuseAmount)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Effects")]
    pub fn NormalMapSource(&self) -> windows_core::Result<super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NormalMapSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_Effects")]
    pub fn SetNormalMapSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Effects::IGraphicsEffectSource>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNormalMapSource)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SpecularAmount(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpecularAmount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSpecularAmount(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSpecularAmount)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SpecularShine(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SpecularShine)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSpecularShine(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSpecularShine)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReflectanceModel(&self) -> windows_core::Result<SceneLightingEffectReflectanceModel> {
        let this = &windows_core::Interface::cast::<ISceneLightingEffect2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReflectanceModel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReflectanceModel(&self, value: SceneLightingEffectReflectanceModel) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISceneLightingEffect2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReflectanceModel)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for SceneLightingEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISceneLightingEffect>();
}
unsafe impl windows_core::Interface for SceneLightingEffect {
    type Vtable = ISceneLightingEffect_Vtbl;
    const IID: windows_core::GUID = <ISceneLightingEffect as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SceneLightingEffect {
    const NAME: &'static str = "Windows.UI.Composition.Effects.SceneLightingEffect";
}
unsafe impl Send for SceneLightingEffect {}
unsafe impl Sync for SceneLightingEffect {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SceneLightingEffectReflectanceModel(pub i32);
impl SceneLightingEffectReflectanceModel {
    pub const BlinnPhong: Self = Self(0i32);
    pub const PhysicallyBasedBlinnPhong: Self = Self(1i32);
}
impl windows_core::TypeKind for SceneLightingEffectReflectanceModel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SceneLightingEffectReflectanceModel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SceneLightingEffectReflectanceModel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SceneLightingEffectReflectanceModel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Effects.SceneLightingEffectReflectanceModel;i4)");
}
