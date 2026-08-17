windows_core::imp::define_interface!(IConnectionRequestedEventArgs, IConnectionRequestedEventArgs_Vtbl, 0xeb6891ae_4f1e_4c66_bd0d_46924a942e08);
impl windows_core::RuntimeType for IConnectionRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConnectionRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PeerInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPeerFinderStatics, IPeerFinderStatics_Vtbl, 0x914b3b61_f6e1_47c4_a14c_148a1903d0c6);
impl windows_core::RuntimeType for IPeerFinderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPeerFinderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowBluetooth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowBluetooth: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AllowInfrastructure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowInfrastructure: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AllowWiFiDirect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowWiFiDirect: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SupportedDiscoveryTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PeerDiscoveryTypes) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AlternateIdentities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AlternateIdentities: usize,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartWithMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TriggeredConnectionStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTriggeredConnectionStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ConnectionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveConnectionRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllPeersAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllPeersAsync: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub ConnectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    ConnectAsync: usize,
}
windows_core::imp::define_interface!(IPeerFinderStatics2, IPeerFinderStatics2_Vtbl, 0xd6e73c65_fdd0_4b0b_9312_866408935d82);
impl windows_core::RuntimeType for IPeerFinderStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPeerFinderStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Role: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PeerRole) -> windows_core::HRESULT,
    pub SetRole: unsafe extern "system" fn(*mut core::ffi::c_void, PeerRole) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub DiscoveryData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DiscoveryData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetDiscoveryData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetDiscoveryData: usize,
    pub CreateWatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPeerInformation, IPeerInformation_Vtbl, 0x20024f08_9fff_45f4_b6e9_408b2ebef373);
impl windows_core::RuntimeType for IPeerInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPeerInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPeerInformation3, IPeerInformation3_Vtbl, 0xb20f612a_dbd0_40f8_95bd_2d4209c7836f);
impl windows_core::RuntimeType for IPeerInformation3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPeerInformation3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub DiscoveryData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DiscoveryData: usize,
}
windows_core::imp::define_interface!(IPeerInformationWithHostAndService, IPeerInformationWithHostAndService_Vtbl, 0xecc7ccad_1b70_4e8b_92db_bbe781419308);
impl windows_core::RuntimeType for IPeerInformationWithHostAndService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPeerInformationWithHostAndService_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HostName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPeerWatcher, IPeerWatcher_Vtbl, 0x3cee21f8_2fa6_4679_9691_03c94a420f34);
impl windows_core::RuntimeType for IPeerWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPeerWatcher_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Added: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAdded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Removed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Updated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub EnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Stopped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStopped: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PeerWatcherStatus) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProximityDevice, IProximityDevice_Vtbl, 0xefa8a552_f6e1_4329_a0fc_ab6b0fd28262);
impl windows_core::RuntimeType for IProximityDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProximityDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SubscribeForMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub PublishMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut i64) -> windows_core::HRESULT,
    pub PublishMessageWithCallback: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PublishBinaryMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PublishBinaryMessage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PublishBinaryMessageWithCallback: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PublishBinaryMessageWithCallback: usize,
    pub PublishUriMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub PublishUriMessageWithCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub StopSubscribingForMessage: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub StopPublishingMessage: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub DeviceArrived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDeviceArrived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DeviceDeparted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDeviceDeparted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub MaxMessageBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub BitsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProximityDeviceStatics, IProximityDeviceStatics_Vtbl, 0x914ba01d_f6e1_47c4_a14c_148a1903d0c6);
