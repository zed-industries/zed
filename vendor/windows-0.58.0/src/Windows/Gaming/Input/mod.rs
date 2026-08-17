#[cfg(feature = "Gaming_Input_Custom")]
pub mod Custom;
#[cfg(feature = "Gaming_Input_ForceFeedback")]
pub mod ForceFeedback;
#[cfg(feature = "Gaming_Input_Preview")]
pub mod Preview;
windows_core::imp::define_interface!(IArcadeStick, IArcadeStick_Vtbl, 0xb14a539d_befb_4c81_8051_15ecf3b13036);
impl windows_core::RuntimeType for IArcadeStick {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IArcadeStick_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetButtonLabel: unsafe extern "system" fn(*mut core::ffi::c_void, ArcadeStickButtons, *mut GameControllerButtonLabel) -> windows_core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ArcadeStickReading) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IArcadeStickStatics, IArcadeStickStatics_Vtbl, 0x5c37b8c8_37b1_4ad8_9458_200f1a30018e);
impl windows_core::RuntimeType for IArcadeStickStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IArcadeStickStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ArcadeStickAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveArcadeStickAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ArcadeStickRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveArcadeStickRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ArcadeSticks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ArcadeSticks: usize,
}
windows_core::imp::define_interface!(IArcadeStickStatics2, IArcadeStickStatics2_Vtbl, 0x52b5d744_bb86_445a_b59c_596f0e2a49df);
impl windows_core::RuntimeType for IArcadeStickStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IArcadeStickStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromGameController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFlightStick, IFlightStick_Vtbl, 0xb4a2c01c_b83b_4459_a1a9_97b03c33da7c);
impl windows_core::RuntimeType for IFlightStick {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFlightStick_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HatSwitchKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameControllerSwitchKind) -> windows_core::HRESULT,
    pub GetButtonLabel: unsafe extern "system" fn(*mut core::ffi::c_void, FlightStickButtons, *mut GameControllerButtonLabel) -> windows_core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FlightStickReading) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFlightStickStatics, IFlightStickStatics_Vtbl, 0x5514924a_fecc_435e_83dc_5cec8a18a520);
impl windows_core::RuntimeType for IFlightStickStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IFlightStickStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FlightStickAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFlightStickAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub FlightStickRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFlightStickRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FlightSticks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FlightSticks: usize,
    pub FromGameController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameController, IGameController_Vtbl, 0x1baf6522_5f64_42c5_8267_b9fe2215bfbd);
impl core::ops::Deref for IGameController {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGameController, windows_core::IUnknown, windows_core::IInspectable);
impl IGameController {
    pub fn HeadsetConnected<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, Headset>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadsetConnected)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHeadsetConnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveHeadsetConnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn HeadsetDisconnected<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, Headset>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadsetDisconnected)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHeadsetDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveHeadsetDisconnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "System")]
    pub fn UserChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUserChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUserChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Headset(&self) -> windows_core::Result<Headset> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Headset)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsWireless(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWireless)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IGameController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HeadsetConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveHeadsetConnected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub HeadsetDisconnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveHeadsetDisconnected: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub UserChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    UserChanged: usize,
    pub RemoveUserChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Headset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsWireless: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IGameControllerBatteryInfo, IGameControllerBatteryInfo_Vtbl, 0xdcecc681_3963_4da6_955d_553f3b6f6161);
impl core::ops::Deref for IGameControllerBatteryInfo {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGameControllerBatteryInfo, windows_core::IUnknown, windows_core::IInspectable);
impl IGameControllerBatteryInfo {
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> windows_core::Result<super::super::Devices::Power::BatteryReport> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetBatteryReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IGameControllerBatteryInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameControllerBatteryInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Power")]
    pub TryGetBatteryReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Power"))]
    TryGetBatteryReport: usize,
}
windows_core::imp::define_interface!(IGamepad, IGamepad_Vtbl, 0xbc7bb43c_0a69_3903_9e9d_a50f86a45de5);
impl windows_core::RuntimeType for IGamepad {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGamepad_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Vibration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GamepadVibration) -> windows_core::HRESULT,
    pub SetVibration: unsafe extern "system" fn(*mut core::ffi::c_void, GamepadVibration) -> windows_core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GamepadReading) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGamepad2, IGamepad2_Vtbl, 0x3c1689bd_5915_4245_b0c0_c89fae0308ff);
