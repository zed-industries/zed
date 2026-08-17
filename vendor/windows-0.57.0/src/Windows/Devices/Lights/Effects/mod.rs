windows_core::imp::define_interface!(ILampArrayBitmapEffect, ILampArrayBitmapEffect_Vtbl, 0x3238e065_d877_4627_89e5_2a88f7052fa6);
impl windows_core::RuntimeType for ILampArrayBitmapEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILampArrayBitmapEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub StartDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetStartDelay: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub UpdateInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetUpdateInterval: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SuggestedBitmapSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Size) -> windows_core::HRESULT,
    pub BitmapRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBitmapRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILampArrayBitmapEffectFactory, ILampArrayBitmapEffectFactory_Vtbl, 0x13608090_e336_4c8f_9053_a92407ca7b1d);
impl windows_core::RuntimeType for ILampArrayBitmapEffectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILampArrayBitmapEffectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILampArrayBitmapRequestedEventArgs, ILampArrayBitmapRequestedEventArgs_Vtbl, 0xc8b4af9e_fe63_4d51_babd_619defb454ba);
impl windows_core::RuntimeType for ILampArrayBitmapRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILampArrayBitmapRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SinceStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub UpdateBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    UpdateBitmap: usize,
}
windows_core::imp::define_interface!(ILampArrayBlinkEffect, ILampArrayBlinkEffect_Vtbl, 0xebbf35f6_2fc5_4bb3_b3c3_6221a7680d13);
impl windows_core::RuntimeType for ILampArrayBlinkEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILampArrayBlinkEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    pub AttackDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetAttackDuration: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SustainDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetSustainDuration: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub DecayDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetDecayDuration: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub RepetitionDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetRepetitionDelay: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub StartDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetStartDelay: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Occurrences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOccurrences: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub RepetitionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LampArrayRepetitionMode) -> windows_core::HRESULT,
    pub SetRepetitionMode: unsafe extern "system" fn(*mut core::ffi::c_void, LampArrayRepetitionMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILampArrayBlinkEffectFactory, ILampArrayBlinkEffectFactory_Vtbl, 0x879f1d97_9f50_49b2_a56f_013aa08d55e0);
impl windows_core::RuntimeType for ILampArrayBlinkEffectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILampArrayBlinkEffectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILampArrayColorRampEffect, ILampArrayColorRampEffect_Vtbl, 0x2b004437_40a7_432e_a0b9_0d570c2153ff);
impl windows_core::RuntimeType for ILampArrayColorRampEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILampArrayColorRampEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    pub RampDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetRampDuration: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub StartDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetStartDelay: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub CompletionBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LampArrayEffectCompletionBehavior) -> windows_core::HRESULT,
    pub SetCompletionBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, LampArrayEffectCompletionBehavior) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILampArrayColorRampEffectFactory, ILampArrayColorRampEffectFactory_Vtbl, 0x520bd133_0c74_4df5_bea7_4899e0266b0f);
impl windows_core::RuntimeType for ILampArrayColorRampEffectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILampArrayColorRampEffectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILampArrayCustomEffect, ILampArrayCustomEffect_Vtbl, 0xec579170_3c34_4876_818b_5765f78b0ee4);
impl windows_core::RuntimeType for ILampArrayCustomEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILampArrayCustomEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub UpdateInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetUpdateInterval: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub UpdateRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUpdateRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILampArrayCustomEffectFactory, ILampArrayCustomEffectFactory_Vtbl, 0x68b4774d_63e5_4af0_a58b_3e535b94e8c9);
impl windows_core::RuntimeType for ILampArrayCustomEffectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILampArrayCustomEffectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILampArrayEffect, ILampArrayEffect_Vtbl, 0x11d45590_57fb_4546_b1ce_863107f740df);
impl core::ops::Deref for ILampArrayEffect {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILampArrayEffect, windows_core::IUnknown, windows_core::IInspectable);
impl ILampArrayEffect {
    pub fn ZIndex(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetZIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for ILampArrayEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILampArrayEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ZIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILampArrayEffectPlaylist, ILampArrayEffectPlaylist_Vtbl, 0x7de58bfe_6f61_4103_98c7_d6632f7b9169);
impl windows_core::RuntimeType for ILampArrayEffectPlaylist {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILampArrayEffectPlaylist_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OverrideZIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EffectStartMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LampArrayEffectStartMode) -> windows_core::HRESULT,
    pub SetEffectStartMode: unsafe extern "system" fn(*mut core::ffi::c_void, LampArrayEffectStartMode) -> windows_core::HRESULT,
    pub Occurrences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOccurrences: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub RepetitionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LampArrayRepetitionMode) -> windows_core::HRESULT,
    pub SetRepetitionMode: unsafe extern "system" fn(*mut core::ffi::c_void, LampArrayRepetitionMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILampArrayEffectPlaylistStatics, ILampArrayEffectPlaylistStatics_Vtbl, 0xfb15235c_ea35_4c7f_a016_f3bfc6a6c47d);
