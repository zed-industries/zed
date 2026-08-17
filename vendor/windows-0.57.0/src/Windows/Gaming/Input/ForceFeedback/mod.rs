windows_core::imp::define_interface!(IConditionForceEffect, IConditionForceEffect_Vtbl, 0x32d1ea68_3695_4e69_85c0_cd1944189140);
impl windows_core::RuntimeType for IConditionForceEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConditionForceEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ConditionForceEffectKind) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3, f32, f32, f32, f32, f32, f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
}
windows_core::imp::define_interface!(IConditionForceEffectFactory, IConditionForceEffectFactory_Vtbl, 0x91a99264_1810_4eb6_a773_bfd3b8cddbab);
impl windows_core::RuntimeType for IConditionForceEffectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConditionForceEffectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, ConditionForceEffectKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConstantForceEffect, IConstantForceEffect_Vtbl, 0x9bfa0140_f3c7_415c_b068_0f068734bce0);
impl windows_core::RuntimeType for IConstantForceEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConstantForceEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParametersWithEnvelope: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3, f32, f32, f32, super::super::super::Foundation::TimeSpan, super::super::super::Foundation::TimeSpan, super::super::super::Foundation::TimeSpan, super::super::super::Foundation::TimeSpan, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParametersWithEnvelope: usize,
}
windows_core::imp::define_interface!(IForceFeedbackEffect, IForceFeedbackEffect_Vtbl, 0xa17fba0c_2ae4_48c2_8063_eabd0777cb89);
impl core::ops::Deref for IForceFeedbackEffect {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IForceFeedbackEffect, windows_core::IUnknown, windows_core::IInspectable);
impl IForceFeedbackEffect {
    pub fn Gain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> windows_core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for IForceFeedbackEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IForceFeedbackEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Gain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetGain: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ForceFeedbackEffectState) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IForceFeedbackMotor, IForceFeedbackMotor_Vtbl, 0x8d3d417c_a5ea_4516_8026_2b00f74ef6e5);
impl windows_core::RuntimeType for IForceFeedbackMotor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IForceFeedbackMotor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AreEffectsPaused: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MasterGain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMasterGain: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SupportedAxes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ForceFeedbackEffectAxes) -> windows_core::HRESULT,
    pub LoadEffectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PauseAllEffects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResumeAllEffects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopAllEffects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryDisableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryEnableAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryResetAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryUnloadEffectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPeriodicForceEffect, IPeriodicForceEffect_Vtbl, 0x5c5138d7_fc75_4d52_9a0a_efe4cab5fe64);
