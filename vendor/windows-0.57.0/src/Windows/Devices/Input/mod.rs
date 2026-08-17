#[cfg(feature = "Devices_Input_Preview")]
pub mod Preview;
windows_core::imp::define_interface!(IKeyboardCapabilities, IKeyboardCapabilities_Vtbl, 0x3a3f9b56_6798_4bbc_833e_0f34b17c65ff);
impl windows_core::RuntimeType for IKeyboardCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKeyboardCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KeyboardPresent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMouseCapabilities, IMouseCapabilities_Vtbl, 0xbca5e023_7dd9_4b6b_9a92_55d43cb38f73);
impl windows_core::RuntimeType for IMouseCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMouseCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MousePresent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub VerticalWheelPresent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub HorizontalWheelPresent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SwapButtons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfButtons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMouseDevice, IMouseDevice_Vtbl, 0x88edf458_f2c8_49f4_be1f_c256b388bc11);
impl windows_core::RuntimeType for IMouseDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMouseDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MouseMoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMouseMoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMouseDeviceStatics, IMouseDeviceStatics_Vtbl, 0x484a9045_6d70_49db_8e68_46ffbd17d38d);
impl windows_core::RuntimeType for IMouseDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMouseDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMouseEventArgs, IMouseEventArgs_Vtbl, 0xf625aa5d_2354_4cc7_9230_96941c969fde);
impl windows_core::RuntimeType for IMouseEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMouseEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MouseDelta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MouseDelta) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPenButtonListener, IPenButtonListener_Vtbl, 0x8245c376_1ee3_53f7_b1f7_8334a16f2815);
impl windows_core::RuntimeType for IPenButtonListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPenButtonListener_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSupportedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveIsSupportedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TailButtonClicked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTailButtonClicked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TailButtonDoubleClicked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTailButtonDoubleClicked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub TailButtonLongPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTailButtonLongPressed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPenButtonListenerStatics, IPenButtonListenerStatics_Vtbl, 0x19a8a584_862f_5f69_bfea_05f6584f133f);
impl windows_core::RuntimeType for IPenButtonListenerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPenButtonListenerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPenDevice, IPenDevice_Vtbl, 0x31856eba_a738_5a8c_b8f6_f97ef68d18ef);
impl windows_core::RuntimeType for IPenDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPenDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PenId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPenDevice2, IPenDevice2_Vtbl, 0x0207d327_7fb8_5566_8c34_f8342037b7f9);
impl windows_core::RuntimeType for IPenDevice2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPenDevice2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Haptics")]
    pub SimpleHapticsController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Haptics"))]
    SimpleHapticsController: usize,
}
windows_core::imp::define_interface!(IPenDeviceStatics, IPenDeviceStatics_Vtbl, 0x9dfbbe01_0966_5180_bcb4_b85060e39479);
impl windows_core::RuntimeType for IPenDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPenDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetFromPointerId: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPenDockListener, IPenDockListener_Vtbl, 0x759f4d90_1dc0_55cb_ad18_b9101456f592);
impl windows_core::RuntimeType for IPenDockListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPenDockListener_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsSupportedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveIsSupportedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Docked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDocked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Undocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUndocked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPenDockListenerStatics, IPenDockListenerStatics_Vtbl, 0xcab75e9a_0016_5c72_969e_a97e11992a93);
impl windows_core::RuntimeType for IPenDockListenerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPenDockListenerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPenDockedEventArgs, IPenDockedEventArgs_Vtbl, 0xfd4277c6_ca63_5d4e_9ed3_a28a54521c8c);
impl windows_core::RuntimeType for IPenDockedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPenDockedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPenTailButtonClickedEventArgs, IPenTailButtonClickedEventArgs_Vtbl, 0x5d2fb7b6_6ad3_5d3e_ab29_05ea2410e390);
impl windows_core::RuntimeType for IPenTailButtonClickedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPenTailButtonClickedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPenTailButtonDoubleClickedEventArgs, IPenTailButtonDoubleClickedEventArgs_Vtbl, 0x846321a2_618a_5478_b04c_b358231da4a7);
impl windows_core::RuntimeType for IPenTailButtonDoubleClickedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPenTailButtonDoubleClickedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPenTailButtonLongPressedEventArgs, IPenTailButtonLongPressedEventArgs_Vtbl, 0xf37c606e_c60a_5f42_b818_a53112406c13);
impl windows_core::RuntimeType for IPenTailButtonLongPressedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPenTailButtonLongPressedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPenUndockedEventArgs, IPenUndockedEventArgs_Vtbl, 0xccd09150_261b_59e6_a5d4_c1964cd03feb);
impl windows_core::RuntimeType for IPenUndockedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPenUndockedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPointerDevice, IPointerDevice_Vtbl, 0x93c9bafc_ebcb_467e_82c6_276feae36b5a);
impl windows_core::RuntimeType for IPointerDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPointerDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PointerDeviceType) -> windows_core::HRESULT,
    pub IsIntegrated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MaxContacts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PhysicalDeviceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub ScreenRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Rect) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedUsages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedUsages: usize,
}
windows_core::imp::define_interface!(IPointerDevice2, IPointerDevice2_Vtbl, 0xf8a6d2a0_c484_489f_ae3e_30d2ee1ffd3e);
impl windows_core::RuntimeType for IPointerDevice2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPointerDevice2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxPointersWithZDistance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerDeviceStatics, IPointerDeviceStatics_Vtbl, 0xd8b89aa1_d1c6_416e_bd8d_5790914dc563);
impl windows_core::RuntimeType for IPointerDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPointerDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetPointerDevice: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPointerDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPointerDevices: usize,
}
windows_core::imp::define_interface!(ITouchCapabilities, ITouchCapabilities_Vtbl, 0x20dd55f9_13f1_46c8_9285_2c05fa3eda6f);
impl windows_core::RuntimeType for ITouchCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITouchCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TouchPresent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Contacts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct KeyboardCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(KeyboardCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl KeyboardCapabilities {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KeyboardCapabilities, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn KeyboardPresent(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardPresent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for KeyboardCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IKeyboardCapabilities>();
}
unsafe impl windows_core::Interface for KeyboardCapabilities {
    type Vtable = IKeyboardCapabilities_Vtbl;
    const IID: windows_core::GUID = <IKeyboardCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for KeyboardCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.KeyboardCapabilities";
}
unsafe impl Send for KeyboardCapabilities {}
unsafe impl Sync for KeyboardCapabilities {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MouseCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MouseCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl MouseCapabilities {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MouseCapabilities, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MousePresent(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MousePresent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VerticalWheelPresent(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalWheelPresent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HorizontalWheelPresent(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalWheelPresent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SwapButtons(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SwapButtons)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NumberOfButtons(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberOfButtons)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MouseCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMouseCapabilities>();
}
unsafe impl windows_core::Interface for MouseCapabilities {
    type Vtable = IMouseCapabilities_Vtbl;
    const IID: windows_core::GUID = <IMouseCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MouseCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.MouseCapabilities";
}
unsafe impl Send for MouseCapabilities {}
unsafe impl Sync for MouseCapabilities {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MouseDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MouseDevice, windows_core::IUnknown, windows_core::IInspectable);
impl MouseDevice {
    pub fn MouseMoved<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MouseDevice, MouseEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MouseMoved)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMouseMoved(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMouseMoved)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<MouseDevice> {
        Self::IMouseDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMouseDeviceStatics<R, F: FnOnce(&IMouseDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MouseDevice, IMouseDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MouseDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMouseDevice>();
}
unsafe impl windows_core::Interface for MouseDevice {
    type Vtable = IMouseDevice_Vtbl;
    const IID: windows_core::GUID = <IMouseDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MouseDevice {
    const NAME: &'static str = "Windows.Devices.Input.MouseDevice";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MouseEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MouseEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MouseEventArgs {
    pub fn MouseDelta(&self) -> windows_core::Result<MouseDelta> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MouseDelta)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MouseEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMouseEventArgs>();
}
unsafe impl windows_core::Interface for MouseEventArgs {
    type Vtable = IMouseEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMouseEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MouseEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.MouseEventArgs";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PenButtonListener(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PenButtonListener, windows_core::IUnknown, windows_core::IInspectable);
impl PenButtonListener {
    pub fn IsSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSupportedChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PenButtonListener, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupportedChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveIsSupportedChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveIsSupportedChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TailButtonClicked<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonClickedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TailButtonClicked)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTailButtonClicked(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTailButtonClicked)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TailButtonDoubleClicked<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonDoubleClickedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TailButtonDoubleClicked)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTailButtonDoubleClicked(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTailButtonDoubleClicked)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TailButtonLongPressed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PenButtonListener, PenTailButtonLongPressedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TailButtonLongPressed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTailButtonLongPressed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTailButtonLongPressed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefault() -> windows_core::Result<PenButtonListener> {
        Self::IPenButtonListenerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPenButtonListenerStatics<R, F: FnOnce(&IPenButtonListenerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PenButtonListener, IPenButtonListenerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PenButtonListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPenButtonListener>();
}
unsafe impl windows_core::Interface for PenButtonListener {
    type Vtable = IPenButtonListener_Vtbl;
    const IID: windows_core::GUID = <IPenButtonListener as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PenButtonListener {
    const NAME: &'static str = "Windows.Devices.Input.PenButtonListener";
}
unsafe impl Send for PenButtonListener {}
unsafe impl Sync for PenButtonListener {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PenDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PenDevice, windows_core::IUnknown, windows_core::IInspectable);
impl PenDevice {
    pub fn PenId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PenId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Devices_Haptics")]
    pub fn SimpleHapticsController(&self) -> windows_core::Result<super::Haptics::SimpleHapticsController> {
        let this = &windows_core::Interface::cast::<IPenDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimpleHapticsController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetFromPointerId(pointerid: u32) -> windows_core::Result<PenDevice> {
        Self::IPenDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFromPointerId)(windows_core::Interface::as_raw(this), pointerid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPenDeviceStatics<R, F: FnOnce(&IPenDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PenDevice, IPenDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PenDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPenDevice>();
}
unsafe impl windows_core::Interface for PenDevice {
    type Vtable = IPenDevice_Vtbl;
    const IID: windows_core::GUID = <IPenDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PenDevice {
    const NAME: &'static str = "Windows.Devices.Input.PenDevice";
}
unsafe impl Send for PenDevice {}
unsafe impl Sync for PenDevice {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PenDockListener(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PenDockListener, windows_core::IUnknown, windows_core::IInspectable);
impl PenDockListener {
    pub fn IsSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsSupportedChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PenDockListener, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupportedChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveIsSupportedChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveIsSupportedChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Docked<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PenDockListener, PenDockedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Docked)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDocked(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDocked)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Undocked<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PenDockListener, PenUndockedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Undocked)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUndocked(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUndocked)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefault() -> windows_core::Result<PenDockListener> {
        Self::IPenDockListenerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPenDockListenerStatics<R, F: FnOnce(&IPenDockListenerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PenDockListener, IPenDockListenerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PenDockListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPenDockListener>();
}
unsafe impl windows_core::Interface for PenDockListener {
    type Vtable = IPenDockListener_Vtbl;
    const IID: windows_core::GUID = <IPenDockListener as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PenDockListener {
    const NAME: &'static str = "Windows.Devices.Input.PenDockListener";
}
unsafe impl Send for PenDockListener {}
unsafe impl Sync for PenDockListener {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PenDockedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PenDockedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PenDockedEventArgs {}
impl windows_core::RuntimeType for PenDockedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPenDockedEventArgs>();
}
unsafe impl windows_core::Interface for PenDockedEventArgs {
    type Vtable = IPenDockedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPenDockedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PenDockedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenDockedEventArgs";
}
unsafe impl Send for PenDockedEventArgs {}
unsafe impl Sync for PenDockedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PenTailButtonClickedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PenTailButtonClickedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PenTailButtonClickedEventArgs {}
impl windows_core::RuntimeType for PenTailButtonClickedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPenTailButtonClickedEventArgs>();
}
unsafe impl windows_core::Interface for PenTailButtonClickedEventArgs {
    type Vtable = IPenTailButtonClickedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPenTailButtonClickedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PenTailButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenTailButtonClickedEventArgs";
}
unsafe impl Send for PenTailButtonClickedEventArgs {}
unsafe impl Sync for PenTailButtonClickedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PenTailButtonDoubleClickedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PenTailButtonDoubleClickedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PenTailButtonDoubleClickedEventArgs {}
impl windows_core::RuntimeType for PenTailButtonDoubleClickedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPenTailButtonDoubleClickedEventArgs>();
}
unsafe impl windows_core::Interface for PenTailButtonDoubleClickedEventArgs {
    type Vtable = IPenTailButtonDoubleClickedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPenTailButtonDoubleClickedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PenTailButtonDoubleClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenTailButtonDoubleClickedEventArgs";
}
unsafe impl Send for PenTailButtonDoubleClickedEventArgs {}
unsafe impl Sync for PenTailButtonDoubleClickedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PenTailButtonLongPressedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PenTailButtonLongPressedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PenTailButtonLongPressedEventArgs {}
impl windows_core::RuntimeType for PenTailButtonLongPressedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPenTailButtonLongPressedEventArgs>();
}
unsafe impl windows_core::Interface for PenTailButtonLongPressedEventArgs {
    type Vtable = IPenTailButtonLongPressedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPenTailButtonLongPressedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PenTailButtonLongPressedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenTailButtonLongPressedEventArgs";
}
unsafe impl Send for PenTailButtonLongPressedEventArgs {}
unsafe impl Sync for PenTailButtonLongPressedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PenUndockedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PenUndockedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PenUndockedEventArgs {}
impl windows_core::RuntimeType for PenUndockedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPenUndockedEventArgs>();
}
unsafe impl windows_core::Interface for PenUndockedEventArgs {
    type Vtable = IPenUndockedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPenUndockedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PenUndockedEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.PenUndockedEventArgs";
}
unsafe impl Send for PenUndockedEventArgs {}
unsafe impl Sync for PenUndockedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PointerDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PointerDevice, windows_core::IUnknown, windows_core::IInspectable);
impl PointerDevice {
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerDeviceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsIntegrated(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIntegrated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxContacts(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxContacts)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PhysicalDeviceRect(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhysicalDeviceRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ScreenRect(&self) -> windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScreenRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedUsages(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PointerDeviceUsage>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedUsages)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxPointersWithZDistance(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IPointerDevice2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPointersWithZDistance)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetPointerDevice(pointerid: u32) -> windows_core::Result<PointerDevice> {
        Self::IPointerDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPointerDevice)(windows_core::Interface::as_raw(this), pointerid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPointerDevices() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<PointerDevice>> {
        Self::IPointerDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPointerDevices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPointerDeviceStatics<R, F: FnOnce(&IPointerDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PointerDevice, IPointerDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PointerDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPointerDevice>();
}
unsafe impl windows_core::Interface for PointerDevice {
    type Vtable = IPointerDevice_Vtbl;
    const IID: windows_core::GUID = <IPointerDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PointerDevice {
    const NAME: &'static str = "Windows.Devices.Input.PointerDevice";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TouchCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TouchCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl TouchCapabilities {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TouchCapabilities, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TouchPresent(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TouchPresent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Contacts(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contacts)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TouchCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITouchCapabilities>();
}
unsafe impl windows_core::Interface for TouchCapabilities {
    type Vtable = ITouchCapabilities_Vtbl;
    const IID: windows_core::GUID = <ITouchCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TouchCapabilities {
    const NAME: &'static str = "Windows.Devices.Input.TouchCapabilities";
}
unsafe impl Send for TouchCapabilities {}
unsafe impl Sync for TouchCapabilities {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PointerDeviceType(pub i32);
impl PointerDeviceType {
    pub const Touch: Self = Self(0i32);
    pub const Pen: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
}
impl windows_core::TypeKind for PointerDeviceType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PointerDeviceType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PointerDeviceType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PointerDeviceType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Input.PointerDeviceType;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MouseDelta {
    pub X: i32,
    pub Y: i32,
}
impl windows_core::TypeKind for MouseDelta {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for MouseDelta {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Input.MouseDelta;i4;i4)");
}
impl Default for MouseDelta {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointerDeviceUsage {
    pub UsagePage: u32,
    pub Usage: u32,
    pub MinLogical: i32,
    pub MaxLogical: i32,
    pub MinPhysical: i32,
    pub MaxPhysical: i32,
    pub Unit: u32,
    pub PhysicalMultiplier: f32,
}
impl windows_core::TypeKind for PointerDeviceUsage {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PointerDeviceUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Input.PointerDeviceUsage;u4;u4;i4;i4;i4;i4;u4;f4)");
}
impl Default for PointerDeviceUsage {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