impl windows_core::RuntimeType for ILampArrayEffectPlaylistStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILampArrayEffectPlaylistStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub StartAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartAll: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StopAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StopAll: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PauseAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PauseAll: usize,
}
windows_core::imp::define_interface!(ILampArraySolidEffect, ILampArraySolidEffect_Vtbl, 0x441f8213_43cc_4b33_80eb_c6ddde7dc8ed);
impl windows_core::RuntimeType for ILampArraySolidEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILampArraySolidEffect_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub StartDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetStartDelay: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub CompletionBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LampArrayEffectCompletionBehavior) -> windows_core::HRESULT,
    pub SetCompletionBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, LampArrayEffectCompletionBehavior) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILampArraySolidEffectFactory, ILampArraySolidEffectFactory_Vtbl, 0xf862a32c_5576_4341_961b_aee1f13cf9dd);
impl windows_core::RuntimeType for ILampArraySolidEffectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILampArraySolidEffectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILampArrayUpdateRequestedEventArgs, ILampArrayUpdateRequestedEventArgs_Vtbl, 0x73560d6a_576a_48af_8539_67ffa0ab3516);
impl windows_core::RuntimeType for ILampArrayUpdateRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILampArrayUpdateRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SinceStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "UI")]
    pub SetColorForIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorForIndex: usize,
    #[cfg(feature = "UI")]
    pub SetSingleColorForIndices: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::UI::Color, u32, *const i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSingleColorForIndices: usize,
    #[cfg(feature = "UI")]
    pub SetColorsForIndices: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::super::UI::Color, u32, *const i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorsForIndices: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LampArrayBitmapEffect(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LampArrayBitmapEffect, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LampArrayBitmapEffect, ILampArrayEffect);
