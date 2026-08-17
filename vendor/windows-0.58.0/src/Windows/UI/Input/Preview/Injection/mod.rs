windows_core::imp::define_interface!(IInjectedInputGamepadInfo, IInjectedInputGamepadInfo_Vtbl, 0x20ae9a3f_df11_4572_a9ab_d75b8a5e48ad);
impl windows_core::RuntimeType for IInjectedInputGamepadInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInjectedInputGamepadInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Gaming_Input")]
    pub Buttons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::Gaming::Input::GamepadButtons) -> windows_core::HRESULT,
    #[cfg(not(feature = "Gaming_Input"))]
    Buttons: usize,
    #[cfg(feature = "Gaming_Input")]
    pub SetButtons: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Gaming::Input::GamepadButtons) -> windows_core::HRESULT,
    #[cfg(not(feature = "Gaming_Input"))]
    SetButtons: usize,
    pub LeftThumbstickX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLeftThumbstickX: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub LeftThumbstickY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLeftThumbstickY: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub LeftTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLeftTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub RightThumbstickX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetRightThumbstickX: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub RightThumbstickY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetRightThumbstickY: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub RightTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetRightTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInjectedInputGamepadInfoFactory, IInjectedInputGamepadInfoFactory_Vtbl, 0x59596876_6c39_4ec4_8b2a_29ef7de18aca);
impl windows_core::RuntimeType for IInjectedInputGamepadInfoFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInjectedInputGamepadInfoFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Gaming_Input")]
    pub CreateInstanceFromGamepadReading: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Gaming::Input::GamepadReading, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Gaming_Input"))]
    CreateInstanceFromGamepadReading: usize,
}
windows_core::imp::define_interface!(IInjectedInputKeyboardInfo, IInjectedInputKeyboardInfo_Vtbl, 0x4b46d140_2b6a_5ffa_7eae_bd077b052acd);
impl windows_core::RuntimeType for IInjectedInputKeyboardInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInjectedInputKeyboardInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KeyOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InjectedInputKeyOptions) -> windows_core::HRESULT,
    pub SetKeyOptions: unsafe extern "system" fn(*mut core::ffi::c_void, InjectedInputKeyOptions) -> windows_core::HRESULT,
    pub ScanCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SetScanCode: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub VirtualKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SetVirtualKey: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInjectedInputMouseInfo, IInjectedInputMouseInfo_Vtbl, 0x96f56e6b_e47a_5cf4_418d_8a5fb9670c7d);
impl windows_core::RuntimeType for IInjectedInputMouseInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInjectedInputMouseInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MouseOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InjectedInputMouseOptions) -> windows_core::HRESULT,
    pub SetMouseOptions: unsafe extern "system" fn(*mut core::ffi::c_void, InjectedInputMouseOptions) -> windows_core::HRESULT,
    pub MouseData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMouseData: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeltaY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDeltaY: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DeltaX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDeltaX: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub TimeOffsetInMilliseconds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTimeOffsetInMilliseconds: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInjectedInputPenInfo, IInjectedInputPenInfo_Vtbl, 0x6b40ad03_ca1e_5527_7e02_2828540bb1d4);
impl windows_core::RuntimeType for IInjectedInputPenInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInjectedInputPenInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InjectedInputPointerInfo) -> windows_core::HRESULT,
    pub SetPointerInfo: unsafe extern "system" fn(*mut core::ffi::c_void, InjectedInputPointerInfo) -> windows_core::HRESULT,
    pub PenButtons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InjectedInputPenButtons) -> windows_core::HRESULT,
    pub SetPenButtons: unsafe extern "system" fn(*mut core::ffi::c_void, InjectedInputPenButtons) -> windows_core::HRESULT,
    pub PenParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InjectedInputPenParameters) -> windows_core::HRESULT,
    pub SetPenParameters: unsafe extern "system" fn(*mut core::ffi::c_void, InjectedInputPenParameters) -> windows_core::HRESULT,
    pub Pressure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetPressure: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Rotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub TiltX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTiltX: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub TiltY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTiltY: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInjectedInputTouchInfo, IInjectedInputTouchInfo_Vtbl, 0x224fd1df_43e8_5ef5_510a_69ca8c9b4c28);