impl windows_core::RuntimeType for IGamepad2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGamepad2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetButtonLabel: unsafe extern "system" fn(*mut core::ffi::c_void, GamepadButtons, *mut GameControllerButtonLabel) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGamepadStatics, IGamepadStatics_Vtbl, 0x8bbce529_d49c_39e9_9560_e47dde96b7c8);
impl windows_core::RuntimeType for IGamepadStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGamepadStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GamepadAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveGamepadAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub GamepadRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveGamepadRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Gamepads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Gamepads: usize,
}
windows_core::imp::define_interface!(IGamepadStatics2, IGamepadStatics2_Vtbl, 0x42676dc5_0856_47c4_9213_b395504c3a3c);
impl windows_core::RuntimeType for IGamepadStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGamepadStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromGameController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHeadset, IHeadset_Vtbl, 0x3fd156ef_6925_3fa8_9181_029c5223ae3b);
impl windows_core::RuntimeType for IHeadset {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHeadset_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CaptureDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RenderDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRacingWheel, IRacingWheel_Vtbl, 0xf546656f_e106_4c82_a90f_554012904b85);
impl windows_core::RuntimeType for IRacingWheel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRacingWheel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HasClutch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasHandbrake: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasPatternShifter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MaxPatternShifterGear: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub MaxWheelAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(feature = "Gaming_Input_ForceFeedback")]
    pub WheelMotor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_ForceFeedback"))]
    WheelMotor: usize,
    pub GetButtonLabel: unsafe extern "system" fn(*mut core::ffi::c_void, RacingWheelButtons, *mut GameControllerButtonLabel) -> windows_core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RacingWheelReading) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRacingWheelStatics, IRacingWheelStatics_Vtbl, 0x3ac12cd5_581b_4936_9f94_69f1e6514c7d);
impl windows_core::RuntimeType for IRacingWheelStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRacingWheelStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RacingWheelAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRacingWheelAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RacingWheelRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRacingWheelRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RacingWheels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RacingWheels: usize,
}
windows_core::imp::define_interface!(IRacingWheelStatics2, IRacingWheelStatics2_Vtbl, 0xe666bcaa_edfd_4323_a9f6_3c384048d1ed);
impl windows_core::RuntimeType for IRacingWheelStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRacingWheelStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromGameController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRawGameController, IRawGameController_Vtbl, 0x7cad6d91_a7e1_4f71_9a78_33e9c5dfea62);
impl windows_core::RuntimeType for IRawGameController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRawGameController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AxisCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ButtonCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback"))]
    pub ForceFeedbackMotors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback")))]
    ForceFeedbackMotors: usize,
    pub HardwareProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub HardwareVendorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SwitchCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetButtonLabel: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut GameControllerButtonLabel) -> windows_core::HRESULT,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut bool, u32, *mut GameControllerSwitchPosition, u32, *mut f64, *mut u64) -> windows_core::HRESULT,
    pub GetSwitchKind: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut GameControllerSwitchKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRawGameController2, IRawGameController2_Vtbl, 0x43c0c035_bb73_4756_a787_3ed6bea617bd);
impl windows_core::RuntimeType for IRawGameController2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRawGameController2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Haptics", feature = "Foundation_Collections"))]
    pub SimpleHapticsControllers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Haptics", feature = "Foundation_Collections")))]
    SimpleHapticsControllers: usize,
    pub NonRoamableId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRawGameControllerStatics, IRawGameControllerStatics_Vtbl, 0xeb8d0792_e95a_4b19_afc7_0a59f8bf759e);
impl windows_core::RuntimeType for IRawGameControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRawGameControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RawGameControllerAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRawGameControllerAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RawGameControllerRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRawGameControllerRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RawGameControllers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RawGameControllers: usize,
    pub FromGameController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUINavigationController, IUINavigationController_Vtbl, 0xe5aeefdd_f50e_4a55_8cdc_d33229548175);
impl windows_core::RuntimeType for IUINavigationController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUINavigationController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentReading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UINavigationReading) -> windows_core::HRESULT,
    pub GetOptionalButtonLabel: unsafe extern "system" fn(*mut core::ffi::c_void, OptionalUINavigationButtons, *mut GameControllerButtonLabel) -> windows_core::HRESULT,
    pub GetRequiredButtonLabel: unsafe extern "system" fn(*mut core::ffi::c_void, RequiredUINavigationButtons, *mut GameControllerButtonLabel) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUINavigationControllerStatics, IUINavigationControllerStatics_Vtbl, 0x2f14930a_f6f8_4a48_8d89_94786cca0c2e);
