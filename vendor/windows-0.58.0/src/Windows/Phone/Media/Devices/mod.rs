windows_core::imp::define_interface!(IAudioRoutingManager, IAudioRoutingManager_Vtbl, 0x79340d20_71cc_4526_9f29_fc8d2486418b);
impl windows_core::RuntimeType for IAudioRoutingManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioRoutingManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAudioEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioRoutingEndpoint) -> windows_core::HRESULT,
    pub SetAudioEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, AudioRoutingEndpoint) -> windows_core::HRESULT,
    pub AudioEndpointChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAudioEndpointChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AvailableAudioEndpoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AvailableAudioRoutingEndpoints) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioRoutingManagerStatics, IAudioRoutingManagerStatics_Vtbl, 0x977fb2a4_5590_4a6f_adde_6a3d0ad58250);
impl windows_core::RuntimeType for IAudioRoutingManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAudioRoutingManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AudioRoutingManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AudioRoutingManager, windows_core::IUnknown, windows_core::IInspectable);
impl AudioRoutingManager {
    pub fn GetAudioEndpoint(&self) -> windows_core::Result<AudioRoutingEndpoint> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAudioEndpoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAudioEndpoint(&self, endpoint: AudioRoutingEndpoint) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAudioEndpoint)(windows_core::Interface::as_raw(this), endpoint).ok() }
    }
    pub fn AudioEndpointChanged<P0>(&self, endpointchangehandler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<AudioRoutingManager, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioEndpointChanged)(windows_core::Interface::as_raw(this), endpointchangehandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAudioEndpointChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAudioEndpointChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AvailableAudioEndpoints(&self) -> windows_core::Result<AvailableAudioRoutingEndpoints> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AvailableAudioEndpoints)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDefault() -> windows_core::Result<AudioRoutingManager> {
        Self::IAudioRoutingManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IAudioRoutingManagerStatics<R, F: FnOnce(&IAudioRoutingManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AudioRoutingManager, IAudioRoutingManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AudioRoutingManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAudioRoutingManager>();
}
unsafe impl windows_core::Interface for AudioRoutingManager {
    type Vtable = IAudioRoutingManager_Vtbl;
    const IID: windows_core::GUID = <IAudioRoutingManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AudioRoutingManager {
    const NAME: &'static str = "Windows.Phone.Media.Devices.AudioRoutingManager";
}
unsafe impl Send for AudioRoutingManager {}
unsafe impl Sync for AudioRoutingManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioRoutingEndpoint(pub i32);
impl AudioRoutingEndpoint {
    pub const Default: Self = Self(0i32);
    pub const Earpiece: Self = Self(1i32);
    pub const Speakerphone: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
    pub const WiredHeadset: Self = Self(4i32);
    pub const WiredHeadsetSpeakerOnly: Self = Self(5i32);
    pub const BluetoothWithNoiseAndEchoCancellation: Self = Self(6i32);
    pub const BluetoothPreferred: Self = Self(7i32);
}
impl windows_core::TypeKind for AudioRoutingEndpoint {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioRoutingEndpoint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioRoutingEndpoint").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AudioRoutingEndpoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Media.Devices.AudioRoutingEndpoint;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AvailableAudioRoutingEndpoints(pub u32);
impl AvailableAudioRoutingEndpoints {
    pub const None: Self = Self(0u32);
    pub const Earpiece: Self = Self(1u32);
    pub const Speakerphone: Self = Self(2u32);
    pub const Bluetooth: Self = Self(4u32);
}
impl windows_core::TypeKind for AvailableAudioRoutingEndpoints {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AvailableAudioRoutingEndpoints {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AvailableAudioRoutingEndpoints").field(&self.0).finish()
    }
}
impl AvailableAudioRoutingEndpoints {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AvailableAudioRoutingEndpoints {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AvailableAudioRoutingEndpoints {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AvailableAudioRoutingEndpoints {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AvailableAudioRoutingEndpoints {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AvailableAudioRoutingEndpoints {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for AvailableAudioRoutingEndpoints {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.Media.Devices.AvailableAudioRoutingEndpoints;u4)");
}
