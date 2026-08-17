#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DeviceCommand(pub i32);
impl DeviceCommand {
    pub const Reset: Self = Self(0i32);
}
impl windows_core::TypeKind for DeviceCommand {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DeviceCommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Preview.DeviceCommand;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameControllerBatteryChargingState(pub i32);
impl GameControllerBatteryChargingState {
    pub const Unknown: Self = Self(0i32);
    pub const Inactive: Self = Self(1i32);
    pub const Active: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
}
impl windows_core::TypeKind for GameControllerBatteryChargingState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GameControllerBatteryChargingState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Preview.GameControllerBatteryChargingState;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameControllerBatteryKind(pub i32);
impl GameControllerBatteryKind {
    pub const Unknown: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Standard: Self = Self(2i32);
    pub const Rechargeable: Self = Self(3i32);
}
impl windows_core::TypeKind for GameControllerBatteryKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GameControllerBatteryKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Preview.GameControllerBatteryKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameControllerBatteryLevel(pub i32);
impl GameControllerBatteryLevel {
    pub const Unknown: Self = Self(0i32);
    pub const Critical: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
    pub const Medium: Self = Self(3i32);
    pub const Full: Self = Self(4i32);
}
impl windows_core::TypeKind for GameControllerBatteryLevel {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GameControllerBatteryLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Preview.GameControllerBatteryLevel;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GameControllerFirmwareCorruptReason(pub i32);
impl GameControllerFirmwareCorruptReason {
    pub const Unknown: Self = Self(0i32);
    pub const NotCorrupt: Self = Self(1i32);
    pub const TwoUpCorrupt: Self = Self(2i32);
    pub const AppCorrupt: Self = Self(3i32);
    pub const RadioCorrupt: Self = Self(4i32);
    pub const EepromCorrupt: Self = Self(5i32);
    pub const SafeToUpdate: Self = Self(6i32);
}
impl windows_core::TypeKind for GameControllerFirmwareCorruptReason {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GameControllerFirmwareCorruptReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Preview.GameControllerFirmwareCorruptReason;i4)");
}
pub struct GameControllerProviderInfo;
impl GameControllerProviderInfo {
    #[cfg(feature = "Gaming_Input_Custom")]
    pub fn GetParentProviderId<P0>(provider: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::Custom::IGameControllerProvider>,
    {
        Self::IGameControllerProviderInfoStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetParentProviderId)(windows_core::Interface::as_raw(this), provider.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    #[cfg(feature = "Gaming_Input_Custom")]
    pub fn GetProviderId<P0>(provider: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::Custom::IGameControllerProvider>,
    {
        Self::IGameControllerProviderInfoStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetProviderId)(windows_core::Interface::as_raw(this), provider.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    fn IGameControllerProviderInfoStatics<R, F: FnOnce(&IGameControllerProviderInfoStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GameControllerProviderInfo, IGameControllerProviderInfoStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for GameControllerProviderInfo {
    const NAME: &'static str = "Windows.Gaming.Input.Preview.GameControllerProviderInfo";
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HeadsetGeqGains {
    pub band1Gain: i32,
    pub band2Gain: i32,
    pub band3Gain: i32,
    pub band4Gain: i32,
    pub band5Gain: i32,
}
impl windows_core::TypeKind for HeadsetGeqGains {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HeadsetGeqGains {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.Preview.HeadsetGeqGains;i4;i4;i4;i4;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HeadsetLevel(pub i32);
impl HeadsetLevel {
    pub const Off: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
    pub const Medium: Self = Self(2i32);
    pub const High: Self = Self(3i32);
}
impl windows_core::TypeKind for HeadsetLevel {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HeadsetLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Preview.HeadsetLevel;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HeadsetOperation(pub i32);
impl HeadsetOperation {
    pub const Geq: Self = Self(0i32);
    pub const BassBoostGain: Self = Self(1i32);
    pub const SmartMute: Self = Self(2i32);
    pub const SideTone: Self = Self(3i32);
    pub const MuteLedBrightness: Self = Self(4i32);
    pub const SwapMixAndVolumeDials: Self = Self(5i32);
}
impl windows_core::TypeKind for HeadsetOperation {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HeadsetOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Preview.HeadsetOperation;i4)");
}
windows_core::imp::define_interface!(IGameControllerProviderInfoStatics, IGameControllerProviderInfoStatics_Vtbl, 0x0be1e6c5_d9bd_44ee_8362_488b2e464bfb);
impl windows_core::RuntimeType for IGameControllerProviderInfoStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerProviderInfoStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Gaming_Input_Custom")]
    pub GetParentProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_Custom"))]
    GetParentProviderId: usize,
    #[cfg(feature = "Gaming_Input_Custom")]
    pub GetProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_Custom"))]
    GetProviderId: usize,
}
windows_core::imp::define_interface!(ILegacyGipGameControllerProvider, ILegacyGipGameControllerProvider_Vtbl, 0x2da3ed52_ffd9_43e2_825c_1d2790e04d14);
impl windows_core::RuntimeType for ILegacyGipGameControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILegacyGipGameControllerProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BatteryChargingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameControllerBatteryChargingState) -> windows_core::HRESULT,
    pub BatteryKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameControllerBatteryKind) -> windows_core::HRESULT,
    pub BatteryLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameControllerBatteryLevel) -> windows_core::HRESULT,
    pub GetDeviceFirmwareCorruptionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameControllerFirmwareCorruptReason) -> windows_core::HRESULT,
    pub IsFirmwareCorrupted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInterfaceSupported: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut bool) -> windows_core::HRESULT,
    pub IsSyntheticDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PreferredTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecuteCommand: unsafe extern "system" fn(*mut core::ffi::c_void, DeviceCommand) -> windows_core::HRESULT,
    pub SetHomeLedIntensity: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub GetExtendedDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub SetHeadsetOperation: unsafe extern "system" fn(*mut core::ffi::c_void, HeadsetOperation, u32, *const u8) -> windows_core::HRESULT,
    pub GetHeadsetOperation: unsafe extern "system" fn(*mut core::ffi::c_void, HeadsetOperation, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub AppCompatVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub SetStandardControllerButtonRemapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetStandardControllerButtonRemapping: usize,
    #[cfg(feature = "System")]
    pub GetStandardControllerButtonRemapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetStandardControllerButtonRemapping: usize,
}
windows_core::imp::define_interface!(ILegacyGipGameControllerProviderStatics, ILegacyGipGameControllerProviderStatics_Vtbl, 0xd40dda17_b1f4_499a_874c_7095aac15291);
impl windows_core::RuntimeType for ILegacyGipGameControllerProviderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ILegacyGipGameControllerProviderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromGameController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Gaming_Input_Custom")]
    pub FromGameControllerProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_Custom"))]
    FromGameControllerProvider: usize,
    #[cfg(feature = "System")]
    pub PairPilotToCopilot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    PairPilotToCopilot: usize,
    #[cfg(feature = "System")]
    pub ClearPairing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ClearPairing: usize,
    #[cfg(feature = "System")]
    pub IsPilot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    IsPilot: usize,
    #[cfg(feature = "System")]
    pub IsCopilot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    IsCopilot: usize,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegacyGipGameControllerProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LegacyGipGameControllerProvider, windows_core::IUnknown, windows_core::IInspectable);
impl LegacyGipGameControllerProvider {
    pub fn BatteryChargingState(&self) -> windows_core::Result<GameControllerBatteryChargingState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BatteryChargingState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BatteryKind(&self) -> windows_core::Result<GameControllerBatteryKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BatteryKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BatteryLevel(&self) -> windows_core::Result<GameControllerBatteryLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BatteryLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDeviceFirmwareCorruptionState(&self) -> windows_core::Result<GameControllerFirmwareCorruptReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceFirmwareCorruptionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsFirmwareCorrupted(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFirmwareCorrupted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsInterfaceSupported(&self, interfaceid: windows_core::GUID) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInterfaceSupported)(windows_core::Interface::as_raw(this), interfaceid, &mut result__).map(|| result__)
        }
    }
    pub fn IsSyntheticDevice(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSyntheticDevice)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PreferredTypes(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreferredTypes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExecuteCommand(&self, command: DeviceCommand) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ExecuteCommand)(windows_core::Interface::as_raw(this), command).ok() }
    }
    pub fn SetHomeLedIntensity(&self, intensity: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHomeLedIntensity)(windows_core::Interface::as_raw(this), intensity).ok() }
    }
    pub fn GetExtendedDeviceInfo(&self) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetExtendedDeviceInfo)(windows_core::Interface::as_raw(this), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn SetHeadsetOperation(&self, operation: HeadsetOperation, buffer: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHeadsetOperation)(windows_core::Interface::as_raw(this), operation, buffer.len().try_into().unwrap(), buffer.as_ptr()).ok() }
    }
    pub fn GetHeadsetOperation(&self, operation: HeadsetOperation) -> windows_core::Result<windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetHeadsetOperation)(windows_core::Interface::as_raw(this), operation, windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn AppCompatVersion(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppCompatVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn SetStandardControllerButtonRemapping<P0, P2>(&self, user: P0, previous: bool, remapping: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::User>,
        P2: windows_core::Param<windows_collections::IMapView<RemappingButtonCategory, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetStandardControllerButtonRemapping)(windows_core::Interface::as_raw(this), user.param().abi(), previous, remapping.param().abi()).ok() }
    }
    #[cfg(feature = "System")]
    pub fn GetStandardControllerButtonRemapping<P0>(&self, user: P0, previous: bool) -> windows_core::Result<windows_collections::IMapView<RemappingButtonCategory, windows_core::IInspectable>>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStandardControllerButtonRemapping)(windows_core::Interface::as_raw(this), user.param().abi(), previous, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromGameController<P0>(controller: P0) -> windows_core::Result<LegacyGipGameControllerProvider>
    where
        P0: windows_core::Param<super::IGameController>,
    {
        Self::ILegacyGipGameControllerProviderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromGameController)(windows_core::Interface::as_raw(this), controller.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Gaming_Input_Custom")]
    pub fn FromGameControllerProvider<P0>(provider: P0) -> windows_core::Result<LegacyGipGameControllerProvider>
    where
        P0: windows_core::Param<super::Custom::IGameControllerProvider>,
    {
        Self::ILegacyGipGameControllerProviderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromGameControllerProvider)(windows_core::Interface::as_raw(this), provider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn PairPilotToCopilot<P0>(user: P0, pilotcontrollerproviderid: &windows_core::HSTRING, copilotcontrollerproviderid: &windows_core::HSTRING) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::ILegacyGipGameControllerProviderStatics(|this| unsafe { (windows_core::Interface::vtable(this).PairPilotToCopilot)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(pilotcontrollerproviderid), core::mem::transmute_copy(copilotcontrollerproviderid)).ok() })
    }
    #[cfg(feature = "System")]
    pub fn ClearPairing<P0>(user: P0, controllerproviderid: &windows_core::HSTRING) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::ILegacyGipGameControllerProviderStatics(|this| unsafe { (windows_core::Interface::vtable(this).ClearPairing)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(controllerproviderid)).ok() })
    }
    #[cfg(feature = "System")]
    pub fn IsPilot<P0>(user: P0, controllerproviderid: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::ILegacyGipGameControllerProviderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPilot)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(controllerproviderid), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn IsCopilot<P0>(user: P0, controllerproviderid: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::super::super::System::User>,
    {
        Self::ILegacyGipGameControllerProviderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCopilot)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(controllerproviderid), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    fn ILegacyGipGameControllerProviderStatics<R, F: FnOnce(&ILegacyGipGameControllerProviderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LegacyGipGameControllerProvider, ILegacyGipGameControllerProviderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LegacyGipGameControllerProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILegacyGipGameControllerProvider>();
}
unsafe impl windows_core::Interface for LegacyGipGameControllerProvider {
    type Vtable = <ILegacyGipGameControllerProvider as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILegacyGipGameControllerProvider as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LegacyGipGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Preview.LegacyGipGameControllerProvider";
}
unsafe impl Send for LegacyGipGameControllerProvider {}
unsafe impl Sync for LegacyGipGameControllerProvider {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RemappingButtonCategory(pub i32);
impl RemappingButtonCategory {
    pub const ButtonSettings: Self = Self(0i32);
    pub const AnalogSettings: Self = Self(1i32);
    pub const VibrationSettings: Self = Self(2i32);
    pub const ShareShortPress: Self = Self(3i32);
    pub const ShareShortPressMetaData: Self = Self(4i32);
    pub const ShareShortPressMetaDataDisplay: Self = Self(5i32);
    pub const ShareLongPress: Self = Self(6i32);
    pub const ShareLongPressMetaData: Self = Self(7i32);
    pub const ShareLongPressMetaDataDisplay: Self = Self(8i32);
    pub const ShareDoublePress: Self = Self(9i32);
    pub const ShareDoublePressMetaData: Self = Self(10i32);
    pub const ShareDoublePressMetaDataDisplay: Self = Self(11i32);
}
impl windows_core::TypeKind for RemappingButtonCategory {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for RemappingButtonCategory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Preview.RemappingButtonCategory;i4)");
}