impl LampArrayBitmapEffect {
    pub fn Duration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDuration(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDuration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StartDelay(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartDelay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartDelay(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStartDelay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UpdateInterval(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUpdateInterval(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUpdateInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SuggestedBitmapSize(&self) -> windows_core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SuggestedBitmapSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BitmapRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<LampArrayBitmapEffect, LampArrayBitmapRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBitmapRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveBitmapRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateInstance<P0>(lamparray: P0, lampindexes: &[i32]) -> windows_core::Result<LampArrayBitmapEffect>
    where
        P0: windows_core::Param<super::LampArray>,
    {
        Self::ILampArrayBitmapEffectFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), lamparray.param().abi(), lampindexes.len().try_into().unwrap(), lampindexes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ZIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetZIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ILampArrayBitmapEffectFactory<R, F: FnOnce(&ILampArrayBitmapEffectFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LampArrayBitmapEffect, ILampArrayBitmapEffectFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LampArrayBitmapEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILampArrayBitmapEffect>();
}
unsafe impl windows_core::Interface for LampArrayBitmapEffect {
    type Vtable = ILampArrayBitmapEffect_Vtbl;
    const IID: windows_core::GUID = <ILampArrayBitmapEffect as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LampArrayBitmapEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayBitmapEffect";
}
unsafe impl Send for LampArrayBitmapEffect {}
unsafe impl Sync for LampArrayBitmapEffect {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LampArrayBitmapRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LampArrayBitmapRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl LampArrayBitmapRequestedEventArgs {
    pub fn SinceStarted(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SinceStarted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn UpdateBitmap<P0>(&self, bitmap: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateBitmap)(windows_core::Interface::as_raw(this), bitmap.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for LampArrayBitmapRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILampArrayBitmapRequestedEventArgs>();
}
unsafe impl windows_core::Interface for LampArrayBitmapRequestedEventArgs {
    type Vtable = ILampArrayBitmapRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ILampArrayBitmapRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LampArrayBitmapRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs";
}
unsafe impl Send for LampArrayBitmapRequestedEventArgs {}
unsafe impl Sync for LampArrayBitmapRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LampArrayBlinkEffect(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LampArrayBlinkEffect, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LampArrayBlinkEffect, ILampArrayEffect);
impl LampArrayBlinkEffect {
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> windows_core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Color)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetColor(&self, value: super::super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AttackDuration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttackDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAttackDuration(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAttackDuration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SustainDuration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SustainDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSustainDuration(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSustainDuration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DecayDuration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DecayDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDecayDuration(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDecayDuration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RepetitionDelay(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RepetitionDelay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRepetitionDelay(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRepetitionDelay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StartDelay(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartDelay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartDelay(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStartDelay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Occurrences(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Occurrences)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOccurrences(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOccurrences)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RepetitionMode(&self) -> windows_core::Result<LampArrayRepetitionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RepetitionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRepetitionMode(&self, value: LampArrayRepetitionMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRepetitionMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance<P0>(lamparray: P0, lampindexes: &[i32]) -> windows_core::Result<LampArrayBlinkEffect>
    where
        P0: windows_core::Param<super::LampArray>,
    {
        Self::ILampArrayBlinkEffectFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), lamparray.param().abi(), lampindexes.len().try_into().unwrap(), lampindexes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ZIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetZIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ILampArrayBlinkEffectFactory<R, F: FnOnce(&ILampArrayBlinkEffectFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LampArrayBlinkEffect, ILampArrayBlinkEffectFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LampArrayBlinkEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILampArrayBlinkEffect>();
}
unsafe impl windows_core::Interface for LampArrayBlinkEffect {
    type Vtable = ILampArrayBlinkEffect_Vtbl;
    const IID: windows_core::GUID = <ILampArrayBlinkEffect as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LampArrayBlinkEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayBlinkEffect";
}
unsafe impl Send for LampArrayBlinkEffect {}
unsafe impl Sync for LampArrayBlinkEffect {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LampArrayColorRampEffect(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LampArrayColorRampEffect, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LampArrayColorRampEffect, ILampArrayEffect);
impl LampArrayColorRampEffect {
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> windows_core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Color)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetColor(&self, value: super::super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RampDuration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RampDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRampDuration(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRampDuration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StartDelay(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartDelay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartDelay(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStartDelay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CompletionBehavior(&self) -> windows_core::Result<LampArrayEffectCompletionBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompletionBehavior)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCompletionBehavior(&self, value: LampArrayEffectCompletionBehavior) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCompletionBehavior)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance<P0>(lamparray: P0, lampindexes: &[i32]) -> windows_core::Result<LampArrayColorRampEffect>
    where
        P0: windows_core::Param<super::LampArray>,
    {
        Self::ILampArrayColorRampEffectFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), lamparray.param().abi(), lampindexes.len().try_into().unwrap(), lampindexes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ZIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetZIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ILampArrayColorRampEffectFactory<R, F: FnOnce(&ILampArrayColorRampEffectFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LampArrayColorRampEffect, ILampArrayColorRampEffectFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LampArrayColorRampEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILampArrayColorRampEffect>();
}
unsafe impl windows_core::Interface for LampArrayColorRampEffect {
    type Vtable = ILampArrayColorRampEffect_Vtbl;
    const IID: windows_core::GUID = <ILampArrayColorRampEffect as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LampArrayColorRampEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayColorRampEffect";
}
unsafe impl Send for LampArrayColorRampEffect {}
unsafe impl Sync for LampArrayColorRampEffect {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LampArrayCustomEffect(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LampArrayCustomEffect, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LampArrayCustomEffect, ILampArrayEffect);
impl LampArrayCustomEffect {
    pub fn Duration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDuration(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDuration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UpdateInterval(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUpdateInterval(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUpdateInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UpdateRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<LampArrayCustomEffect, LampArrayUpdateRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUpdateRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUpdateRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateInstance<P0>(lamparray: P0, lampindexes: &[i32]) -> windows_core::Result<LampArrayCustomEffect>
    where
        P0: windows_core::Param<super::LampArray>,
    {
        Self::ILampArrayCustomEffectFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), lamparray.param().abi(), lampindexes.len().try_into().unwrap(), lampindexes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ZIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetZIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ILampArrayCustomEffectFactory<R, F: FnOnce(&ILampArrayCustomEffectFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LampArrayCustomEffect, ILampArrayCustomEffectFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LampArrayCustomEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILampArrayCustomEffect>();
}
unsafe impl windows_core::Interface for LampArrayCustomEffect {
    type Vtable = ILampArrayCustomEffect_Vtbl;
    const IID: windows_core::GUID = <ILampArrayCustomEffect as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LampArrayCustomEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayCustomEffect";
}
unsafe impl Send for LampArrayCustomEffect {}
unsafe impl Sync for LampArrayCustomEffect {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LampArrayEffectPlaylist(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LampArrayEffectPlaylist, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(LampArrayEffectPlaylist, super::super::super::Foundation::Collections::IIterable::<ILampArrayEffect>, super::super::super::Foundation::Collections::IVectorView::<ILampArrayEffect>);
impl LampArrayEffectPlaylist {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LampArrayEffectPlaylist, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<ILampArrayEffect>> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Append<P0>(&self, effect: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ILampArrayEffect>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), effect.param().abi()).ok() }
    }
    pub fn OverrideZIndex(&self, zindex: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).OverrideZIndex)(windows_core::Interface::as_raw(this), zindex).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Pause)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn EffectStartMode(&self) -> windows_core::Result<LampArrayEffectStartMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EffectStartMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetEffectStartMode(&self, value: LampArrayEffectStartMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEffectStartMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Occurrences(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Occurrences)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOccurrences(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOccurrences)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RepetitionMode(&self) -> windows_core::Result<LampArrayRepetitionMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RepetitionMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRepetitionMode(&self, value: LampArrayRepetitionMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRepetitionMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartAll<P0>(value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>,
    {
        Self::ILampArrayEffectPlaylistStatics(|this| unsafe { (windows_core::Interface::vtable(this).StartAll)(windows_core::Interface::as_raw(this), value.param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StopAll<P0>(value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>,
    {
        Self::ILampArrayEffectPlaylistStatics(|this| unsafe { (windows_core::Interface::vtable(this).StopAll)(windows_core::Interface::as_raw(this), value.param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PauseAll<P0>(value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>,
    {
        Self::ILampArrayEffectPlaylistStatics(|this| unsafe { (windows_core::Interface::vtable(this).PauseAll)(windows_core::Interface::as_raw(this), value.param().abi()).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> windows_core::Result<ILampArrayEffect> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<ILampArrayEffect>,
    {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<ILampArrayEffect>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    #[doc(hidden)]
    pub fn ILampArrayEffectPlaylistStatics<R, F: FnOnce(&ILampArrayEffectPlaylistStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LampArrayEffectPlaylist, ILampArrayEffectPlaylistStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LampArrayEffectPlaylist {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILampArrayEffectPlaylist>();
}
unsafe impl windows_core::Interface for LampArrayEffectPlaylist {
    type Vtable = ILampArrayEffectPlaylist_Vtbl;
    const IID: windows_core::GUID = <ILampArrayEffectPlaylist as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LampArrayEffectPlaylist {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayEffectPlaylist";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for LampArrayEffectPlaylist {
    type Item = ILampArrayEffect;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &LampArrayEffectPlaylist {
    type Item = ILampArrayEffect;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(windows_core::Interface::cast(self).ok())
    }
}
unsafe impl Send for LampArrayEffectPlaylist {}
unsafe impl Sync for LampArrayEffectPlaylist {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LampArraySolidEffect(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LampArraySolidEffect, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LampArraySolidEffect, ILampArrayEffect);
impl LampArraySolidEffect {
    pub fn ZIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ZIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ILampArrayEffect>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetZIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> windows_core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Color)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetColor(&self, value: super::super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDuration(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDuration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StartDelay(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartDelay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartDelay(&self, value: super::super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStartDelay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CompletionBehavior(&self) -> windows_core::Result<LampArrayEffectCompletionBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompletionBehavior)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCompletionBehavior(&self, value: LampArrayEffectCompletionBehavior) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCompletionBehavior)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance<P0>(lamparray: P0, lampindexes: &[i32]) -> windows_core::Result<LampArraySolidEffect>
    where
        P0: windows_core::Param<super::LampArray>,
    {
        Self::ILampArraySolidEffectFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), lamparray.param().abi(), lampindexes.len().try_into().unwrap(), lampindexes.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILampArraySolidEffectFactory<R, F: FnOnce(&ILampArraySolidEffectFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LampArraySolidEffect, ILampArraySolidEffectFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LampArraySolidEffect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILampArraySolidEffect>();
}
unsafe impl windows_core::Interface for LampArraySolidEffect {
    type Vtable = ILampArraySolidEffect_Vtbl;
    const IID: windows_core::GUID = <ILampArraySolidEffect as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LampArraySolidEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArraySolidEffect";
}
unsafe impl Send for LampArraySolidEffect {}
unsafe impl Sync for LampArraySolidEffect {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LampArrayUpdateRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LampArrayUpdateRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl LampArrayUpdateRequestedEventArgs {
    pub fn SinceStarted(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SinceStarted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetColor(&self, desiredcolor: super::super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetColor)(windows_core::Interface::as_raw(this), desiredcolor).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn SetColorForIndex(&self, lampindex: i32, desiredcolor: super::super::super::UI::Color) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetColorForIndex)(windows_core::Interface::as_raw(this), lampindex, desiredcolor).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn SetSingleColorForIndices(&self, desiredcolor: super::super::super::UI::Color, lampindexes: &[i32]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSingleColorForIndices)(windows_core::Interface::as_raw(this), desiredcolor, lampindexes.len().try_into().unwrap(), lampindexes.as_ptr()).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn SetColorsForIndices(&self, desiredcolors: &[super::super::super::UI::Color], lampindexes: &[i32]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetColorsForIndices)(windows_core::Interface::as_raw(this), desiredcolors.len().try_into().unwrap(), desiredcolors.as_ptr(), lampindexes.len().try_into().unwrap(), lampindexes.as_ptr()).ok() }
    }
}
impl windows_core::RuntimeType for LampArrayUpdateRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILampArrayUpdateRequestedEventArgs>();
}
unsafe impl windows_core::Interface for LampArrayUpdateRequestedEventArgs {
    type Vtable = ILampArrayUpdateRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ILampArrayUpdateRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LampArrayUpdateRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs";
}
unsafe impl Send for LampArrayUpdateRequestedEventArgs {}
unsafe impl Sync for LampArrayUpdateRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LampArrayEffectCompletionBehavior(pub i32);
impl LampArrayEffectCompletionBehavior {
    pub const ClearState: Self = Self(0i32);
    pub const KeepState: Self = Self(1i32);
}
impl windows_core::TypeKind for LampArrayEffectCompletionBehavior {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LampArrayEffectCompletionBehavior {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LampArrayEffectCompletionBehavior").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LampArrayEffectCompletionBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.Effects.LampArrayEffectCompletionBehavior;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LampArrayEffectStartMode(pub i32);
impl LampArrayEffectStartMode {
    pub const Sequential: Self = Self(0i32);
    pub const Simultaneous: Self = Self(1i32);
}
impl windows_core::TypeKind for LampArrayEffectStartMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LampArrayEffectStartMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LampArrayEffectStartMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LampArrayEffectStartMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.Effects.LampArrayEffectStartMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LampArrayRepetitionMode(pub i32);
impl LampArrayRepetitionMode {
    pub const Occurrences: Self = Self(0i32);
    pub const Forever: Self = Self(1i32);
}
impl windows_core::TypeKind for LampArrayRepetitionMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LampArrayRepetitionMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LampArrayRepetitionMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LampArrayRepetitionMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.Effects.LampArrayRepetitionMode;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