impl windows_core::RuntimeType for IUINavigationControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUINavigationControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UINavigationControllerAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUINavigationControllerAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub UINavigationControllerRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUINavigationControllerRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UINavigationControllers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UINavigationControllers: usize,
}
windows_core::imp::define_interface!(IUINavigationControllerStatics2, IUINavigationControllerStatics2_Vtbl, 0xe0cb28e3_b20b_4b0b_9ed4_f3d53cec0de4);
impl windows_core::RuntimeType for IUINavigationControllerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUINavigationControllerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromGameController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ArcadeStick(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ArcadeStick, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ArcadeStick, IGameController, IGameControllerBatteryInfo);
impl ArcadeStick {
    pub fn GetButtonLabel(&self, button: ArcadeStickButtons) -> windows_core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetButtonLabel)(windows_core::Interface::as_raw(this), button, &mut result__).map(|| result__)
        }
    }
    pub fn GetCurrentReading(&self) -> windows_core::Result<ArcadeStickReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ArcadeStickAdded<P0>(value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<ArcadeStick>>,
    {
        Self::IArcadeStickStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArcadeStickAdded)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveArcadeStickAdded(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IArcadeStickStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveArcadeStickAdded)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn ArcadeStickRemoved<P0>(value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<ArcadeStick>>,
    {
        Self::IArcadeStickStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArcadeStickRemoved)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveArcadeStickRemoved(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IArcadeStickStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveArcadeStickRemoved)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ArcadeSticks() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ArcadeStick>> {
        Self::IArcadeStickStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArcadeSticks)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromGameController<P0>(gamecontroller: P0) -> windows_core::Result<ArcadeStick>
    where
        P0: windows_core::Param<IGameController>,
    {
        Self::IArcadeStickStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromGameController)(windows_core::Interface::as_raw(this), gamecontroller.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HeadsetConnected<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, Headset>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadsetConnected)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHeadsetConnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveHeadsetConnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn HeadsetDisconnected<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, Headset>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadsetDisconnected)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHeadsetDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveHeadsetDisconnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "System")]
    pub fn UserChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUserChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveUserChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Headset(&self) -> windows_core::Result<Headset> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Headset)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsWireless(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWireless)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> windows_core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &windows_core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetBatteryReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[doc(hidden)]
    pub fn IArcadeStickStatics<R, F: FnOnce(&IArcadeStickStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ArcadeStick, IArcadeStickStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IArcadeStickStatics2<R, F: FnOnce(&IArcadeStickStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ArcadeStick, IArcadeStickStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ArcadeStick {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IArcadeStick>();
}
unsafe impl windows_core::Interface for ArcadeStick {
    type Vtable = IArcadeStick_Vtbl;
    const IID: windows_core::GUID = <IArcadeStick as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ArcadeStick {
    const NAME: &'static str = "Windows.Gaming.Input.ArcadeStick";
}
unsafe impl Send for ArcadeStick {}
unsafe impl Sync for ArcadeStick {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FlightStick(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FlightStick, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(FlightStick, IGameController, IGameControllerBatteryInfo);
impl FlightStick {
    pub fn HatSwitchKind(&self) -> windows_core::Result<GameControllerSwitchKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HatSwitchKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetButtonLabel(&self, button: FlightStickButtons) -> windows_core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetButtonLabel)(windows_core::Interface::as_raw(this), button, &mut result__).map(|| result__)
        }
    }
    pub fn GetCurrentReading(&self) -> windows_core::Result<FlightStickReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FlightStickAdded<P0>(value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<FlightStick>>,
    {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlightStickAdded)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveFlightStickAdded(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IFlightStickStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveFlightStickAdded)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn FlightStickRemoved<P0>(value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<FlightStick>>,
    {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlightStickRemoved)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveFlightStickRemoved(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IFlightStickStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveFlightStickRemoved)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FlightSticks() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<FlightStick>> {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlightSticks)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromGameController<P0>(gamecontroller: P0) -> windows_core::Result<FlightStick>
    where
        P0: windows_core::Param<IGameController>,
    {
        Self::IFlightStickStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromGameController)(windows_core::Interface::as_raw(this), gamecontroller.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn HeadsetConnected<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, Headset>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadsetConnected)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHeadsetConnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveHeadsetConnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn HeadsetDisconnected<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, Headset>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadsetDisconnected)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHeadsetDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveHeadsetDisconnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "System")]
    pub fn UserChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUserChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveUserChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Headset(&self) -> windows_core::Result<Headset> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Headset)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsWireless(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWireless)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> windows_core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &windows_core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetBatteryReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[doc(hidden)]
    pub fn IFlightStickStatics<R, F: FnOnce(&IFlightStickStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FlightStick, IFlightStickStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FlightStick {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFlightStick>();
}
unsafe impl windows_core::Interface for FlightStick {
    type Vtable = IFlightStick_Vtbl;
    const IID: windows_core::GUID = <IFlightStick as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FlightStick {
    const NAME: &'static str = "Windows.Gaming.Input.FlightStick";
}
unsafe impl Send for FlightStick {}
unsafe impl Sync for FlightStick {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Gamepad(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Gamepad, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Gamepad, IGameController, IGameControllerBatteryInfo);
impl Gamepad {
    pub fn HeadsetConnected<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, Headset>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadsetConnected)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHeadsetConnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveHeadsetConnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn HeadsetDisconnected<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, Headset>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadsetDisconnected)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHeadsetDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveHeadsetDisconnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "System")]
    pub fn UserChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUserChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveUserChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Headset(&self) -> windows_core::Result<Headset> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Headset)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsWireless(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWireless)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> windows_core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &windows_core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetBatteryReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Vibration(&self) -> windows_core::Result<GamepadVibration> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Vibration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVibration(&self, value: GamepadVibration) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVibration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetCurrentReading(&self) -> windows_core::Result<GamepadReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetButtonLabel(&self, button: GamepadButtons) -> windows_core::Result<GameControllerButtonLabel> {
        let this = &windows_core::Interface::cast::<IGamepad2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetButtonLabel)(windows_core::Interface::as_raw(this), button, &mut result__).map(|| result__)
        }
    }
    pub fn GamepadAdded<P0>(value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<Gamepad>>,
    {
        Self::IGamepadStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GamepadAdded)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveGamepadAdded(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IGamepadStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveGamepadAdded)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn GamepadRemoved<P0>(value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<Gamepad>>,
    {
        Self::IGamepadStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GamepadRemoved)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveGamepadRemoved(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IGamepadStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveGamepadRemoved)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Gamepads() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<Gamepad>> {
        Self::IGamepadStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Gamepads)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromGameController<P0>(gamecontroller: P0) -> windows_core::Result<Gamepad>
    where
        P0: windows_core::Param<IGameController>,
    {
        Self::IGamepadStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromGameController)(windows_core::Interface::as_raw(this), gamecontroller.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGamepadStatics<R, F: FnOnce(&IGamepadStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Gamepad, IGamepadStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGamepadStatics2<R, F: FnOnce(&IGamepadStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Gamepad, IGamepadStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Gamepad {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGamepad>();
}
unsafe impl windows_core::Interface for Gamepad {
    type Vtable = IGamepad_Vtbl;
    const IID: windows_core::GUID = <IGamepad as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Gamepad {
    const NAME: &'static str = "Windows.Gaming.Input.Gamepad";
}
unsafe impl Send for Gamepad {}
unsafe impl Sync for Gamepad {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Headset(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Headset, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Headset, IGameControllerBatteryInfo);
impl Headset {
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> windows_core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &windows_core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetBatteryReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CaptureDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CaptureDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RenderDeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderDeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Headset {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHeadset>();
}
unsafe impl windows_core::Interface for Headset {
    type Vtable = IHeadset_Vtbl;
    const IID: windows_core::GUID = <IHeadset as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Headset {
    const NAME: &'static str = "Windows.Gaming.Input.Headset";
}
unsafe impl Send for Headset {}
unsafe impl Sync for Headset {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RacingWheel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RacingWheel, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RacingWheel, IGameController, IGameControllerBatteryInfo);
impl RacingWheel {
    pub fn HeadsetConnected<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, Headset>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadsetConnected)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHeadsetConnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveHeadsetConnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn HeadsetDisconnected<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, Headset>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadsetDisconnected)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHeadsetDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveHeadsetDisconnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "System")]
    pub fn UserChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUserChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveUserChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Headset(&self) -> windows_core::Result<Headset> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Headset)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsWireless(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWireless)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> windows_core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &windows_core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetBatteryReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasClutch(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasClutch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasHandbrake(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasHandbrake)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasPatternShifter(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasPatternShifter)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxPatternShifterGear(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPatternShifterGear)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxWheelAngle(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWheelAngle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Gaming_Input_ForceFeedback")]
    pub fn WheelMotor(&self) -> windows_core::Result<ForceFeedback::ForceFeedbackMotor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WheelMotor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetButtonLabel(&self, button: RacingWheelButtons) -> windows_core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetButtonLabel)(windows_core::Interface::as_raw(this), button, &mut result__).map(|| result__)
        }
    }
    pub fn GetCurrentReading(&self) -> windows_core::Result<RacingWheelReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RacingWheelAdded<P0>(value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<RacingWheel>>,
    {
        Self::IRacingWheelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RacingWheelAdded)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveRacingWheelAdded(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IRacingWheelStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveRacingWheelAdded)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn RacingWheelRemoved<P0>(value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<RacingWheel>>,
    {
        Self::IRacingWheelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RacingWheelRemoved)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveRacingWheelRemoved(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IRacingWheelStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveRacingWheelRemoved)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RacingWheels() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<RacingWheel>> {
        Self::IRacingWheelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RacingWheels)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromGameController<P0>(gamecontroller: P0) -> windows_core::Result<RacingWheel>
    where
        P0: windows_core::Param<IGameController>,
    {
        Self::IRacingWheelStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromGameController)(windows_core::Interface::as_raw(this), gamecontroller.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRacingWheelStatics<R, F: FnOnce(&IRacingWheelStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RacingWheel, IRacingWheelStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRacingWheelStatics2<R, F: FnOnce(&IRacingWheelStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RacingWheel, IRacingWheelStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RacingWheel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRacingWheel>();
}
unsafe impl windows_core::Interface for RacingWheel {
    type Vtable = IRacingWheel_Vtbl;
    const IID: windows_core::GUID = <IRacingWheel as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RacingWheel {
    const NAME: &'static str = "Windows.Gaming.Input.RacingWheel";
}
unsafe impl Send for RacingWheel {}
unsafe impl Sync for RacingWheel {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RawGameController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RawGameController, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RawGameController, IGameController, IGameControllerBatteryInfo);
impl RawGameController {
    pub fn HeadsetConnected<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, Headset>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadsetConnected)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHeadsetConnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveHeadsetConnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn HeadsetDisconnected<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, Headset>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadsetDisconnected)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHeadsetDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveHeadsetDisconnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "System")]
    pub fn UserChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUserChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveUserChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Headset(&self) -> windows_core::Result<Headset> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Headset)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsWireless(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWireless)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> windows_core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &windows_core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetBatteryReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AxisCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AxisCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ButtonCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ButtonCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Gaming_Input_ForceFeedback"))]
    pub fn ForceFeedbackMotors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ForceFeedback::ForceFeedbackMotor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ForceFeedbackMotors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HardwareProductId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareProductId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HardwareVendorId(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareVendorId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SwitchCount(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SwitchCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetButtonLabel(&self, buttonindex: i32) -> windows_core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetButtonLabel)(windows_core::Interface::as_raw(this), buttonindex, &mut result__).map(|| result__)
        }
    }
    pub fn GetCurrentReading(&self, buttonarray: &mut [bool], switcharray: &mut [GameControllerSwitchPosition], axisarray: &mut [f64]) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), buttonarray.len().try_into().unwrap(), buttonarray.as_mut_ptr(), switcharray.len().try_into().unwrap(), switcharray.as_mut_ptr(), axisarray.len().try_into().unwrap(), axisarray.as_mut_ptr(), &mut result__).map(|| result__)
        }
    }
    pub fn GetSwitchKind(&self, switchindex: i32) -> windows_core::Result<GameControllerSwitchKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSwitchKind)(windows_core::Interface::as_raw(this), switchindex, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Devices_Haptics", feature = "Foundation_Collections"))]
    pub fn SimpleHapticsControllers(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Devices::Haptics::SimpleHapticsController>> {
        let this = &windows_core::Interface::cast::<IRawGameController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimpleHapticsControllers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NonRoamableId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IRawGameController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NonRoamableId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IRawGameController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RawGameControllerAdded<P0>(value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<RawGameController>>,
    {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawGameControllerAdded)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveRawGameControllerAdded(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IRawGameControllerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveRawGameControllerAdded)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn RawGameControllerRemoved<P0>(value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<RawGameController>>,
    {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawGameControllerRemoved)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveRawGameControllerRemoved(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IRawGameControllerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveRawGameControllerRemoved)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RawGameControllers() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<RawGameController>> {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawGameControllers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromGameController<P0>(gamecontroller: P0) -> windows_core::Result<RawGameController>
    where
        P0: windows_core::Param<IGameController>,
    {
        Self::IRawGameControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromGameController)(windows_core::Interface::as_raw(this), gamecontroller.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRawGameControllerStatics<R, F: FnOnce(&IRawGameControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RawGameController, IRawGameControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RawGameController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRawGameController>();
}
unsafe impl windows_core::Interface for RawGameController {
    type Vtable = IRawGameController_Vtbl;
    const IID: windows_core::GUID = <IRawGameController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RawGameController {
    const NAME: &'static str = "Windows.Gaming.Input.RawGameController";
}
unsafe impl Send for RawGameController {}
unsafe impl Sync for RawGameController {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct UINavigationController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UINavigationController, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(UINavigationController, IGameController, IGameControllerBatteryInfo);
impl UINavigationController {
    pub fn HeadsetConnected<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, Headset>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadsetConnected)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHeadsetConnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveHeadsetConnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn HeadsetDisconnected<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, Headset>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeadsetDisconnected)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveHeadsetDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveHeadsetDisconnected)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "System")]
    pub fn UserChanged<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IGameController, super::super::System::UserChangedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserChanged)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUserChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveUserChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Headset(&self) -> windows_core::Result<Headset> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Headset)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsWireless(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWireless)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = &windows_core::Interface::cast::<IGameController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Power")]
    pub fn TryGetBatteryReport(&self) -> windows_core::Result<super::super::Devices::Power::BatteryReport> {
        let this = &windows_core::Interface::cast::<IGameControllerBatteryInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetBatteryReport)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentReading(&self) -> windows_core::Result<UINavigationReading> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentReading)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetOptionalButtonLabel(&self, button: OptionalUINavigationButtons) -> windows_core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOptionalButtonLabel)(windows_core::Interface::as_raw(this), button, &mut result__).map(|| result__)
        }
    }
    pub fn GetRequiredButtonLabel(&self, button: RequiredUINavigationButtons) -> windows_core::Result<GameControllerButtonLabel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRequiredButtonLabel)(windows_core::Interface::as_raw(this), button, &mut result__).map(|| result__)
        }
    }
    pub fn UINavigationControllerAdded<P0>(value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<UINavigationController>>,
    {
        Self::IUINavigationControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UINavigationControllerAdded)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveUINavigationControllerAdded(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IUINavigationControllerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveUINavigationControllerAdded)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn UINavigationControllerRemoved<P0>(value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<UINavigationController>>,
    {
        Self::IUINavigationControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UINavigationControllerRemoved)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveUINavigationControllerRemoved(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IUINavigationControllerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveUINavigationControllerRemoved)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UINavigationControllers() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<UINavigationController>> {
        Self::IUINavigationControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UINavigationControllers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromGameController<P0>(gamecontroller: P0) -> windows_core::Result<UINavigationController>
    where
        P0: windows_core::Param<IGameController>,
    {
        Self::IUINavigationControllerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromGameController)(windows_core::Interface::as_raw(this), gamecontroller.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUINavigationControllerStatics<R, F: FnOnce(&IUINavigationControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UINavigationController, IUINavigationControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUINavigationControllerStatics2<R, F: FnOnce(&IUINavigationControllerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UINavigationController, IUINavigationControllerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UINavigationController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUINavigationController>();
}
unsafe impl windows_core::Interface for UINavigationController {
    type Vtable = IUINavigationController_Vtbl;
    const IID: windows_core::GUID = <IUINavigationController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UINavigationController {
    const NAME: &'static str = "Windows.Gaming.Input.UINavigationController";
}
unsafe impl Send for UINavigationController {}
unsafe impl Sync for UINavigationController {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ArcadeStickButtons(pub u32);
impl ArcadeStickButtons {
    pub const None: Self = Self(0u32);
    pub const StickUp: Self = Self(1u32);
    pub const StickDown: Self = Self(2u32);
    pub const StickLeft: Self = Self(4u32);
    pub const StickRight: Self = Self(8u32);
    pub const Action1: Self = Self(16u32);
    pub const Action2: Self = Self(32u32);
    pub const Action3: Self = Self(64u32);
    pub const Action4: Self = Self(128u32);
    pub const Action5: Self = Self(256u32);
    pub const Action6: Self = Self(512u32);
    pub const Special1: Self = Self(1024u32);
    pub const Special2: Self = Self(2048u32);
}
impl windows_core::TypeKind for ArcadeStickButtons {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ArcadeStickButtons {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ArcadeStickButtons").field(&self.0).finish()
    }
}
impl ArcadeStickButtons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ArcadeStickButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ArcadeStickButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ArcadeStickButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ArcadeStickButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ArcadeStickButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for ArcadeStickButtons {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ArcadeStickButtons;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FlightStickButtons(pub u32);
impl FlightStickButtons {
    pub const None: Self = Self(0u32);
    pub const FirePrimary: Self = Self(1u32);
    pub const FireSecondary: Self = Self(2u32);
}
impl windows_core::TypeKind for FlightStickButtons {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FlightStickButtons {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FlightStickButtons").field(&self.0).finish()
    }
}
impl FlightStickButtons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FlightStickButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FlightStickButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FlightStickButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FlightStickButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FlightStickButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for FlightStickButtons {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.FlightStickButtons;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GameControllerButtonLabel(pub i32);
impl GameControllerButtonLabel {
    pub const None: Self = Self(0i32);
    pub const XboxBack: Self = Self(1i32);
    pub const XboxStart: Self = Self(2i32);
    pub const XboxMenu: Self = Self(3i32);
    pub const XboxView: Self = Self(4i32);
    pub const XboxUp: Self = Self(5i32);
    pub const XboxDown: Self = Self(6i32);
    pub const XboxLeft: Self = Self(7i32);
    pub const XboxRight: Self = Self(8i32);
    pub const XboxA: Self = Self(9i32);
    pub const XboxB: Self = Self(10i32);
    pub const XboxX: Self = Self(11i32);
    pub const XboxY: Self = Self(12i32);
    pub const XboxLeftBumper: Self = Self(13i32);
    pub const XboxLeftTrigger: Self = Self(14i32);
    pub const XboxLeftStickButton: Self = Self(15i32);
    pub const XboxRightBumper: Self = Self(16i32);
    pub const XboxRightTrigger: Self = Self(17i32);
    pub const XboxRightStickButton: Self = Self(18i32);
    pub const XboxPaddle1: Self = Self(19i32);
    pub const XboxPaddle2: Self = Self(20i32);
    pub const XboxPaddle3: Self = Self(21i32);
    pub const XboxPaddle4: Self = Self(22i32);
    pub const Mode: Self = Self(23i32);
    pub const Select: Self = Self(24i32);
    pub const Menu: Self = Self(25i32);
    pub const View: Self = Self(26i32);
    pub const Back: Self = Self(27i32);
    pub const Start: Self = Self(28i32);
    pub const Options: Self = Self(29i32);
    pub const Share: Self = Self(30i32);
    pub const Up: Self = Self(31i32);
    pub const Down: Self = Self(32i32);
    pub const Left: Self = Self(33i32);
    pub const Right: Self = Self(34i32);
    pub const LetterA: Self = Self(35i32);
    pub const LetterB: Self = Self(36i32);
    pub const LetterC: Self = Self(37i32);
    pub const LetterL: Self = Self(38i32);
    pub const LetterR: Self = Self(39i32);
    pub const LetterX: Self = Self(40i32);
    pub const LetterY: Self = Self(41i32);
    pub const LetterZ: Self = Self(42i32);
    pub const Cross: Self = Self(43i32);
    pub const Circle: Self = Self(44i32);
    pub const Square: Self = Self(45i32);
    pub const Triangle: Self = Self(46i32);
    pub const LeftBumper: Self = Self(47i32);
    pub const LeftTrigger: Self = Self(48i32);
    pub const LeftStickButton: Self = Self(49i32);
    pub const Left1: Self = Self(50i32);
    pub const Left2: Self = Self(51i32);
    pub const Left3: Self = Self(52i32);
    pub const RightBumper: Self = Self(53i32);
    pub const RightTrigger: Self = Self(54i32);
    pub const RightStickButton: Self = Self(55i32);
    pub const Right1: Self = Self(56i32);
    pub const Right2: Self = Self(57i32);
    pub const Right3: Self = Self(58i32);
    pub const Paddle1: Self = Self(59i32);
    pub const Paddle2: Self = Self(60i32);
    pub const Paddle3: Self = Self(61i32);
    pub const Paddle4: Self = Self(62i32);
    pub const Plus: Self = Self(63i32);
    pub const Minus: Self = Self(64i32);
    pub const DownLeftArrow: Self = Self(65i32);
    pub const DialLeft: Self = Self(66i32);
    pub const DialRight: Self = Self(67i32);
    pub const Suspension: Self = Self(68i32);
}
impl windows_core::TypeKind for GameControllerButtonLabel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GameControllerButtonLabel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GameControllerButtonLabel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GameControllerButtonLabel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GameControllerButtonLabel;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GameControllerSwitchKind(pub i32);
impl GameControllerSwitchKind {
    pub const TwoWay: Self = Self(0i32);
    pub const FourWay: Self = Self(1i32);
    pub const EightWay: Self = Self(2i32);
}
impl windows_core::TypeKind for GameControllerSwitchKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GameControllerSwitchKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GameControllerSwitchKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GameControllerSwitchKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GameControllerSwitchKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GameControllerSwitchPosition(pub i32);
impl GameControllerSwitchPosition {
    pub const Center: Self = Self(0i32);
    pub const Up: Self = Self(1i32);
    pub const UpRight: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const DownRight: Self = Self(4i32);
    pub const Down: Self = Self(5i32);
    pub const DownLeft: Self = Self(6i32);
    pub const Left: Self = Self(7i32);
    pub const UpLeft: Self = Self(8i32);
}
impl windows_core::TypeKind for GameControllerSwitchPosition {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GameControllerSwitchPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GameControllerSwitchPosition").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GameControllerSwitchPosition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GameControllerSwitchPosition;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GamepadButtons(pub u32);
impl GamepadButtons {
    pub const None: Self = Self(0u32);
    pub const Menu: Self = Self(1u32);
    pub const View: Self = Self(2u32);
    pub const A: Self = Self(4u32);
    pub const B: Self = Self(8u32);
    pub const X: Self = Self(16u32);
    pub const Y: Self = Self(32u32);
    pub const DPadUp: Self = Self(64u32);
    pub const DPadDown: Self = Self(128u32);
    pub const DPadLeft: Self = Self(256u32);
    pub const DPadRight: Self = Self(512u32);
    pub const LeftShoulder: Self = Self(1024u32);
    pub const RightShoulder: Self = Self(2048u32);
    pub const LeftThumbstick: Self = Self(4096u32);
    pub const RightThumbstick: Self = Self(8192u32);
    pub const Paddle1: Self = Self(16384u32);
    pub const Paddle2: Self = Self(32768u32);
    pub const Paddle3: Self = Self(65536u32);
    pub const Paddle4: Self = Self(131072u32);
}
impl windows_core::TypeKind for GamepadButtons {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GamepadButtons {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GamepadButtons").field(&self.0).finish()
    }
}
impl GamepadButtons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GamepadButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GamepadButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GamepadButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GamepadButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GamepadButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for GamepadButtons {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.GamepadButtons;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OptionalUINavigationButtons(pub u32);
impl OptionalUINavigationButtons {
    pub const None: Self = Self(0u32);
    pub const Context1: Self = Self(1u32);
    pub const Context2: Self = Self(2u32);
    pub const Context3: Self = Self(4u32);
    pub const Context4: Self = Self(8u32);
    pub const PageUp: Self = Self(16u32);
    pub const PageDown: Self = Self(32u32);
    pub const PageLeft: Self = Self(64u32);
    pub const PageRight: Self = Self(128u32);
    pub const ScrollUp: Self = Self(256u32);
    pub const ScrollDown: Self = Self(512u32);
    pub const ScrollLeft: Self = Self(1024u32);
    pub const ScrollRight: Self = Self(2048u32);
}
impl windows_core::TypeKind for OptionalUINavigationButtons {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OptionalUINavigationButtons {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OptionalUINavigationButtons").field(&self.0).finish()
    }
}
impl OptionalUINavigationButtons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for OptionalUINavigationButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for OptionalUINavigationButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for OptionalUINavigationButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for OptionalUINavigationButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for OptionalUINavigationButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for OptionalUINavigationButtons {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.OptionalUINavigationButtons;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RacingWheelButtons(pub u32);
impl RacingWheelButtons {
    pub const None: Self = Self(0u32);
    pub const PreviousGear: Self = Self(1u32);
    pub const NextGear: Self = Self(2u32);
    pub const DPadUp: Self = Self(4u32);
    pub const DPadDown: Self = Self(8u32);
    pub const DPadLeft: Self = Self(16u32);
    pub const DPadRight: Self = Self(32u32);
    pub const Button1: Self = Self(64u32);
    pub const Button2: Self = Self(128u32);
    pub const Button3: Self = Self(256u32);
    pub const Button4: Self = Self(512u32);
    pub const Button5: Self = Self(1024u32);
    pub const Button6: Self = Self(2048u32);
    pub const Button7: Self = Self(4096u32);
    pub const Button8: Self = Self(8192u32);
    pub const Button9: Self = Self(16384u32);
    pub const Button10: Self = Self(32768u32);
    pub const Button11: Self = Self(65536u32);
    pub const Button12: Self = Self(131072u32);
    pub const Button13: Self = Self(262144u32);
    pub const Button14: Self = Self(524288u32);
    pub const Button15: Self = Self(1048576u32);
    pub const Button16: Self = Self(2097152u32);
}
impl windows_core::TypeKind for RacingWheelButtons {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RacingWheelButtons {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RacingWheelButtons").field(&self.0).finish()
    }
}
impl RacingWheelButtons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RacingWheelButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RacingWheelButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RacingWheelButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RacingWheelButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RacingWheelButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for RacingWheelButtons {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.RacingWheelButtons;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RequiredUINavigationButtons(pub u32);
impl RequiredUINavigationButtons {
    pub const None: Self = Self(0u32);
    pub const Menu: Self = Self(1u32);
    pub const View: Self = Self(2u32);
    pub const Accept: Self = Self(4u32);
    pub const Cancel: Self = Self(8u32);
    pub const Up: Self = Self(16u32);
    pub const Down: Self = Self(32u32);
    pub const Left: Self = Self(64u32);
    pub const Right: Self = Self(128u32);
}
impl windows_core::TypeKind for RequiredUINavigationButtons {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RequiredUINavigationButtons {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RequiredUINavigationButtons").field(&self.0).finish()
    }
}
impl RequiredUINavigationButtons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RequiredUINavigationButtons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RequiredUINavigationButtons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RequiredUINavigationButtons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RequiredUINavigationButtons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RequiredUINavigationButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for RequiredUINavigationButtons {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.RequiredUINavigationButtons;u4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArcadeStickReading {
    pub Timestamp: u64,
    pub Buttons: ArcadeStickButtons,
}
impl windows_core::TypeKind for ArcadeStickReading {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ArcadeStickReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.ArcadeStickReading;u8;enum(Windows.Gaming.Input.ArcadeStickButtons;u4))");
}
impl Default for ArcadeStickReading {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlightStickReading {
    pub Timestamp: u64,
    pub Buttons: FlightStickButtons,
    pub HatSwitch: GameControllerSwitchPosition,
    pub Roll: f64,
    pub Pitch: f64,
    pub Yaw: f64,
    pub Throttle: f64,
}
impl windows_core::TypeKind for FlightStickReading {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for FlightStickReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.FlightStickReading;u8;enum(Windows.Gaming.Input.FlightStickButtons;u4);enum(Windows.Gaming.Input.GameControllerSwitchPosition;i4);f8;f8;f8;f8)");
}
impl Default for FlightStickReading {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GamepadReading {
    pub Timestamp: u64,
    pub Buttons: GamepadButtons,
    pub LeftTrigger: f64,
    pub RightTrigger: f64,
    pub LeftThumbstickX: f64,
    pub LeftThumbstickY: f64,
    pub RightThumbstickX: f64,
    pub RightThumbstickY: f64,
}
impl windows_core::TypeKind for GamepadReading {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GamepadReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.GamepadReading;u8;enum(Windows.Gaming.Input.GamepadButtons;u4);f8;f8;f8;f8;f8;f8)");
}
impl Default for GamepadReading {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GamepadVibration {
    pub LeftMotor: f64,
    pub RightMotor: f64,
    pub LeftTrigger: f64,
    pub RightTrigger: f64,
}
impl windows_core::TypeKind for GamepadVibration {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GamepadVibration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.GamepadVibration;f8;f8;f8;f8)");
}
impl Default for GamepadVibration {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RacingWheelReading {
    pub Timestamp: u64,
    pub Buttons: RacingWheelButtons,
    pub PatternShifterGear: i32,
    pub Wheel: f64,
    pub Throttle: f64,
    pub Brake: f64,
    pub Clutch: f64,
    pub Handbrake: f64,
}
impl windows_core::TypeKind for RacingWheelReading {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for RacingWheelReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.RacingWheelReading;u8;enum(Windows.Gaming.Input.RacingWheelButtons;u4);i4;f8;f8;f8;f8;f8)");
}
impl Default for RacingWheelReading {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UINavigationReading {
    pub Timestamp: u64,
    pub RequiredButtons: RequiredUINavigationButtons,
    pub OptionalButtons: OptionalUINavigationButtons,
}
impl windows_core::TypeKind for UINavigationReading {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for UINavigationReading {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.UINavigationReading;u8;enum(Windows.Gaming.Input.RequiredUINavigationButtons;u4);enum(Windows.Gaming.Input.OptionalUINavigationButtons;u4))");
}
impl Default for UINavigationReading {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