impl windows_core::RuntimeType for IPeriodicForceEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPeriodicForceEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PeriodicForceEffectKind) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3, f32, f32, f32, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParametersWithEnvelope: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3, f32, f32, f32, f32, f32, f32, super::super::super::Foundation::TimeSpan, super::super::super::Foundation::TimeSpan, super::super::super::Foundation::TimeSpan, super::super::super::Foundation::TimeSpan, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParametersWithEnvelope: usize,
}
windows_core::imp::define_interface!(IPeriodicForceEffectFactory, IPeriodicForceEffectFactory_Vtbl, 0x6f62eb1a_9851_477b_b318_35ecaa15070f);
impl windows_core::RuntimeType for IPeriodicForceEffectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPeriodicForceEffectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, PeriodicForceEffectKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRampForceEffect, IRampForceEffect_Vtbl, 0xf1f81259_1ca6_4080_b56d_b43f3354d052);
impl windows_core::RuntimeType for IRampForceEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRampForceEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3, super::super::super::Foundation::Numerics::Vector3, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParametersWithEnvelope: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3, super::super::super::Foundation::Numerics::Vector3, f32, f32, f32, super::super::super::Foundation::TimeSpan, super::super::super::Foundation::TimeSpan, super::super::super::Foundation::TimeSpan, super::super::super::Foundation::TimeSpan, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParametersWithEnvelope: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ConditionForceEffect(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConditionForceEffect, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ConditionForceEffect, IForceFeedbackEffect);
impl ConditionForceEffect {
    pub fn Kind(&self) -> windows_core::Result<ConditionForceEffectKind> {
        let this = &windows_core::Interface::cast::<IConditionForceEffect>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters(&self, direction: super::super::super::Foundation::Numerics::Vector3, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IConditionForceEffect>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetParameters)(windows_core::Interface::as_raw(this), direction, positivecoefficient, negativecoefficient, maxpositivemagnitude, maxnegativemagnitude, deadzone, bias).ok() }
    }
    pub fn CreateInstance(effectkind: ConditionForceEffectKind) -> windows_core::Result<ConditionForceEffect> {
        Self::IConditionForceEffectFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), effectkind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Gain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> windows_core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IConditionForceEffectFactory<R, F: FnOnce(&IConditionForceEffectFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ConditionForceEffect, IConditionForceEffectFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ConditionForceEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IForceFeedbackEffect>();
}
unsafe impl windows_core::Interface for ConditionForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
    const IID: windows_core::GUID = <IForceFeedbackEffect as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConditionForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ConditionForceEffect";
}
unsafe impl Send for ConditionForceEffect {}
unsafe impl Sync for ConditionForceEffect {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ConstantForceEffect(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConstantForceEffect, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ConstantForceEffect, IForceFeedbackEffect);
impl ConstantForceEffect {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ConstantForceEffect, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters(&self, vector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IConstantForceEffect>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetParameters)(windows_core::Interface::as_raw(this), vector, duration).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParametersWithEnvelope(&self, vector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IConstantForceEffect>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetParametersWithEnvelope)(windows_core::Interface::as_raw(this), vector, attackgain, sustaingain, releasegain, startdelay, attackduration, sustainduration, releaseduration, repeatcount).ok() }
    }
    pub fn Gain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> windows_core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ConstantForceEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IForceFeedbackEffect>();
}
unsafe impl windows_core::Interface for ConstantForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
    const IID: windows_core::GUID = <IForceFeedbackEffect as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConstantForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ConstantForceEffect";
}
unsafe impl Send for ConstantForceEffect {}
unsafe impl Sync for ConstantForceEffect {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ForceFeedbackMotor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ForceFeedbackMotor, windows_core::IUnknown, windows_core::IInspectable);
impl ForceFeedbackMotor {
    pub fn AreEffectsPaused(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AreEffectsPaused)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MasterGain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MasterGain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMasterGain(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMasterGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SupportedAxes(&self) -> windows_core::Result<ForceFeedbackEffectAxes> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedAxes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LoadEffectAsync<P0>(&self, effect: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>>
    where
        P0: windows_core::Param<IForceFeedbackEffect>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadEffectAsync)(windows_core::Interface::as_raw(this), effect.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PauseAllEffects(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).PauseAllEffects)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ResumeAllEffects(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ResumeAllEffects)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StopAllEffects(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopAllEffects)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TryDisableAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryDisableAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryEnableAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryEnableAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryResetAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryResetAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryUnloadEffectAsync<P0>(&self, effect: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<IForceFeedbackEffect>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryUnloadEffectAsync)(windows_core::Interface::as_raw(this), effect.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ForceFeedbackMotor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IForceFeedbackMotor>();
}
unsafe impl windows_core::Interface for ForceFeedbackMotor {
    type Vtable = IForceFeedbackMotor_Vtbl;
    const IID: windows_core::GUID = <IForceFeedbackMotor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ForceFeedbackMotor {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor";
}
unsafe impl Send for ForceFeedbackMotor {}
unsafe impl Sync for ForceFeedbackMotor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PeriodicForceEffect(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PeriodicForceEffect, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PeriodicForceEffect, IForceFeedbackEffect);
impl PeriodicForceEffect {
    pub fn Gain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> windows_core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<PeriodicForceEffectKind> {
        let this = &windows_core::Interface::cast::<IPeriodicForceEffect>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters(&self, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPeriodicForceEffect>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetParameters)(windows_core::Interface::as_raw(this), vector, frequency, phase, bias, duration).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParametersWithEnvelope(&self, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPeriodicForceEffect>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetParametersWithEnvelope)(windows_core::Interface::as_raw(this), vector, frequency, phase, bias, attackgain, sustaingain, releasegain, startdelay, attackduration, sustainduration, releaseduration, repeatcount).ok() }
    }
    pub fn CreateInstance(effectkind: PeriodicForceEffectKind) -> windows_core::Result<PeriodicForceEffect> {
        Self::IPeriodicForceEffectFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), effectkind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPeriodicForceEffectFactory<R, F: FnOnce(&IPeriodicForceEffectFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PeriodicForceEffect, IPeriodicForceEffectFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PeriodicForceEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IForceFeedbackEffect>();
}
unsafe impl windows_core::Interface for PeriodicForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
    const IID: windows_core::GUID = <IForceFeedbackEffect as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PeriodicForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect";
}
unsafe impl Send for PeriodicForceEffect {}
unsafe impl Sync for PeriodicForceEffect {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RampForceEffect(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RampForceEffect, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RampForceEffect, IForceFeedbackEffect);
impl RampForceEffect {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RampForceEffect, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Gain(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gain)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetGain)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> windows_core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters(&self, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRampForceEffect>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetParameters)(windows_core::Interface::as_raw(this), startvector, endvector, duration).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParametersWithEnvelope(&self, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRampForceEffect>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetParametersWithEnvelope)(windows_core::Interface::as_raw(this), startvector, endvector, attackgain, sustaingain, releasegain, startdelay, attackduration, sustainduration, releaseduration, repeatcount).ok() }
    }
}
impl windows_core::RuntimeType for RampForceEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IForceFeedbackEffect>();
}
unsafe impl windows_core::Interface for RampForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
    const IID: windows_core::GUID = <IForceFeedbackEffect as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RampForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.RampForceEffect";
}
unsafe impl Send for RampForceEffect {}
unsafe impl Sync for RampForceEffect {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ConditionForceEffectKind(pub i32);
impl ConditionForceEffectKind {
    pub const Spring: Self = Self(0i32);
    pub const Damper: Self = Self(1i32);
    pub const Inertia: Self = Self(2i32);
    pub const Friction: Self = Self(3i32);
}
impl windows_core::TypeKind for ConditionForceEffectKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ConditionForceEffectKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ConditionForceEffectKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ConditionForceEffectKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ConditionForceEffectKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ForceFeedbackEffectAxes(pub u32);
impl ForceFeedbackEffectAxes {
    pub const None: Self = Self(0u32);
    pub const X: Self = Self(1u32);
    pub const Y: Self = Self(2u32);
    pub const Z: Self = Self(4u32);
}
impl windows_core::TypeKind for ForceFeedbackEffectAxes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ForceFeedbackEffectAxes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ForceFeedbackEffectAxes").field(&self.0).finish()
    }
}
impl ForceFeedbackEffectAxes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ForceFeedbackEffectAxes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ForceFeedbackEffectAxes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ForceFeedbackEffectAxes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ForceFeedbackEffectAxes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ForceFeedbackEffectAxes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for ForceFeedbackEffectAxes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectAxes;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ForceFeedbackEffectState(pub i32);
impl ForceFeedbackEffectState {
    pub const Stopped: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Paused: Self = Self(2i32);
    pub const Faulted: Self = Self(3i32);
}
impl windows_core::TypeKind for ForceFeedbackEffectState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ForceFeedbackEffectState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ForceFeedbackEffectState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ForceFeedbackEffectState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ForceFeedbackLoadEffectResult(pub i32);
impl ForceFeedbackLoadEffectResult {
    pub const Succeeded: Self = Self(0i32);
    pub const EffectStorageFull: Self = Self(1i32);
    pub const EffectNotSupported: Self = Self(2i32);
}
impl windows_core::TypeKind for ForceFeedbackLoadEffectResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ForceFeedbackLoadEffectResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ForceFeedbackLoadEffectResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ForceFeedbackLoadEffectResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackLoadEffectResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PeriodicForceEffectKind(pub i32);
impl PeriodicForceEffectKind {
    pub const SquareWave: Self = Self(0i32);
    pub const SineWave: Self = Self(1i32);
    pub const TriangleWave: Self = Self(2i32);
    pub const SawtoothWaveUp: Self = Self(3i32);
    pub const SawtoothWaveDown: Self = Self(4i32);
}
impl windows_core::TypeKind for PeriodicForceEffectKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PeriodicForceEffectKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PeriodicForceEffectKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PeriodicForceEffectKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.PeriodicForceEffectKind;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