impl windows_core::RuntimeType for IProximityDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProximityDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProximityMessage, IProximityMessage_Vtbl, 0xefab0782_f6e1_4675_a045_d8e320c24808);
impl windows_core::RuntimeType for IProximityMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProximityMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MessageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SubscriptionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    pub DataAsString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITriggeredConnectionStateChangedEventArgs, ITriggeredConnectionStateChangedEventArgs_Vtbl, 0xc6a780ad_f6e1_4d54_96e2_33f620bca88a);
impl windows_core::RuntimeType for ITriggeredConnectionStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITriggeredConnectionStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TriggeredConnectState) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub Socket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    Socket: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ConnectionRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConnectionRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ConnectionRequestedEventArgs {
    pub fn PeerInformation(&self) -> windows_core::Result<PeerInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PeerInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ConnectionRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IConnectionRequestedEventArgs>();
}
unsafe impl windows_core::Interface for ConnectionRequestedEventArgs {
    type Vtable = IConnectionRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IConnectionRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.Networking.Proximity.ConnectionRequestedEventArgs";
}
unsafe impl Send for ConnectionRequestedEventArgs {}
unsafe impl Sync for ConnectionRequestedEventArgs {}
pub struct PeerFinder;
impl PeerFinder {
    pub fn AllowBluetooth() -> windows_core::Result<bool> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowBluetooth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetAllowBluetooth(value: bool) -> windows_core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetAllowBluetooth)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn AllowInfrastructure() -> windows_core::Result<bool> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowInfrastructure)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetAllowInfrastructure(value: bool) -> windows_core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetAllowInfrastructure)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn AllowWiFiDirect() -> windows_core::Result<bool> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowWiFiDirect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetAllowWiFiDirect(value: bool) -> windows_core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetAllowWiFiDirect)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn DisplayName() -> windows_core::Result<windows_core::HSTRING> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SetDisplayName(value: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetDisplayName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() })
    }
    pub fn SupportedDiscoveryTypes() -> windows_core::Result<PeerDiscoveryTypes> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedDiscoveryTypes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AlternateIdentities() -> windows_core::Result<super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AlternateIdentities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Start() -> windows_core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn StartWithMessage(peermessage: &windows_core::HSTRING) -> windows_core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (windows_core::Interface::vtable(this).StartWithMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(peermessage)).ok() })
    }
    pub fn Stop() -> windows_core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn TriggeredConnectionStateChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, TriggeredConnectionStateChangedEventArgs>>,
    {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriggeredConnectionStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveTriggeredConnectionStateChanged(cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveTriggeredConnectionStateChanged)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    pub fn ConnectionRequested<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, ConnectionRequestedEventArgs>>,
    {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveConnectionRequested(cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveConnectionRequested)(windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllPeersAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PeerInformation>>> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllPeersAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn ConnectAsync<P0>(peerinformation: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::Sockets::StreamSocket>>
    where
        P0: windows_core::Param<PeerInformation>,
    {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectAsync)(windows_core::Interface::as_raw(this), peerinformation.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Role() -> windows_core::Result<PeerRole> {
        Self::IPeerFinderStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Role)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetRole(value: PeerRole) -> windows_core::Result<()> {
        Self::IPeerFinderStatics2(|this| unsafe { (windows_core::Interface::vtable(this).SetRole)(windows_core::Interface::as_raw(this), value).ok() })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn DiscoveryData() -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        Self::IPeerFinderStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DiscoveryData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetDiscoveryData<P0>(value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::IPeerFinderStatics2(|this| unsafe { (windows_core::Interface::vtable(this).SetDiscoveryData)(windows_core::Interface::as_raw(this), value.param().abi()).ok() })
    }
    pub fn CreateWatcher() -> windows_core::Result<PeerWatcher> {
        Self::IPeerFinderStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPeerFinderStatics<R, F: FnOnce(&IPeerFinderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PeerFinder, IPeerFinderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPeerFinderStatics2<R, F: FnOnce(&IPeerFinderStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PeerFinder, IPeerFinderStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PeerFinder {
    const NAME: &'static str = "Windows.Networking.Proximity.PeerFinder";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PeerInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PeerInformation, windows_core::IUnknown, windows_core::IInspectable);
impl PeerInformation {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPeerInformation3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn DiscoveryData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &windows_core::Interface::cast::<IPeerInformation3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DiscoveryData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HostName(&self) -> windows_core::Result<super::HostName> {
        let this = &windows_core::Interface::cast::<IPeerInformationWithHostAndService>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HostName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ServiceName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPeerInformationWithHostAndService>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PeerInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPeerInformation>();
}
unsafe impl windows_core::Interface for PeerInformation {
    type Vtable = IPeerInformation_Vtbl;
    const IID: windows_core::GUID = <IPeerInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PeerInformation {
    const NAME: &'static str = "Windows.Networking.Proximity.PeerInformation";
}
unsafe impl Send for PeerInformation {}
unsafe impl Sync for PeerInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PeerWatcher(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PeerWatcher, windows_core::IUnknown, windows_core::IInspectable);
impl PeerWatcher {
    pub fn Added<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Added)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAdded)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Removed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Removed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRemoved)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Updated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Updated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveUpdated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PeerWatcher, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnumerationCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEnumerationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Stopped<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PeerWatcher, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stopped)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStopped(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStopped)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> windows_core::Result<PeerWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
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
impl windows_core::RuntimeType for PeerWatcher {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPeerWatcher>();
}
unsafe impl windows_core::Interface for PeerWatcher {
    type Vtable = IPeerWatcher_Vtbl;
    const IID: windows_core::GUID = <IPeerWatcher as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PeerWatcher {
    const NAME: &'static str = "Windows.Networking.Proximity.PeerWatcher";
}
unsafe impl Send for PeerWatcher {}
unsafe impl Sync for PeerWatcher {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ProximityDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProximityDevice, windows_core::IUnknown, windows_core::IInspectable);
impl ProximityDevice {
    pub fn SubscribeForMessage<P0>(&self, messagetype: &windows_core::HSTRING, messagereceivedhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<MessageReceivedHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubscribeForMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(messagetype), messagereceivedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn PublishMessage(&self, messagetype: &windows_core::HSTRING, message: &windows_core::HSTRING) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PublishMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(messagetype), core::mem::transmute_copy(message), &mut result__).map(|| result__)
        }
    }
    pub fn PublishMessageWithCallback<P0>(&self, messagetype: &windows_core::HSTRING, message: &windows_core::HSTRING, messagetransmittedhandler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<MessageTransmittedHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PublishMessageWithCallback)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(messagetype), core::mem::transmute_copy(message), messagetransmittedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PublishBinaryMessage<P0>(&self, messagetype: &windows_core::HSTRING, message: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PublishBinaryMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(messagetype), message.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PublishBinaryMessageWithCallback<P0, P1>(&self, messagetype: &windows_core::HSTRING, message: P0, messagetransmittedhandler: P1) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<MessageTransmittedHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PublishBinaryMessageWithCallback)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(messagetype), message.param().abi(), messagetransmittedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn PublishUriMessage<P0>(&self, message: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PublishUriMessage)(windows_core::Interface::as_raw(this), message.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn PublishUriMessageWithCallback<P0, P1>(&self, message: P0, messagetransmittedhandler: P1) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<MessageTransmittedHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PublishUriMessageWithCallback)(windows_core::Interface::as_raw(this), message.param().abi(), messagetransmittedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn StopSubscribingForMessage(&self, subscriptionid: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopSubscribingForMessage)(windows_core::Interface::as_raw(this), subscriptionid).ok() }
    }
    pub fn StopPublishingMessage(&self, messageid: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopPublishingMessage)(windows_core::Interface::as_raw(this), messageid).ok() }
    }
    pub fn DeviceArrived<P0>(&self, arrivedhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<DeviceArrivedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceArrived)(windows_core::Interface::as_raw(this), arrivedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDeviceArrived(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDeviceArrived)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn DeviceDeparted<P0>(&self, departedhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<DeviceDepartedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceDeparted)(windows_core::Interface::as_raw(this), departedhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDeviceDeparted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDeviceDeparted)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn MaxMessageBytes(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxMessageBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BitsPerSecond(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitsPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IProximityDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefault() -> windows_core::Result<ProximityDevice> {
        Self::IProximityDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromId(deviceid: &windows_core::HSTRING) -> windows_core::Result<ProximityDevice> {
        Self::IProximityDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IProximityDeviceStatics<R, F: FnOnce(&IProximityDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProximityDevice, IProximityDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ProximityDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProximityDevice>();
}
unsafe impl windows_core::Interface for ProximityDevice {
    type Vtable = IProximityDevice_Vtbl;
    const IID: windows_core::GUID = <IProximityDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProximityDevice {
    const NAME: &'static str = "Windows.Networking.Proximity.ProximityDevice";
}
unsafe impl Send for ProximityDevice {}
unsafe impl Sync for ProximityDevice {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ProximityMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProximityMessage, windows_core::IUnknown, windows_core::IInspectable);
impl ProximityMessage {
    pub fn MessageType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SubscriptionId(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubscriptionId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DataAsString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataAsString)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ProximityMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProximityMessage>();
}
unsafe impl windows_core::Interface for ProximityMessage {
    type Vtable = IProximityMessage_Vtbl;
    const IID: windows_core::GUID = <IProximityMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProximityMessage {
    const NAME: &'static str = "Windows.Networking.Proximity.ProximityMessage";
}
unsafe impl Send for ProximityMessage {}
unsafe impl Sync for ProximityMessage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TriggeredConnectionStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TriggeredConnectionStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl TriggeredConnectionStateChangedEventArgs {
    pub fn State(&self) -> windows_core::Result<TriggeredConnectState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn Socket(&self) -> windows_core::Result<super::Sockets::StreamSocket> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Socket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TriggeredConnectionStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITriggeredConnectionStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for TriggeredConnectionStateChangedEventArgs {
    type Vtable = ITriggeredConnectionStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ITriggeredConnectionStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TriggeredConnectionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs";
}
unsafe impl Send for TriggeredConnectionStateChangedEventArgs {}
unsafe impl Sync for TriggeredConnectionStateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PeerDiscoveryTypes(pub u32);
impl PeerDiscoveryTypes {
    pub const None: Self = Self(0u32);
    pub const Browse: Self = Self(1u32);
    pub const Triggered: Self = Self(2u32);
}
impl windows_core::TypeKind for PeerDiscoveryTypes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PeerDiscoveryTypes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PeerDiscoveryTypes").field(&self.0).finish()
    }
}
impl PeerDiscoveryTypes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PeerDiscoveryTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PeerDiscoveryTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PeerDiscoveryTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PeerDiscoveryTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PeerDiscoveryTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for PeerDiscoveryTypes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.PeerDiscoveryTypes;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PeerRole(pub i32);
impl PeerRole {
    pub const Peer: Self = Self(0i32);
    pub const Host: Self = Self(1i32);
    pub const Client: Self = Self(2i32);
}
impl windows_core::TypeKind for PeerRole {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PeerRole {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PeerRole").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PeerRole {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.PeerRole;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PeerWatcherStatus(pub i32);
impl PeerWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl windows_core::TypeKind for PeerWatcherStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PeerWatcherStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PeerWatcherStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PeerWatcherStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.PeerWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TriggeredConnectState(pub i32);
impl TriggeredConnectState {
    pub const PeerFound: Self = Self(0i32);
    pub const Listening: Self = Self(1i32);
    pub const Connecting: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
    pub const Canceled: Self = Self(4i32);
    pub const Failed: Self = Self(5i32);
}
impl windows_core::TypeKind for TriggeredConnectState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TriggeredConnectState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TriggeredConnectState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TriggeredConnectState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.TriggeredConnectState;i4)");
}
windows_core::imp::define_interface!(DeviceArrivedEventHandler, DeviceArrivedEventHandler_Vtbl, 0xefa9da69_f6e1_49c9_a49e_8e0fc58fb911);
impl DeviceArrivedEventHandler {
    pub fn new<F: FnMut(Option<&ProximityDevice>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = DeviceArrivedEventHandlerBox::<F> { vtable: &DeviceArrivedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ProximityDevice>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi()).ok() }
    }
}
#[repr(C)]
struct DeviceArrivedEventHandlerBox<F: FnMut(Option<&ProximityDevice>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const DeviceArrivedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&ProximityDevice>) -> windows_core::Result<()> + Send + 'static> DeviceArrivedEventHandlerBox<F> {
    const VTABLE: DeviceArrivedEventHandler_Vtbl = DeviceArrivedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <DeviceArrivedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender)).into()
    }
}
impl windows_core::RuntimeType for DeviceArrivedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct DeviceArrivedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(DeviceDepartedEventHandler, DeviceDepartedEventHandler_Vtbl, 0xefa9da69_f6e2_49c9_a49e_8e0fc58fb911);
impl DeviceDepartedEventHandler {
    pub fn new<F: FnMut(Option<&ProximityDevice>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = DeviceDepartedEventHandlerBox::<F> { vtable: &DeviceDepartedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ProximityDevice>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi()).ok() }
    }
}
#[repr(C)]
struct DeviceDepartedEventHandlerBox<F: FnMut(Option<&ProximityDevice>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const DeviceDepartedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&ProximityDevice>) -> windows_core::Result<()> + Send + 'static> DeviceDepartedEventHandlerBox<F> {
    const VTABLE: DeviceDepartedEventHandler_Vtbl = DeviceDepartedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <DeviceDepartedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender)).into()
    }
}
impl windows_core::RuntimeType for DeviceDepartedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct DeviceDepartedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(MessageReceivedHandler, MessageReceivedHandler_Vtbl, 0xefab0782_f6e2_4675_a045_d8e320c24808);
impl MessageReceivedHandler {
    pub fn new<F: FnMut(Option<&ProximityDevice>, Option<&ProximityMessage>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = MessageReceivedHandlerBox::<F> { vtable: &MessageReceivedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, message: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ProximityDevice>,
        P1: windows_core::Param<ProximityMessage>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), message.param().abi()).ok() }
    }
}
#[repr(C)]
struct MessageReceivedHandlerBox<F: FnMut(Option<&ProximityDevice>, Option<&ProximityMessage>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const MessageReceivedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&ProximityDevice>, Option<&ProximityMessage>) -> windows_core::Result<()> + Send + 'static> MessageReceivedHandlerBox<F> {
    const VTABLE: MessageReceivedHandler_Vtbl = MessageReceivedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <MessageReceivedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, message: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&message)).into()
    }
}
impl windows_core::RuntimeType for MessageReceivedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct MessageReceivedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(MessageTransmittedHandler, MessageTransmittedHandler_Vtbl, 0xefaa0b4a_f6e2_4d7d_856c_78fc8efc021e);
impl MessageTransmittedHandler {
    pub fn new<F: FnMut(Option<&ProximityDevice>, i64) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = MessageTransmittedHandlerBox::<F> { vtable: &MessageTransmittedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0, messageid: i64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ProximityDevice>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), messageid).ok() }
    }
}
#[repr(C)]
struct MessageTransmittedHandlerBox<F: FnMut(Option<&ProximityDevice>, i64) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const MessageTransmittedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&ProximityDevice>, i64) -> windows_core::Result<()> + Send + 'static> MessageTransmittedHandlerBox<F> {
    const VTABLE: MessageTransmittedHandler_Vtbl = MessageTransmittedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <MessageTransmittedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, messageid: i64) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), messageid).into()
    }
}
impl windows_core::RuntimeType for MessageTransmittedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct MessageTransmittedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