impl windows_core::RuntimeType for IInjectedInputTouchInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInjectedInputTouchInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InjectedInputRectangle) -> windows_core::HRESULT,
    pub SetContact: unsafe extern "system" fn(*mut core::ffi::c_void, InjectedInputRectangle) -> windows_core::HRESULT,
    pub Orientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PointerInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InjectedInputPointerInfo) -> windows_core::HRESULT,
    pub SetPointerInfo: unsafe extern "system" fn(*mut core::ffi::c_void, InjectedInputPointerInfo) -> windows_core::HRESULT,
    pub Pressure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetPressure: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub TouchParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InjectedInputTouchParameters) -> windows_core::HRESULT,
    pub SetTouchParameters: unsafe extern "system" fn(*mut core::ffi::c_void, InjectedInputTouchParameters) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputInjector, IInputInjector_Vtbl, 0x8ec26f84_0b02_4bd2_ad7a_3d4658be3e18);
impl windows_core::RuntimeType for IInputInjector {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputInjector_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub InjectKeyboardInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InjectKeyboardInput: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InjectMouseInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InjectMouseInput: usize,
    pub InitializeTouchInjection: unsafe extern "system" fn(*mut core::ffi::c_void, InjectedInputVisualizationMode) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub InjectTouchInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InjectTouchInput: usize,
    pub UninitializeTouchInjection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializePenInjection: unsafe extern "system" fn(*mut core::ffi::c_void, InjectedInputVisualizationMode) -> windows_core::HRESULT,
    pub InjectPenInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UninitializePenInjection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InjectShortcut: unsafe extern "system" fn(*mut core::ffi::c_void, InjectedInputShortcut) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputInjector2, IInputInjector2_Vtbl, 0x8e7a905d_1453_43a7_9bcb_06d6d7b305f7);
impl windows_core::RuntimeType for IInputInjector2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputInjector2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InitializeGamepadInjection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InjectGamepadInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UninitializeGamepadInjection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputInjectorStatics, IInputInjectorStatics_Vtbl, 0xdeae6943_7402_4141_a5c6_0c01aa57b16a);
impl windows_core::RuntimeType for IInputInjectorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputInjectorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputInjectorStatics2, IInputInjectorStatics2_Vtbl, 0xa4db38fb_dd8c_414f_95ea_f87ef4c0ae6c);
impl windows_core::RuntimeType for IInputInjectorStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputInjectorStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryCreateForAppBroadcastOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InjectedInputGamepadInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InjectedInputGamepadInfo, windows_core::IUnknown, windows_core::IInspectable);
impl InjectedInputGamepadInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InjectedInputGamepadInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Gaming_Input")]
    pub fn Buttons(&self) -> windows_core::Result<super::super::super::super::Gaming::Input::GamepadButtons> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Buttons)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Gaming_Input")]
    pub fn SetButtons(&self, value: super::super::super::super::Gaming::Input::GamepadButtons) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetButtons)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LeftThumbstickX(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LeftThumbstickX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLeftThumbstickX(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLeftThumbstickX)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LeftThumbstickY(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LeftThumbstickY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLeftThumbstickY(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLeftThumbstickY)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LeftTrigger(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LeftTrigger)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLeftTrigger(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLeftTrigger)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RightThumbstickX(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RightThumbstickX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRightThumbstickX(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRightThumbstickX)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RightThumbstickY(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RightThumbstickY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRightThumbstickY(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRightThumbstickY)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RightTrigger(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RightTrigger)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRightTrigger(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRightTrigger)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Gaming_Input")]
    pub fn CreateInstanceFromGamepadReading(reading: super::super::super::super::Gaming::Input::GamepadReading) -> windows_core::Result<InjectedInputGamepadInfo> {
        Self::IInjectedInputGamepadInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceFromGamepadReading)(windows_core::Interface::as_raw(this), reading, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IInjectedInputGamepadInfoFactory<R, F: FnOnce(&IInjectedInputGamepadInfoFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InjectedInputGamepadInfo, IInjectedInputGamepadInfoFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InjectedInputGamepadInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInjectedInputGamepadInfo>();
}
unsafe impl windows_core::Interface for InjectedInputGamepadInfo {
    type Vtable = IInjectedInputGamepadInfo_Vtbl;
    const IID: windows_core::GUID = <IInjectedInputGamepadInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InjectedInputGamepadInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputGamepadInfo";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InjectedInputKeyboardInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InjectedInputKeyboardInfo, windows_core::IUnknown, windows_core::IInspectable);
impl InjectedInputKeyboardInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InjectedInputKeyboardInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn KeyOptions(&self) -> windows_core::Result<InjectedInputKeyOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyOptions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyOptions(&self, value: InjectedInputKeyOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeyOptions)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScanCode(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScanCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScanCode(&self, value: u16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetScanCode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VirtualKey(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VirtualKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVirtualKey(&self, value: u16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVirtualKey)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for InjectedInputKeyboardInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInjectedInputKeyboardInfo>();
}
unsafe impl windows_core::Interface for InjectedInputKeyboardInfo {
    type Vtable = IInjectedInputKeyboardInfo_Vtbl;
    const IID: windows_core::GUID = <IInjectedInputKeyboardInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InjectedInputKeyboardInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputKeyboardInfo";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InjectedInputMouseInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InjectedInputMouseInfo, windows_core::IUnknown, windows_core::IInspectable);
impl InjectedInputMouseInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InjectedInputMouseInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MouseOptions(&self) -> windows_core::Result<InjectedInputMouseOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MouseOptions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMouseOptions(&self, value: InjectedInputMouseOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMouseOptions)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MouseData(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MouseData)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMouseData(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMouseData)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeltaY(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeltaY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDeltaY(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDeltaY)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DeltaX(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeltaX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDeltaX(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDeltaX)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TimeOffsetInMilliseconds(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeOffsetInMilliseconds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTimeOffsetInMilliseconds(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTimeOffsetInMilliseconds)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for InjectedInputMouseInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInjectedInputMouseInfo>();
}
unsafe impl windows_core::Interface for InjectedInputMouseInfo {
    type Vtable = IInjectedInputMouseInfo_Vtbl;
    const IID: windows_core::GUID = <IInjectedInputMouseInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InjectedInputMouseInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputMouseInfo";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InjectedInputPenInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InjectedInputPenInfo, windows_core::IUnknown, windows_core::IInspectable);
impl InjectedInputPenInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InjectedInputPenInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PointerInfo(&self) -> windows_core::Result<InjectedInputPointerInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerInfo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPointerInfo(&self, value: InjectedInputPointerInfo) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPointerInfo)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PenButtons(&self) -> windows_core::Result<InjectedInputPenButtons> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PenButtons)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPenButtons(&self, value: InjectedInputPenButtons) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPenButtons)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PenParameters(&self) -> windows_core::Result<InjectedInputPenParameters> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PenParameters)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPenParameters(&self, value: InjectedInputPenParameters) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPenParameters)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Pressure(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pressure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPressure(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPressure)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TiltX(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TiltX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTiltX(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTiltX)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TiltY(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TiltY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTiltY(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTiltY)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for InjectedInputPenInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInjectedInputPenInfo>();
}
unsafe impl windows_core::Interface for InjectedInputPenInfo {
    type Vtable = IInjectedInputPenInfo_Vtbl;
    const IID: windows_core::GUID = <IInjectedInputPenInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InjectedInputPenInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputPenInfo";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InjectedInputTouchInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InjectedInputTouchInfo, windows_core::IUnknown, windows_core::IInspectable);
impl InjectedInputTouchInfo {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InjectedInputTouchInfo, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Contact(&self) -> windows_core::Result<InjectedInputRectangle> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetContact(&self, value: InjectedInputRectangle) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContact)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Orientation(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Orientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOrientation(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOrientation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PointerInfo(&self) -> windows_core::Result<InjectedInputPointerInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerInfo)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPointerInfo(&self, value: InjectedInputPointerInfo) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPointerInfo)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Pressure(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pressure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPressure(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPressure)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TouchParameters(&self) -> windows_core::Result<InjectedInputTouchParameters> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TouchParameters)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTouchParameters(&self, value: InjectedInputTouchParameters) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTouchParameters)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for InjectedInputTouchInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInjectedInputTouchInfo>();
}
unsafe impl windows_core::Interface for InjectedInputTouchInfo {
    type Vtable = IInjectedInputTouchInfo_Vtbl;
    const IID: windows_core::GUID = <IInjectedInputTouchInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InjectedInputTouchInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputTouchInfo";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InputInjector(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputInjector, windows_core::IUnknown, windows_core::IInspectable);
impl InputInjector {
    #[cfg(feature = "Foundation_Collections")]
    pub fn InjectKeyboardInput<P0>(&self, input: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Collections::IIterable<InjectedInputKeyboardInfo>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InjectKeyboardInput)(windows_core::Interface::as_raw(this), input.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InjectMouseInput<P0>(&self, input: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Collections::IIterable<InjectedInputMouseInfo>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InjectMouseInput)(windows_core::Interface::as_raw(this), input.param().abi()).ok() }
    }
    pub fn InitializeTouchInjection(&self, visualmode: InjectedInputVisualizationMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InitializeTouchInjection)(windows_core::Interface::as_raw(this), visualmode).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InjectTouchInput<P0>(&self, input: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Collections::IIterable<InjectedInputTouchInfo>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InjectTouchInput)(windows_core::Interface::as_raw(this), input.param().abi()).ok() }
    }
    pub fn UninitializeTouchInjection(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UninitializeTouchInjection)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InitializePenInjection(&self, visualmode: InjectedInputVisualizationMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InitializePenInjection)(windows_core::Interface::as_raw(this), visualmode).ok() }
    }
    pub fn InjectPenInput<P0>(&self, input: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InjectedInputPenInfo>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InjectPenInput)(windows_core::Interface::as_raw(this), input.param().abi()).ok() }
    }
    pub fn UninitializePenInjection(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UninitializePenInjection)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InjectShortcut(&self, shortcut: InjectedInputShortcut) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InjectShortcut)(windows_core::Interface::as_raw(this), shortcut).ok() }
    }
    pub fn InitializeGamepadInjection(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInputInjector2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InitializeGamepadInjection)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InjectGamepadInput<P0>(&self, input: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InjectedInputGamepadInfo>,
    {
        let this = &windows_core::Interface::cast::<IInputInjector2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InjectGamepadInput)(windows_core::Interface::as_raw(this), input.param().abi()).ok() }
    }
    pub fn UninitializeGamepadInjection(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInputInjector2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UninitializeGamepadInjection)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TryCreate() -> windows_core::Result<InputInjector> {
        Self::IInputInjectorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryCreateForAppBroadcastOnly() -> windows_core::Result<InputInjector> {
        Self::IInputInjectorStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateForAppBroadcastOnly)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IInputInjectorStatics<R, F: FnOnce(&IInputInjectorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputInjector, IInputInjectorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInputInjectorStatics2<R, F: FnOnce(&IInputInjectorStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputInjector, IInputInjectorStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputInjector {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputInjector>();
}
unsafe impl windows_core::Interface for InputInjector {
    type Vtable = IInputInjector_Vtbl;
    const IID: windows_core::GUID = <IInputInjector as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputInjector {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InputInjector";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InjectedInputButtonChangeKind(pub i32);
impl InjectedInputButtonChangeKind {
    pub const None: Self = Self(0i32);
    pub const FirstButtonDown: Self = Self(1i32);
    pub const FirstButtonUp: Self = Self(2i32);
    pub const SecondButtonDown: Self = Self(3i32);
    pub const SecondButtonUp: Self = Self(4i32);
    pub const ThirdButtonDown: Self = Self(5i32);
    pub const ThirdButtonUp: Self = Self(6i32);
    pub const FourthButtonDown: Self = Self(7i32);
    pub const FourthButtonUp: Self = Self(8i32);
    pub const FifthButtonDown: Self = Self(9i32);
    pub const FifthButtonUp: Self = Self(10i32);
}
impl windows_core::TypeKind for InjectedInputButtonChangeKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InjectedInputButtonChangeKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InjectedInputButtonChangeKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InjectedInputButtonChangeKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputButtonChangeKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InjectedInputKeyOptions(pub u32);
impl InjectedInputKeyOptions {
    pub const None: Self = Self(0u32);
    pub const ExtendedKey: Self = Self(1u32);
    pub const KeyUp: Self = Self(2u32);
    pub const ScanCode: Self = Self(8u32);
    pub const Unicode: Self = Self(4u32);
}
impl windows_core::TypeKind for InjectedInputKeyOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InjectedInputKeyOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InjectedInputKeyOptions").field(&self.0).finish()
    }
}
impl InjectedInputKeyOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for InjectedInputKeyOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for InjectedInputKeyOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for InjectedInputKeyOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for InjectedInputKeyOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for InjectedInputKeyOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for InjectedInputKeyOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputKeyOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InjectedInputMouseOptions(pub u32);
impl InjectedInputMouseOptions {
    pub const None: Self = Self(0u32);
    pub const Move: Self = Self(1u32);
    pub const LeftDown: Self = Self(2u32);
    pub const LeftUp: Self = Self(4u32);
    pub const RightDown: Self = Self(8u32);
    pub const RightUp: Self = Self(16u32);
    pub const MiddleDown: Self = Self(32u32);
    pub const MiddleUp: Self = Self(64u32);
    pub const XDown: Self = Self(128u32);
    pub const XUp: Self = Self(256u32);
    pub const Wheel: Self = Self(2048u32);
    pub const HWheel: Self = Self(4096u32);
    pub const MoveNoCoalesce: Self = Self(8192u32);
    pub const VirtualDesk: Self = Self(16384u32);
    pub const Absolute: Self = Self(32768u32);
}
impl windows_core::TypeKind for InjectedInputMouseOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InjectedInputMouseOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InjectedInputMouseOptions").field(&self.0).finish()
    }
}
impl InjectedInputMouseOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for InjectedInputMouseOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for InjectedInputMouseOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for InjectedInputMouseOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for InjectedInputMouseOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for InjectedInputMouseOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for InjectedInputMouseOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputMouseOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InjectedInputPenButtons(pub u32);
impl InjectedInputPenButtons {
    pub const None: Self = Self(0u32);
    pub const Barrel: Self = Self(1u32);
    pub const Inverted: Self = Self(2u32);
    pub const Eraser: Self = Self(4u32);
}
impl windows_core::TypeKind for InjectedInputPenButtons {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InjectedInputPenButtons {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InjectedInputPenButtons").field(&self.0).finish()
    }
}
impl InjectedInputPenButtons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for InjectedInputPenButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for InjectedInputPenButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for InjectedInputPenButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for InjectedInputPenButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for InjectedInputPenButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for InjectedInputPenButtons {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputPenButtons;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InjectedInputPenParameters(pub u32);
impl InjectedInputPenParameters {
    pub const None: Self = Self(0u32);
    pub const Pressure: Self = Self(1u32);
    pub const Rotation: Self = Self(2u32);
    pub const TiltX: Self = Self(4u32);
    pub const TiltY: Self = Self(8u32);
}
impl windows_core::TypeKind for InjectedInputPenParameters {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InjectedInputPenParameters {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InjectedInputPenParameters").field(&self.0).finish()
    }
}
impl InjectedInputPenParameters {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for InjectedInputPenParameters {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for InjectedInputPenParameters {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for InjectedInputPenParameters {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for InjectedInputPenParameters {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for InjectedInputPenParameters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for InjectedInputPenParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputPenParameters;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InjectedInputPointerOptions(pub u32);
impl InjectedInputPointerOptions {
    pub const None: Self = Self(0u32);
    pub const New: Self = Self(1u32);
    pub const InRange: Self = Self(2u32);
    pub const InContact: Self = Self(4u32);
    pub const FirstButton: Self = Self(16u32);
    pub const SecondButton: Self = Self(32u32);
    pub const Primary: Self = Self(8192u32);
    pub const Confidence: Self = Self(16384u32);
    pub const Canceled: Self = Self(32768u32);
    pub const PointerDown: Self = Self(65536u32);
    pub const Update: Self = Self(131072u32);
    pub const PointerUp: Self = Self(262144u32);
    pub const CaptureChanged: Self = Self(2097152u32);
}
impl windows_core::TypeKind for InjectedInputPointerOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InjectedInputPointerOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InjectedInputPointerOptions").field(&self.0).finish()
    }
}
impl InjectedInputPointerOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for InjectedInputPointerOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for InjectedInputPointerOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for InjectedInputPointerOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for InjectedInputPointerOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for InjectedInputPointerOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for InjectedInputPointerOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputPointerOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InjectedInputShortcut(pub i32);
impl InjectedInputShortcut {
    pub const Back: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const Search: Self = Self(2i32);
}
impl windows_core::TypeKind for InjectedInputShortcut {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InjectedInputShortcut {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InjectedInputShortcut").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InjectedInputShortcut {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputShortcut;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InjectedInputTouchParameters(pub u32);
impl InjectedInputTouchParameters {
    pub const None: Self = Self(0u32);
    pub const Contact: Self = Self(1u32);
    pub const Orientation: Self = Self(2u32);
    pub const Pressure: Self = Self(4u32);
}
impl windows_core::TypeKind for InjectedInputTouchParameters {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InjectedInputTouchParameters {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InjectedInputTouchParameters").field(&self.0).finish()
    }
}
impl InjectedInputTouchParameters {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for InjectedInputTouchParameters {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for InjectedInputTouchParameters {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for InjectedInputTouchParameters {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for InjectedInputTouchParameters {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for InjectedInputTouchParameters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for InjectedInputTouchParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputTouchParameters;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InjectedInputVisualizationMode(pub i32);
impl InjectedInputVisualizationMode {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1i32);
    pub const Indirect: Self = Self(2i32);
}
impl windows_core::TypeKind for InjectedInputVisualizationMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InjectedInputVisualizationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InjectedInputVisualizationMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for InjectedInputVisualizationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputVisualizationMode;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InjectedInputPoint {
    pub PositionX: i32,
    pub PositionY: i32,
}
impl windows_core::TypeKind for InjectedInputPoint {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for InjectedInputPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Input.Preview.Injection.InjectedInputPoint;i4;i4)");
}
impl Default for InjectedInputPoint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InjectedInputPointerInfo {
    pub PointerId: u32,
    pub PointerOptions: InjectedInputPointerOptions,
    pub PixelLocation: InjectedInputPoint,
    pub TimeOffsetInMilliseconds: u32,
    pub PerformanceCount: u64,
}
impl windows_core::TypeKind for InjectedInputPointerInfo {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for InjectedInputPointerInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Input.Preview.Injection.InjectedInputPointerInfo;u4;enum(Windows.UI.Input.Preview.Injection.InjectedInputPointerOptions;u4);struct(Windows.UI.Input.Preview.Injection.InjectedInputPoint;i4;i4);u4;u8)");
}
impl Default for InjectedInputPointerInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InjectedInputRectangle {
    pub Left: i32,
    pub Top: i32,
    pub Bottom: i32,
    pub Right: i32,
}
impl windows_core::TypeKind for InjectedInputRectangle {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for InjectedInputRectangle {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Input.Preview.Injection.InjectedInputRectangle;i4;i4;i4;i4)");
}
impl Default for InjectedInputRectangle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
