windows_core::imp::define_interface!(IControlChannelTrigger, IControlChannelTrigger_Vtbl, 0x7d1431a7_ee96_40e8_a199_8703cd969ec3);
impl windows_core::RuntimeType for IControlChannelTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IControlChannelTrigger_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ControlChannelTriggerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ServerKeepAliveIntervalInMinutes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetServerKeepAliveIntervalInMinutes: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CurrentKeepAliveIntervalInMinutes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TransportObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Background")]
    pub KeepAliveTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    KeepAliveTrigger: usize,
    #[cfg(feature = "ApplicationModel_Background")]
    pub PushNotificationTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    PushNotificationTrigger: usize,
    pub UsingTransport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForPushEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ControlChannelTriggerStatus) -> windows_core::HRESULT,
    pub DecreaseNetworkKeepAliveInterval: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FlushTransport: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IControlChannelTrigger2, IControlChannelTrigger2_Vtbl, 0xaf00d237_51be_4514_9725_3556e1879580);
impl windows_core::RuntimeType for IControlChannelTrigger2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IControlChannelTrigger2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsWakeFromLowPowerSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IControlChannelTriggerEventDetails, IControlChannelTriggerEventDetails_Vtbl, 0x1b36e047_89bb_4236_96ac_71d012bb4869);
impl core::ops::Deref for IControlChannelTriggerEventDetails {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IControlChannelTriggerEventDetails, windows_core::IUnknown, windows_core::IInspectable);
impl IControlChannelTriggerEventDetails {
    pub fn ControlChannelTrigger(&self) -> windows_core::Result<ControlChannelTrigger> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControlChannelTrigger)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IControlChannelTriggerEventDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IControlChannelTriggerEventDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ControlChannelTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IControlChannelTriggerFactory, IControlChannelTriggerFactory_Vtbl, 0xda4b7cf0_8d71_446f_88c3_b95184a2d6cd);
impl windows_core::RuntimeType for IControlChannelTriggerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IControlChannelTriggerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateControlChannelTrigger: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateControlChannelTriggerEx: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32, ControlChannelTriggerResourceType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IControlChannelTriggerResetEventDetails, IControlChannelTriggerResetEventDetails_Vtbl, 0x6851038e_8ec4_42fe_9bb2_21e91b7bfcb1);
impl core::ops::Deref for IControlChannelTriggerResetEventDetails {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IControlChannelTriggerResetEventDetails, windows_core::IUnknown, windows_core::IInspectable);
impl IControlChannelTriggerResetEventDetails {
    pub fn ResetReason(&self) -> windows_core::Result<ControlChannelTriggerResetReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResetReason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HardwareSlotReset(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HardwareSlotReset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SoftwareSlotReset(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SoftwareSlotReset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IControlChannelTriggerResetEventDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IControlChannelTriggerResetEventDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResetReason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ControlChannelTriggerResetReason) -> windows_core::HRESULT,
    pub HardwareSlotReset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SoftwareSlotReset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDatagramSocket, IDatagramSocket_Vtbl, 0x7fe25bbb_c3bc_4677_8446_ca28a465a3af);
impl windows_core::RuntimeType for IDatagramSocket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDatagramSocket_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Control: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Information: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    pub ConnectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectWithEndpointPairAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BindServiceNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BindEndpointAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub JoinMulticastGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetOutputStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOutputStreamAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetOutputStreamWithEndpointPairAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOutputStreamWithEndpointPairAsync: usize,
    pub MessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDatagramSocket2, IDatagramSocket2_Vtbl, 0xd83ba354_9a9d_4185_a20a_1424c9c2a7cd);
impl windows_core::RuntimeType for IDatagramSocket2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDatagramSocket2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub BindServiceNameAndAdapterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    BindServiceNameAndAdapterAsync: usize,
}
windows_core::imp::define_interface!(IDatagramSocket3, IDatagramSocket3_Vtbl, 0x37544f09_ab92_4306_9ac1_0c381283d9c6);
impl windows_core::RuntimeType for IDatagramSocket3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDatagramSocket3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CancelIOAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableTransferOwnership: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub EnableTransferOwnershipWithConnectedStandbyAction: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, SocketActivityConnectedStandbyAction) -> windows_core::HRESULT,
    pub TransferOwnership: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TransferOwnershipWithContext: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransferOwnershipWithContextAndKeepAliveTime: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDatagramSocketControl, IDatagramSocketControl_Vtbl, 0x52ac3f2e_349a_4135_bb58_b79b2647d390);
impl windows_core::RuntimeType for IDatagramSocketControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDatagramSocketControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub QualityOfService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SocketQualityOfService) -> windows_core::HRESULT,
    pub SetQualityOfService: unsafe extern "system" fn(*mut core::ffi::c_void, SocketQualityOfService) -> windows_core::HRESULT,
    pub OutboundUnicastHopLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetOutboundUnicastHopLimit: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDatagramSocketControl2, IDatagramSocketControl2_Vtbl, 0x33ead5c2_979c_4415_82a1_3cfaf646c192);
impl windows_core::RuntimeType for IDatagramSocketControl2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDatagramSocketControl2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InboundBufferSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetInboundBufferSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DontFragment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDontFragment: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDatagramSocketControl3, IDatagramSocketControl3_Vtbl, 0xd4eb8256_1f6d_4598_9b57_d42a001df349);
impl windows_core::RuntimeType for IDatagramSocketControl3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDatagramSocketControl3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MulticastOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetMulticastOnly: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDatagramSocketInformation, IDatagramSocketInformation_Vtbl, 0x5f1a569a_55fb_48cd_9706_7a974f7b1585);
impl windows_core::RuntimeType for IDatagramSocketInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDatagramSocketInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LocalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RemoteAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemotePort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDatagramSocketMessageReceivedEventArgs, IDatagramSocketMessageReceivedEventArgs_Vtbl, 0x9e2ddca2_1712_4ce4_b179_8c652c6d107e);
impl windows_core::RuntimeType for IDatagramSocketMessageReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDatagramSocketMessageReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RemoteAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemotePort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LocalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetDataReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetDataReader: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetDataStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetDataStream: usize,
}
windows_core::imp::define_interface!(IDatagramSocketStatics, IDatagramSocketStatics_Vtbl, 0xe9c62aee_1494_4a21_bb7e_8589fc751d9d);
impl windows_core::RuntimeType for IDatagramSocketStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDatagramSocketStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEndpointPairsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEndpointPairsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEndpointPairsWithSortOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::HostNameSortOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEndpointPairsWithSortOptionsAsync: usize,
}
windows_core::imp::define_interface!(IMessageWebSocket, IMessageWebSocket_Vtbl, 0x33727d08_34d5_4746_ad7b_8dde5bc2ef88);
impl windows_core::RuntimeType for IMessageWebSocket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMessageWebSocket_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Control: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Information: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMessageWebSocket2, IMessageWebSocket2_Vtbl, 0xbed0cee7_f9c8_440a_9ad5_737281d9742e);
impl windows_core::RuntimeType for IMessageWebSocket2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMessageWebSocket2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServerCustomValidationRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveServerCustomValidationRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMessageWebSocket3, IMessageWebSocket3_Vtbl, 0x59d9defb_71af_4349_8487_911fcf681597);
impl windows_core::RuntimeType for IMessageWebSocket3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMessageWebSocket3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SendNonfinalFrameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SendNonfinalFrameAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SendFinalFrameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SendFinalFrameAsync: usize,
}
windows_core::imp::define_interface!(IMessageWebSocketControl, IMessageWebSocketControl_Vtbl, 0x8118388a_c629_4f0a_80fb_81fc05538862);
impl windows_core::RuntimeType for IMessageWebSocketControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMessageWebSocketControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxMessageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxMessageSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MessageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SocketMessageType) -> windows_core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(*mut core::ffi::c_void, SocketMessageType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMessageWebSocketControl2, IMessageWebSocketControl2_Vtbl, 0xe30fd791_080c_400a_a712_27dfa9e744d8);
impl windows_core::RuntimeType for IMessageWebSocketControl2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMessageWebSocketControl2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DesiredUnsolicitedPongInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetDesiredUnsolicitedPongInterval: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub ActualUnsolicitedPongInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub ReceiveMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MessageWebSocketReceiveMode) -> windows_core::HRESULT,
    pub SetReceiveMode: unsafe extern "system" fn(*mut core::ffi::c_void, MessageWebSocketReceiveMode) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ClientCertificate: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub SetClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    SetClientCertificate: usize,
}
windows_core::imp::define_interface!(IMessageWebSocketMessageReceivedEventArgs, IMessageWebSocketMessageReceivedEventArgs_Vtbl, 0x478c22ac_4c4b_42ed_9ed7_1ef9f94fa3d5);
impl windows_core::RuntimeType for IMessageWebSocketMessageReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMessageWebSocketMessageReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MessageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SocketMessageType) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetDataReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetDataReader: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetDataStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetDataStream: usize,
}
windows_core::imp::define_interface!(IMessageWebSocketMessageReceivedEventArgs2, IMessageWebSocketMessageReceivedEventArgs2_Vtbl, 0x89ce06fd_dd6f_4a07_87f9_f9eb4d89d83d);
impl windows_core::RuntimeType for IMessageWebSocketMessageReceivedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMessageWebSocketMessageReceivedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsMessageComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServerMessageWebSocket, IServerMessageWebSocket_Vtbl, 0xe3ac9240_813b_5efd_7e11_ae2305fc77f1);
impl windows_core::RuntimeType for IServerMessageWebSocket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IServerMessageWebSocket_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Control: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Information: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CloseWithStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u16, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServerMessageWebSocketControl, IServerMessageWebSocketControl_Vtbl, 0x69c2f051_1c1f_587a_4519_2181610192b7);
impl windows_core::RuntimeType for IServerMessageWebSocketControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IServerMessageWebSocketControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MessageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SocketMessageType) -> windows_core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(*mut core::ffi::c_void, SocketMessageType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServerMessageWebSocketInformation, IServerMessageWebSocketInformation_Vtbl, 0xfc32b45f_4448_5505_6cc9_09afa8915f5d);
impl windows_core::RuntimeType for IServerMessageWebSocketInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IServerMessageWebSocketInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BandwidthStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BandwidthStatistics) -> windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LocalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServerStreamWebSocket, IServerStreamWebSocket_Vtbl, 0x2ced5bbf_74f6_55e4_79df_9132680dfee8);
impl windows_core::RuntimeType for IServerStreamWebSocket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IServerStreamWebSocket_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Information: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CloseWithStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u16, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServerStreamWebSocketInformation, IServerStreamWebSocketInformation_Vtbl, 0xfc32b45f_4448_5505_6cc9_09aba8915f5d);
impl windows_core::RuntimeType for IServerStreamWebSocketInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IServerStreamWebSocketInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BandwidthStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BandwidthStatistics) -> windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LocalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISocketActivityContext, ISocketActivityContext_Vtbl, 0x43b04d64_4c85_4396_a637_1d973f6ebd49);
impl windows_core::RuntimeType for ISocketActivityContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISocketActivityContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
windows_core::imp::define_interface!(ISocketActivityContextFactory, ISocketActivityContextFactory_Vtbl, 0xb99fc3c3_088c_4388_83ae_2525138e049a);
impl windows_core::RuntimeType for ISocketActivityContextFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISocketActivityContextFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
windows_core::imp::define_interface!(ISocketActivityInformation, ISocketActivityInformation_Vtbl, 0x8d8a42e4_a87e_4b74_9968_185b2511defe);
impl windows_core::RuntimeType for ISocketActivityInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISocketActivityInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TaskId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SocketKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SocketActivityKind) -> windows_core::HRESULT,
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DatagramSocket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StreamSocket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StreamSocketListener: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISocketActivityInformationStatics, ISocketActivityInformationStatics_Vtbl, 0x8570b47a_7e7d_4736_8041_1327a6543c56);
impl windows_core::RuntimeType for ISocketActivityInformationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISocketActivityInformationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AllSockets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllSockets: usize,
}
windows_core::imp::define_interface!(ISocketActivityTriggerDetails, ISocketActivityTriggerDetails_Vtbl, 0x45f406a7_fc9f_4f81_acad_355fef51e67b);
impl windows_core::RuntimeType for ISocketActivityTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISocketActivityTriggerDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SocketActivityTriggerReason) -> windows_core::HRESULT,
    pub SocketInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISocketErrorStatics, ISocketErrorStatics_Vtbl, 0x828337f4_7d56_4d8e_b7b4_a07dd7c1bca9);
impl windows_core::RuntimeType for ISocketErrorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISocketErrorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut SocketErrorStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamSocket, IStreamSocket_Vtbl, 0x69a22cf3_fc7b_4857_af38_f6e7de6a5b49);
impl windows_core::RuntimeType for IStreamSocket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocket_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Control: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Information: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    pub ConnectWithEndpointPairAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectWithEndpointPairAndProtectionLevelAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SocketProtectionLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectWithProtectionLevelAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, SocketProtectionLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpgradeToSslAsync: unsafe extern "system" fn(*mut core::ffi::c_void, SocketProtectionLevel, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamSocket2, IStreamSocket2_Vtbl, 0x29d0e575_f314_4d09_adf0_0fbd967fbd9f);
impl windows_core::RuntimeType for IStreamSocket2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocket2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub ConnectWithProtectionLevelAndAdapterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, SocketProtectionLevel, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    ConnectWithProtectionLevelAndAdapterAsync: usize,
}
windows_core::imp::define_interface!(IStreamSocket3, IStreamSocket3_Vtbl, 0x3f430b00_9d28_4854_bac3_2301941ec223);
impl windows_core::RuntimeType for IStreamSocket3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocket3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CancelIOAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableTransferOwnership: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub EnableTransferOwnershipWithConnectedStandbyAction: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, SocketActivityConnectedStandbyAction) -> windows_core::HRESULT,
    pub TransferOwnership: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TransferOwnershipWithContext: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransferOwnershipWithContextAndKeepAliveTime: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamSocketControl, IStreamSocketControl_Vtbl, 0xfe25adf1_92ab_4af3_9992_0f4c85e36cc4);
impl windows_core::RuntimeType for IStreamSocketControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocketControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NoDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetNoDelay: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub KeepAlive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetKeepAlive: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub OutboundBufferSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetOutboundBufferSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub QualityOfService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SocketQualityOfService) -> windows_core::HRESULT,
    pub SetQualityOfService: unsafe extern "system" fn(*mut core::ffi::c_void, SocketQualityOfService) -> windows_core::HRESULT,
    pub OutboundUnicastHopLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetOutboundUnicastHopLimit: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamSocketControl2, IStreamSocketControl2_Vtbl, 0xc2d09a56_060f_44c1_b8e2_1fbf60bd62c5);
impl windows_core::RuntimeType for IStreamSocketControl2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocketControl2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub IgnorableServerCertificateErrors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    IgnorableServerCertificateErrors: usize,
}
windows_core::imp::define_interface!(IStreamSocketControl3, IStreamSocketControl3_Vtbl, 0xc56a444c_4e74_403e_894c_b31cae5c7342);
impl windows_core::RuntimeType for IStreamSocketControl3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocketControl3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SerializeConnectionAttempts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSerializeConnectionAttempts: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ClientCertificate: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub SetClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    SetClientCertificate: usize,
}
windows_core::imp::define_interface!(IStreamSocketControl4, IStreamSocketControl4_Vtbl, 0x964e2b3d_ec27_4888_b3ce_c74b418423ad);
impl windows_core::RuntimeType for IStreamSocketControl4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocketControl4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MinProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SocketProtectionLevel) -> windows_core::HRESULT,
    pub SetMinProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, SocketProtectionLevel) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamSocketInformation, IStreamSocketInformation_Vtbl, 0x3b80ae30_5e68_4205_88f0_dc85d2e25ded);
impl windows_core::RuntimeType for IStreamSocketInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocketInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LocalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RemoteHostName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RemotePort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RoundTripTimeStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RoundTripTimeStatistics) -> windows_core::HRESULT,
    pub BandwidthStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BandwidthStatistics) -> windows_core::HRESULT,
    pub ProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SocketProtectionLevel) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SessionKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SessionKey: usize,
}
windows_core::imp::define_interface!(IStreamSocketInformation2, IStreamSocketInformation2_Vtbl, 0x12c28452_4bdc_4ee4_976a_cf130e9d92e3);
impl windows_core::RuntimeType for IStreamSocketInformation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocketInformation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SocketSslErrorSeverity) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerCertificateErrors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerCertificateErrors: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerIntermediateCertificates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerIntermediateCertificates: usize,
}
windows_core::imp::define_interface!(IStreamSocketListener, IStreamSocketListener_Vtbl, 0xff513437_df9f_4df0_bf82_0ec5d7b35aae);
impl windows_core::RuntimeType for IStreamSocketListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocketListener_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Control: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Information: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BindServiceNameAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BindEndpointAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConnectionReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveConnectionReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamSocketListener2, IStreamSocketListener2_Vtbl, 0x658dc13e_bb3e_4458_b232_ed1088694b98);
impl windows_core::RuntimeType for IStreamSocketListener2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocketListener2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BindServiceNameWithProtectionLevelAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, SocketProtectionLevel, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Networking_Connectivity")]
    pub BindServiceNameWithProtectionLevelAndAdapterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, SocketProtectionLevel, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    BindServiceNameWithProtectionLevelAndAdapterAsync: usize,
}
windows_core::imp::define_interface!(IStreamSocketListener3, IStreamSocketListener3_Vtbl, 0x4798201c_bdf8_4919_8542_28d450e74507);
impl windows_core::RuntimeType for IStreamSocketListener3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocketListener3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CancelIOAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableTransferOwnership: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub EnableTransferOwnershipWithConnectedStandbyAction: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, SocketActivityConnectedStandbyAction) -> windows_core::HRESULT,
    pub TransferOwnership: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TransferOwnershipWithContext: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamSocketListenerConnectionReceivedEventArgs, IStreamSocketListenerConnectionReceivedEventArgs_Vtbl, 0x0c472ea9_373f_447b_85b1_ddd4548803ba);
impl windows_core::RuntimeType for IStreamSocketListenerConnectionReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocketListenerConnectionReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Socket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamSocketListenerControl, IStreamSocketListenerControl_Vtbl, 0x20d8c576_8d8a_4dba_9722_a16c4d984980);
impl windows_core::RuntimeType for IStreamSocketListenerControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocketListenerControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub QualityOfService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SocketQualityOfService) -> windows_core::HRESULT,
    pub SetQualityOfService: unsafe extern "system" fn(*mut core::ffi::c_void, SocketQualityOfService) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamSocketListenerControl2, IStreamSocketListenerControl2_Vtbl, 0x948bb665_2c3e_404b_b8b0_8eb249a2b0a1);
impl windows_core::RuntimeType for IStreamSocketListenerControl2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocketListenerControl2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NoDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetNoDelay: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub KeepAlive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetKeepAlive: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub OutboundBufferSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetOutboundBufferSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OutboundUnicastHopLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub SetOutboundUnicastHopLimit: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamSocketListenerInformation, IStreamSocketListenerInformation_Vtbl, 0xe62ba82f_a63a_430b_bf62_29e93e5633b4);
impl windows_core::RuntimeType for IStreamSocketListenerInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocketListenerInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LocalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamSocketStatics, IStreamSocketStatics_Vtbl, 0xa420bc4a_6e2e_4af5_b556_355ae0cd4f29);
impl windows_core::RuntimeType for IStreamSocketStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamSocketStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEndpointPairsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEndpointPairsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEndpointPairsWithSortOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, super::HostNameSortOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEndpointPairsWithSortOptionsAsync: usize,
}
windows_core::imp::define_interface!(IStreamWebSocket, IStreamWebSocket_Vtbl, 0xbd4a49d8_b289_45bb_97eb_c7525205a843);
impl windows_core::RuntimeType for IStreamWebSocket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamWebSocket_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Control: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Information: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
}
windows_core::imp::define_interface!(IStreamWebSocket2, IStreamWebSocket2_Vtbl, 0xaa4d08cb_93f5_4678_8236_57cce5417ed5);
impl windows_core::RuntimeType for IStreamWebSocket2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamWebSocket2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServerCustomValidationRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveServerCustomValidationRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamWebSocketControl, IStreamWebSocketControl_Vtbl, 0xb4f478b1_a45a_48db_953a_645b7d964c07);
impl windows_core::RuntimeType for IStreamWebSocketControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamWebSocketControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NoDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetNoDelay: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamWebSocketControl2, IStreamWebSocketControl2_Vtbl, 0x215d9f7e_fa58_40da_9f11_a48dafe95037);
impl windows_core::RuntimeType for IStreamWebSocketControl2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStreamWebSocketControl2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DesiredUnsolicitedPongInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SetDesiredUnsolicitedPongInterval: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub ActualUnsolicitedPongInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ClientCertificate: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub SetClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    SetClientCertificate: usize,
}
windows_core::imp::define_interface!(IWebSocket, IWebSocket_Vtbl, 0xf877396f_99b1_4e18_bc08_850c9adf156e);
impl core::ops::Deref for IWebSocket {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebSocket, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IWebSocket, super::super::Foundation::IClosable);
impl IWebSocket {
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectAsync<P0>(&self, uri: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRequestHeader(&self, headername: &windows_core::HSTRING, headervalue: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRequestHeader)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(headername), core::mem::transmute_copy(headervalue)).ok() }
    }
    pub fn Closed<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn CloseWithStatus(&self, code: u16, reason: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CloseWithStatus)(windows_core::Interface::as_raw(this), code, core::mem::transmute_copy(reason)).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for IWebSocket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebSocket_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    pub ConnectAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRequestHeader: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CloseWithStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u16, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebSocketClosedEventArgs, IWebSocketClosedEventArgs_Vtbl, 0xceb78d07_d0a8_4703_a091_c8c2c0915bc3);
impl windows_core::RuntimeType for IWebSocketClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebSocketClosedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Code: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebSocketControl, IWebSocketControl_Vtbl, 0x2ec4bdc3_d9a5_455a_9811_de24d45337e9);
impl core::ops::Deref for IWebSocketControl {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebSocketControl, windows_core::IUnknown, windows_core::IInspectable);
impl IWebSocketControl {
    pub fn OutboundBufferSizeInBytes(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutboundBufferSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOutboundBufferSizeInBytes)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetServerCredential)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProxyCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProxyCredential)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedProtocols(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedProtocols)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IWebSocketControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebSocketControl_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OutboundBufferSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetOutboundBufferSizeInBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub ServerCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetServerCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub ProxyCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    ProxyCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetProxyCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetProxyCredential: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedProtocols: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedProtocols: usize,
}
windows_core::imp::define_interface!(IWebSocketControl2, IWebSocketControl2_Vtbl, 0x79c3be03_f2ca_461e_af4e_9665bc2d0620);
impl core::ops::Deref for IWebSocketControl2 {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebSocketControl2, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IWebSocketControl2, IWebSocketControl);
impl IWebSocketControl2 {
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IgnorableServerCertificateErrors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OutboundBufferSizeInBytes(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutboundBufferSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutboundBufferSizeInBytes)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetServerCredential)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProxyCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProxyCredential)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedProtocols(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedProtocols)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IWebSocketControl2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebSocketControl2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub IgnorableServerCertificateErrors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    IgnorableServerCertificateErrors: usize,
}
windows_core::imp::define_interface!(IWebSocketErrorStatics, IWebSocketErrorStatics_Vtbl, 0x27cdf35b_1f61_4709_8e02_61283ada4e9d);
impl windows_core::RuntimeType for IWebSocketErrorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebSocketErrorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Web")]
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Web::WebErrorStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "Web"))]
    GetStatus: usize,
}
windows_core::imp::define_interface!(IWebSocketInformation, IWebSocketInformation_Vtbl, 0x5e01e316_c92a_47a5_b25f_07847639d181);
impl core::ops::Deref for IWebSocketInformation {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebSocketInformation, windows_core::IUnknown, windows_core::IInspectable);
impl IWebSocketInformation {
    pub fn LocalAddress(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BandwidthStatistics(&self) -> windows_core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BandwidthStatistics)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Protocol(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Protocol)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IWebSocketInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebSocketInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LocalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BandwidthStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BandwidthStatistics) -> windows_core::HRESULT,
    pub Protocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebSocketInformation2, IWebSocketInformation2_Vtbl, 0xce1d39ce_a1b7_4d43_8269_8d5b981bd47a);
impl core::ops::Deref for IWebSocketInformation2 {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebSocketInformation2, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IWebSocketInformation2, IWebSocketInformation);
impl IWebSocketInformation2 {
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ServerCertificateErrorSeverity(&self) -> windows_core::Result<SocketSslErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificateErrorSeverity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificateErrors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerIntermediateCertificates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalAddress(&self) -> windows_core::Result<super::HostName> {
        let this = &windows_core::Interface::cast::<IWebSocketInformation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BandwidthStatistics(&self) -> windows_core::Result<BandwidthStatistics> {
        let this = &windows_core::Interface::cast::<IWebSocketInformation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BandwidthStatistics)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Protocol(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IWebSocketInformation>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Protocol)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IWebSocketInformation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebSocketInformation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SocketSslErrorSeverity) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerCertificateErrors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerCertificateErrors: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerIntermediateCertificates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerIntermediateCertificates: usize,
}
windows_core::imp::define_interface!(IWebSocketServerCustomValidationRequestedEventArgs, IWebSocketServerCustomValidationRequestedEventArgs_Vtbl, 0xffeffe48_022a_4ab7_8b36_e10af4640e6b);
impl windows_core::RuntimeType for IWebSocketServerCustomValidationRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebSocketServerCustomValidationRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub ServerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    ServerCertificate: usize,
    pub ServerCertificateErrorSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SocketSslErrorSeverity) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerCertificateErrors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerCertificateErrors: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ServerIntermediateCertificates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ServerIntermediateCertificates: usize,
    pub Reject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ControlChannelTrigger(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ControlChannelTrigger, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ControlChannelTrigger, super::super::Foundation::IClosable);
impl ControlChannelTrigger {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ControlChannelTriggerId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControlChannelTriggerId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ServerKeepAliveIntervalInMinutes(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerKeepAliveIntervalInMinutes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetServerKeepAliveIntervalInMinutes(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetServerKeepAliveIntervalInMinutes)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentKeepAliveIntervalInMinutes(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentKeepAliveIntervalInMinutes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TransportObject(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransportObject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn KeepAliveTrigger(&self) -> windows_core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeepAliveTrigger)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn PushNotificationTrigger(&self) -> windows_core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PushNotificationTrigger)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UsingTransport<P0>(&self, transport: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UsingTransport)(windows_core::Interface::as_raw(this), transport.param().abi()).ok() }
    }
    pub fn WaitForPushEnabled(&self) -> windows_core::Result<ControlChannelTriggerStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WaitForPushEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DecreaseNetworkKeepAliveInterval(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DecreaseNetworkKeepAliveInterval)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FlushTransport(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).FlushTransport)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsWakeFromLowPowerSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IControlChannelTrigger2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWakeFromLowPowerSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateControlChannelTrigger(channelid: &windows_core::HSTRING, serverkeepaliveintervalinminutes: u32) -> windows_core::Result<ControlChannelTrigger> {
        Self::IControlChannelTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateControlChannelTrigger)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(channelid), serverkeepaliveintervalinminutes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateControlChannelTriggerEx(channelid: &windows_core::HSTRING, serverkeepaliveintervalinminutes: u32, resourcerequesttype: ControlChannelTriggerResourceType) -> windows_core::Result<ControlChannelTrigger> {
        Self::IControlChannelTriggerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateControlChannelTriggerEx)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(channelid), serverkeepaliveintervalinminutes, resourcerequesttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IControlChannelTriggerFactory<R, F: FnOnce(&IControlChannelTriggerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ControlChannelTrigger, IControlChannelTriggerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ControlChannelTrigger {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IControlChannelTrigger>();
}
unsafe impl windows_core::Interface for ControlChannelTrigger {
    type Vtable = IControlChannelTrigger_Vtbl;
    const IID: windows_core::GUID = <IControlChannelTrigger as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ControlChannelTrigger {
    const NAME: &'static str = "Windows.Networking.Sockets.ControlChannelTrigger";
}
unsafe impl Send for ControlChannelTrigger {}
unsafe impl Sync for ControlChannelTrigger {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DatagramSocket(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DatagramSocket, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DatagramSocket, super::super::Foundation::IClosable);
impl DatagramSocket {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DatagramSocket, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Control(&self) -> windows_core::Result<DatagramSocketControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Information(&self) -> windows_core::Result<DatagramSocketInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Information)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectAsync<P0>(&self, remotehostname: P0, remoteservicename: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::HostName>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectAsync)(windows_core::Interface::as_raw(this), remotehostname.param().abi(), core::mem::transmute_copy(remoteservicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectWithEndpointPairAsync<P0>(&self, endpointpair: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::EndpointPair>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectWithEndpointPairAsync)(windows_core::Interface::as_raw(this), endpointpair.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BindServiceNameAsync(&self, localservicename: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BindServiceNameAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localservicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BindEndpointAsync<P0>(&self, localhostname: P0, localservicename: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::HostName>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BindEndpointAsync)(windows_core::Interface::as_raw(this), localhostname.param().abi(), core::mem::transmute_copy(localservicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn JoinMulticastGroup<P0>(&self, host: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::HostName>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).JoinMulticastGroup)(windows_core::Interface::as_raw(this), host.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamAsync<P0>(&self, remotehostname: P0, remoteservicename: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>>
    where
        P0: windows_core::Param<super::HostName>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOutputStreamAsync)(windows_core::Interface::as_raw(this), remotehostname.param().abi(), core::mem::transmute_copy(remoteservicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamWithEndpointPairAsync<P0>(&self, endpointpair: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>>
    where
        P0: windows_core::Param<super::EndpointPair>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOutputStreamWithEndpointPairAsync)(windows_core::Interface::as_raw(this), endpointpair.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MessageReceived<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DatagramSocket, DatagramSocketMessageReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageReceived)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMessageReceived(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMessageReceived)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[cfg(feature = "Networking_Connectivity")]
    pub fn BindServiceNameAndAdapterAsync<P0>(&self, localservicename: &windows_core::HSTRING, adapter: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::Connectivity::NetworkAdapter>,
    {
        let this = &windows_core::Interface::cast::<IDatagramSocket2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BindServiceNameAndAdapterAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localservicename), adapter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CancelIOAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IDatagramSocket3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelIOAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EnableTransferOwnership(&self, taskid: windows_core::GUID) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDatagramSocket3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableTransferOwnership)(windows_core::Interface::as_raw(this), taskid).ok() }
    }
    pub fn EnableTransferOwnershipWithConnectedStandbyAction(&self, taskid: windows_core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDatagramSocket3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableTransferOwnershipWithConnectedStandbyAction)(windows_core::Interface::as_raw(this), taskid, connectedstandbyaction).ok() }
    }
    pub fn TransferOwnership(&self, socketid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDatagramSocket3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TransferOwnership)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(socketid)).ok() }
    }
    pub fn TransferOwnershipWithContext<P0>(&self, socketid: &windows_core::HSTRING, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SocketActivityContext>,
    {
        let this = &windows_core::Interface::cast::<IDatagramSocket3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TransferOwnershipWithContext)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(socketid), data.param().abi()).ok() }
    }
    pub fn TransferOwnershipWithContextAndKeepAliveTime<P0>(&self, socketid: &windows_core::HSTRING, data: P0, keepalivetime: super::super::Foundation::TimeSpan) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SocketActivityContext>,
    {
        let this = &windows_core::Interface::cast::<IDatagramSocket3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TransferOwnershipWithContextAndKeepAliveTime)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(socketid), data.param().abi(), keepalivetime).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetEndpointPairsAsync<P0>(remotehostname: P0, remoteservicename: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>
    where
        P0: windows_core::Param<super::HostName>,
    {
        Self::IDatagramSocketStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEndpointPairsAsync)(windows_core::Interface::as_raw(this), remotehostname.param().abi(), core::mem::transmute_copy(remoteservicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetEndpointPairsWithSortOptionsAsync<P0>(remotehostname: P0, remoteservicename: &windows_core::HSTRING, sortoptions: super::HostNameSortOptions) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>
    where
        P0: windows_core::Param<super::HostName>,
    {
        Self::IDatagramSocketStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEndpointPairsWithSortOptionsAsync)(windows_core::Interface::as_raw(this), remotehostname.param().abi(), core::mem::transmute_copy(remoteservicename), sortoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDatagramSocketStatics<R, F: FnOnce(&IDatagramSocketStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DatagramSocket, IDatagramSocketStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DatagramSocket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDatagramSocket>();
}
unsafe impl windows_core::Interface for DatagramSocket {
    type Vtable = IDatagramSocket_Vtbl;
    const IID: windows_core::GUID = <IDatagramSocket as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DatagramSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.DatagramSocket";
}
unsafe impl Send for DatagramSocket {}
unsafe impl Sync for DatagramSocket {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DatagramSocketControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DatagramSocketControl, windows_core::IUnknown, windows_core::IInspectable);
impl DatagramSocketControl {
    pub fn QualityOfService(&self) -> windows_core::Result<SocketQualityOfService> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QualityOfService)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetQualityOfService(&self, value: SocketQualityOfService) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQualityOfService)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutboundUnicastHopLimit(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutboundUnicastHopLimit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOutboundUnicastHopLimit(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOutboundUnicastHopLimit)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InboundBufferSizeInBytes(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IDatagramSocketControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InboundBufferSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInboundBufferSizeInBytes(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDatagramSocketControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInboundBufferSizeInBytes)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DontFragment(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDatagramSocketControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DontFragment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDontFragment(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDatagramSocketControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDontFragment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MulticastOnly(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IDatagramSocketControl3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MulticastOnly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMulticastOnly(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDatagramSocketControl3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMulticastOnly)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for DatagramSocketControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDatagramSocketControl>();
}
unsafe impl windows_core::Interface for DatagramSocketControl {
    type Vtable = IDatagramSocketControl_Vtbl;
    const IID: windows_core::GUID = <IDatagramSocketControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DatagramSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.DatagramSocketControl";
}
unsafe impl Send for DatagramSocketControl {}
unsafe impl Sync for DatagramSocketControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DatagramSocketInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DatagramSocketInformation, windows_core::IUnknown, windows_core::IInspectable);
impl DatagramSocketInformation {
    pub fn LocalAddress(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalPort(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalPort)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoteAddress(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemotePort(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemotePort)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DatagramSocketInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDatagramSocketInformation>();
}
unsafe impl windows_core::Interface for DatagramSocketInformation {
    type Vtable = IDatagramSocketInformation_Vtbl;
    const IID: windows_core::GUID = <IDatagramSocketInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DatagramSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.DatagramSocketInformation";
}
unsafe impl Send for DatagramSocketInformation {}
unsafe impl Sync for DatagramSocketInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DatagramSocketMessageReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DatagramSocketMessageReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DatagramSocketMessageReceivedEventArgs {
    pub fn RemoteAddress(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemotePort(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemotePort)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalAddress(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetDataReader(&self) -> windows_core::Result<super::super::Storage::Streams::DataReader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDataReader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetDataStream(&self) -> windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDataStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DatagramSocketMessageReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDatagramSocketMessageReceivedEventArgs>();
}
unsafe impl windows_core::Interface for DatagramSocketMessageReceivedEventArgs {
    type Vtable = IDatagramSocketMessageReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDatagramSocketMessageReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DatagramSocketMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.DatagramSocketMessageReceivedEventArgs";
}
unsafe impl Send for DatagramSocketMessageReceivedEventArgs {}
unsafe impl Sync for DatagramSocketMessageReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MessageWebSocket(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MessageWebSocket, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MessageWebSocket, super::super::Foundation::IClosable, IWebSocket);
impl MessageWebSocket {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MessageWebSocket, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Control(&self) -> windows_core::Result<MessageWebSocketControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Information(&self) -> windows_core::Result<MessageWebSocketInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Information)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MessageReceived<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MessageWebSocket, MessageWebSocketMessageReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageReceived)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMessageReceived(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMessageReceived)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn ServerCustomValidationRequested<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<MessageWebSocket, WebSocketServerCustomValidationRequestedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IMessageWebSocket2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCustomValidationRequested)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveServerCustomValidationRequested(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMessageWebSocket2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveServerCustomValidationRequested)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SendNonfinalFrameAsync<P0>(&self, data: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IMessageWebSocket3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendNonfinalFrameAsync)(windows_core::Interface::as_raw(this), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SendFinalFrameAsync<P0>(&self, data: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IMessageWebSocket3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendFinalFrameAsync)(windows_core::Interface::as_raw(this), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &windows_core::Interface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectAsync<P0>(&self, uri: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRequestHeader(&self, headername: &windows_core::HSTRING, headervalue: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebSocket>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRequestHeader)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(headername), core::mem::transmute_copy(headervalue)).ok() }
    }
    pub fn Closed<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebSocket>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn CloseWithStatus(&self, code: u16, reason: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebSocket>(self)?;
        unsafe { (windows_core::Interface::vtable(this).CloseWithStatus)(windows_core::Interface::as_raw(this), code, core::mem::transmute_copy(reason)).ok() }
    }
}
impl windows_core::RuntimeType for MessageWebSocket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMessageWebSocket>();
}
unsafe impl windows_core::Interface for MessageWebSocket {
    type Vtable = IMessageWebSocket_Vtbl;
    const IID: windows_core::GUID = <IMessageWebSocket as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MessageWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.MessageWebSocket";
}
unsafe impl Send for MessageWebSocket {}
unsafe impl Sync for MessageWebSocket {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MessageWebSocketControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MessageWebSocketControl, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MessageWebSocketControl, IWebSocketControl, IWebSocketControl2);
impl MessageWebSocketControl {
    pub fn MaxMessageSize(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxMessageSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxMessageSize(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxMessageSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MessageType(&self) -> windows_core::Result<SocketMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMessageType(&self, value: SocketMessageType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMessageType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredUnsolicitedPongInterval(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredUnsolicitedPongInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredUnsolicitedPongInterval(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredUnsolicitedPongInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualUnsolicitedPongInterval(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualUnsolicitedPongInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReceiveMode(&self) -> windows_core::Result<MessageWebSocketReceiveMode> {
        let this = &windows_core::Interface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReceiveMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReceiveMode(&self, value: MessageWebSocketReceiveMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetReceiveMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ClientCertificate(&self) -> windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &windows_core::Interface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClientCertificate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn SetClientCertificate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Cryptography::Certificates::Certificate>,
    {
        let this = &windows_core::Interface::cast::<IMessageWebSocketControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetClientCertificate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OutboundBufferSizeInBytes(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutboundBufferSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutboundBufferSizeInBytes)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetServerCredential)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProxyCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProxyCredential)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedProtocols(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedProtocols)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &windows_core::Interface::cast::<IWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IgnorableServerCertificateErrors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MessageWebSocketControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMessageWebSocketControl>();
}
unsafe impl windows_core::Interface for MessageWebSocketControl {
    type Vtable = IMessageWebSocketControl_Vtbl;
    const IID: windows_core::GUID = <IMessageWebSocketControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MessageWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.MessageWebSocketControl";
}
unsafe impl Send for MessageWebSocketControl {}
unsafe impl Sync for MessageWebSocketControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MessageWebSocketInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MessageWebSocketInformation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MessageWebSocketInformation, IWebSocketInformation, IWebSocketInformation2);
impl MessageWebSocketInformation {
    pub fn LocalAddress(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BandwidthStatistics(&self) -> windows_core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BandwidthStatistics)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Protocol(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Protocol)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &windows_core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ServerCertificateErrorSeverity(&self) -> windows_core::Result<SocketSslErrorSeverity> {
        let this = &windows_core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificateErrorSeverity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &windows_core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificateErrors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = &windows_core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerIntermediateCertificates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MessageWebSocketInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebSocketInformation>();
}
unsafe impl windows_core::Interface for MessageWebSocketInformation {
    type Vtable = IWebSocketInformation_Vtbl;
    const IID: windows_core::GUID = <IWebSocketInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MessageWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.MessageWebSocketInformation";
}
unsafe impl Send for MessageWebSocketInformation {}
unsafe impl Sync for MessageWebSocketInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MessageWebSocketMessageReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MessageWebSocketMessageReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MessageWebSocketMessageReceivedEventArgs {
    pub fn MessageType(&self) -> windows_core::Result<SocketMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetDataReader(&self) -> windows_core::Result<super::super::Storage::Streams::DataReader> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDataReader)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetDataStream(&self) -> windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDataStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsMessageComplete(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMessageWebSocketMessageReceivedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMessageComplete)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MessageWebSocketMessageReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMessageWebSocketMessageReceivedEventArgs>();
}
unsafe impl windows_core::Interface for MessageWebSocketMessageReceivedEventArgs {
    type Vtable = IMessageWebSocketMessageReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IMessageWebSocketMessageReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MessageWebSocketMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.MessageWebSocketMessageReceivedEventArgs";
}
unsafe impl Send for MessageWebSocketMessageReceivedEventArgs {}
unsafe impl Sync for MessageWebSocketMessageReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ServerMessageWebSocket(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ServerMessageWebSocket, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ServerMessageWebSocket, super::super::Foundation::IClosable);
impl ServerMessageWebSocket {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MessageReceived<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, MessageWebSocketMessageReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageReceived)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMessageReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMessageReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Control(&self) -> windows_core::Result<ServerMessageWebSocketControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Information(&self) -> windows_core::Result<ServerMessageWebSocketInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Information)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Closed<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, WebSocketClosedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CloseWithStatus(&self, code: u16, reason: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CloseWithStatus)(windows_core::Interface::as_raw(this), code, core::mem::transmute_copy(reason)).ok() }
    }
}
impl windows_core::RuntimeType for ServerMessageWebSocket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IServerMessageWebSocket>();
}
unsafe impl windows_core::Interface for ServerMessageWebSocket {
    type Vtable = IServerMessageWebSocket_Vtbl;
    const IID: windows_core::GUID = <IServerMessageWebSocket as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ServerMessageWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.ServerMessageWebSocket";
}
unsafe impl Send for ServerMessageWebSocket {}
unsafe impl Sync for ServerMessageWebSocket {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ServerMessageWebSocketControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ServerMessageWebSocketControl, windows_core::IUnknown, windows_core::IInspectable);
impl ServerMessageWebSocketControl {
    pub fn MessageType(&self) -> windows_core::Result<SocketMessageType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMessageType(&self, value: SocketMessageType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMessageType)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for ServerMessageWebSocketControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IServerMessageWebSocketControl>();
}
unsafe impl windows_core::Interface for ServerMessageWebSocketControl {
    type Vtable = IServerMessageWebSocketControl_Vtbl;
    const IID: windows_core::GUID = <IServerMessageWebSocketControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ServerMessageWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.ServerMessageWebSocketControl";
}
unsafe impl Send for ServerMessageWebSocketControl {}
unsafe impl Sync for ServerMessageWebSocketControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ServerMessageWebSocketInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ServerMessageWebSocketInformation, windows_core::IUnknown, windows_core::IInspectable);
impl ServerMessageWebSocketInformation {
    pub fn BandwidthStatistics(&self) -> windows_core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BandwidthStatistics)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Protocol(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Protocol)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalAddress(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ServerMessageWebSocketInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IServerMessageWebSocketInformation>();
}
unsafe impl windows_core::Interface for ServerMessageWebSocketInformation {
    type Vtable = IServerMessageWebSocketInformation_Vtbl;
    const IID: windows_core::GUID = <IServerMessageWebSocketInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ServerMessageWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.ServerMessageWebSocketInformation";
}
unsafe impl Send for ServerMessageWebSocketInformation {}
unsafe impl Sync for ServerMessageWebSocketInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ServerStreamWebSocket(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ServerStreamWebSocket, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ServerStreamWebSocket, super::super::Foundation::IClosable);
impl ServerStreamWebSocket {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Information(&self) -> windows_core::Result<ServerStreamWebSocketInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Information)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Closed<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ServerStreamWebSocket, WebSocketClosedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CloseWithStatus(&self, code: u16, reason: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CloseWithStatus)(windows_core::Interface::as_raw(this), code, core::mem::transmute_copy(reason)).ok() }
    }
}
impl windows_core::RuntimeType for ServerStreamWebSocket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IServerStreamWebSocket>();
}
unsafe impl windows_core::Interface for ServerStreamWebSocket {
    type Vtable = IServerStreamWebSocket_Vtbl;
    const IID: windows_core::GUID = <IServerStreamWebSocket as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ServerStreamWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.ServerStreamWebSocket";
}
unsafe impl Send for ServerStreamWebSocket {}
unsafe impl Sync for ServerStreamWebSocket {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ServerStreamWebSocketInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ServerStreamWebSocketInformation, windows_core::IUnknown, windows_core::IInspectable);
impl ServerStreamWebSocketInformation {
    pub fn BandwidthStatistics(&self) -> windows_core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BandwidthStatistics)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Protocol(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Protocol)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalAddress(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ServerStreamWebSocketInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IServerStreamWebSocketInformation>();
}
unsafe impl windows_core::Interface for ServerStreamWebSocketInformation {
    type Vtable = IServerStreamWebSocketInformation_Vtbl;
    const IID: windows_core::GUID = <IServerStreamWebSocketInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ServerStreamWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.ServerStreamWebSocketInformation";
}
unsafe impl Send for ServerStreamWebSocketInformation {}
unsafe impl Sync for ServerStreamWebSocketInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SocketActivityContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SocketActivityContext, windows_core::IUnknown, windows_core::IInspectable);
impl SocketActivityContext {
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<P0>(data: P0) -> windows_core::Result<SocketActivityContext>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ISocketActivityContextFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISocketActivityContextFactory<R, F: FnOnce(&ISocketActivityContextFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SocketActivityContext, ISocketActivityContextFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SocketActivityContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISocketActivityContext>();
}
unsafe impl windows_core::Interface for SocketActivityContext {
    type Vtable = ISocketActivityContext_Vtbl;
    const IID: windows_core::GUID = <ISocketActivityContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SocketActivityContext {
    const NAME: &'static str = "Windows.Networking.Sockets.SocketActivityContext";
}
unsafe impl Send for SocketActivityContext {}
unsafe impl Sync for SocketActivityContext {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SocketActivityInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SocketActivityInformation, windows_core::IUnknown, windows_core::IInspectable);
impl SocketActivityInformation {
    pub fn TaskId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SocketKind(&self) -> windows_core::Result<SocketActivityKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SocketKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Context(&self) -> windows_core::Result<SocketActivityContext> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Context)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DatagramSocket(&self) -> windows_core::Result<DatagramSocket> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DatagramSocket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StreamSocket(&self) -> windows_core::Result<StreamSocket> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StreamSocket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StreamSocketListener(&self) -> windows_core::Result<StreamSocketListener> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StreamSocketListener)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllSockets() -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, SocketActivityInformation>> {
        Self::ISocketActivityInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllSockets)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISocketActivityInformationStatics<R, F: FnOnce(&ISocketActivityInformationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SocketActivityInformation, ISocketActivityInformationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SocketActivityInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISocketActivityInformation>();
}
unsafe impl windows_core::Interface for SocketActivityInformation {
    type Vtable = ISocketActivityInformation_Vtbl;
    const IID: windows_core::GUID = <ISocketActivityInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SocketActivityInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.SocketActivityInformation";
}
unsafe impl Send for SocketActivityInformation {}
unsafe impl Sync for SocketActivityInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SocketActivityTriggerDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SocketActivityTriggerDetails, windows_core::IUnknown, windows_core::IInspectable);
impl SocketActivityTriggerDetails {
    pub fn Reason(&self) -> windows_core::Result<SocketActivityTriggerReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SocketInformation(&self) -> windows_core::Result<SocketActivityInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SocketInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SocketActivityTriggerDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISocketActivityTriggerDetails>();
}
unsafe impl windows_core::Interface for SocketActivityTriggerDetails {
    type Vtable = ISocketActivityTriggerDetails_Vtbl;
    const IID: windows_core::GUID = <ISocketActivityTriggerDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SocketActivityTriggerDetails {
    const NAME: &'static str = "Windows.Networking.Sockets.SocketActivityTriggerDetails";
}
unsafe impl Send for SocketActivityTriggerDetails {}
unsafe impl Sync for SocketActivityTriggerDetails {}
pub struct SocketError;
impl SocketError {
    pub fn GetStatus(hresult: i32) -> windows_core::Result<SocketErrorStatus> {
        Self::ISocketErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatus)(windows_core::Interface::as_raw(this), hresult, &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn ISocketErrorStatics<R, F: FnOnce(&ISocketErrorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SocketError, ISocketErrorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for SocketError {
    const NAME: &'static str = "Windows.Networking.Sockets.SocketError";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StreamSocket(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StreamSocket, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StreamSocket, super::super::Foundation::IClosable);
impl StreamSocket {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StreamSocket, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Control(&self) -> windows_core::Result<StreamSocketControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Information(&self) -> windows_core::Result<StreamSocketInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Information)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectWithEndpointPairAsync<P0>(&self, endpointpair: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::EndpointPair>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectWithEndpointPairAsync)(windows_core::Interface::as_raw(this), endpointpair.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectAsync<P0>(&self, remotehostname: P0, remoteservicename: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::HostName>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectAsync)(windows_core::Interface::as_raw(this), remotehostname.param().abi(), core::mem::transmute_copy(remoteservicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectWithEndpointPairAndProtectionLevelAsync<P0>(&self, endpointpair: P0, protectionlevel: SocketProtectionLevel) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::EndpointPair>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectWithEndpointPairAndProtectionLevelAsync)(windows_core::Interface::as_raw(this), endpointpair.param().abi(), protectionlevel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectWithProtectionLevelAsync<P0>(&self, remotehostname: P0, remoteservicename: &windows_core::HSTRING, protectionlevel: SocketProtectionLevel) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::HostName>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectWithProtectionLevelAsync)(windows_core::Interface::as_raw(this), remotehostname.param().abi(), core::mem::transmute_copy(remoteservicename), protectionlevel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UpgradeToSslAsync<P0>(&self, protectionlevel: SocketProtectionLevel, validationhostname: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::HostName>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpgradeToSslAsync)(windows_core::Interface::as_raw(this), protectionlevel, validationhostname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Networking_Connectivity")]
    pub fn ConnectWithProtectionLevelAndAdapterAsync<P0, P1>(&self, remotehostname: P0, remoteservicename: &windows_core::HSTRING, protectionlevel: SocketProtectionLevel, adapter: P1) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::HostName>,
        P1: windows_core::Param<super::Connectivity::NetworkAdapter>,
    {
        let this = &windows_core::Interface::cast::<IStreamSocket2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectWithProtectionLevelAndAdapterAsync)(windows_core::Interface::as_raw(this), remotehostname.param().abi(), core::mem::transmute_copy(remoteservicename), protectionlevel, adapter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CancelIOAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IStreamSocket3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelIOAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EnableTransferOwnership(&self, taskid: windows_core::GUID) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStreamSocket3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableTransferOwnership)(windows_core::Interface::as_raw(this), taskid).ok() }
    }
    pub fn EnableTransferOwnershipWithConnectedStandbyAction(&self, taskid: windows_core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStreamSocket3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableTransferOwnershipWithConnectedStandbyAction)(windows_core::Interface::as_raw(this), taskid, connectedstandbyaction).ok() }
    }
    pub fn TransferOwnership(&self, socketid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStreamSocket3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TransferOwnership)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(socketid)).ok() }
    }
    pub fn TransferOwnershipWithContext<P0>(&self, socketid: &windows_core::HSTRING, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SocketActivityContext>,
    {
        let this = &windows_core::Interface::cast::<IStreamSocket3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TransferOwnershipWithContext)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(socketid), data.param().abi()).ok() }
    }
    pub fn TransferOwnershipWithContextAndKeepAliveTime<P0>(&self, socketid: &windows_core::HSTRING, data: P0, keepalivetime: super::super::Foundation::TimeSpan) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SocketActivityContext>,
    {
        let this = &windows_core::Interface::cast::<IStreamSocket3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TransferOwnershipWithContextAndKeepAliveTime)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(socketid), data.param().abi(), keepalivetime).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetEndpointPairsAsync<P0>(remotehostname: P0, remoteservicename: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>
    where
        P0: windows_core::Param<super::HostName>,
    {
        Self::IStreamSocketStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEndpointPairsAsync)(windows_core::Interface::as_raw(this), remotehostname.param().abi(), core::mem::transmute_copy(remoteservicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetEndpointPairsWithSortOptionsAsync<P0>(remotehostname: P0, remoteservicename: &windows_core::HSTRING, sortoptions: super::HostNameSortOptions) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>
    where
        P0: windows_core::Param<super::HostName>,
    {
        Self::IStreamSocketStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEndpointPairsWithSortOptionsAsync)(windows_core::Interface::as_raw(this), remotehostname.param().abi(), core::mem::transmute_copy(remoteservicename), sortoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IStreamSocketStatics<R, F: FnOnce(&IStreamSocketStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StreamSocket, IStreamSocketStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for StreamSocket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStreamSocket>();
}
unsafe impl windows_core::Interface for StreamSocket {
    type Vtable = IStreamSocket_Vtbl;
    const IID: windows_core::GUID = <IStreamSocket as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StreamSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocket";
}
unsafe impl Send for StreamSocket {}
unsafe impl Sync for StreamSocket {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StreamSocketControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StreamSocketControl, windows_core::IUnknown, windows_core::IInspectable);
impl StreamSocketControl {
    pub fn NoDelay(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NoDelay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNoDelay(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNoDelay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeepAlive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeepAlive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeepAlive(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeepAlive)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutboundBufferSizeInBytes(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutboundBufferSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOutboundBufferSizeInBytes)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn QualityOfService(&self) -> windows_core::Result<SocketQualityOfService> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QualityOfService)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetQualityOfService(&self, value: SocketQualityOfService) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQualityOfService)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutboundUnicastHopLimit(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutboundUnicastHopLimit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOutboundUnicastHopLimit(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOutboundUnicastHopLimit)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &windows_core::Interface::cast::<IStreamSocketControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IgnorableServerCertificateErrors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SerializeConnectionAttempts(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IStreamSocketControl3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SerializeConnectionAttempts)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSerializeConnectionAttempts(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStreamSocketControl3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSerializeConnectionAttempts)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ClientCertificate(&self) -> windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &windows_core::Interface::cast::<IStreamSocketControl3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClientCertificate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn SetClientCertificate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Cryptography::Certificates::Certificate>,
    {
        let this = &windows_core::Interface::cast::<IStreamSocketControl3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetClientCertificate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn MinProtectionLevel(&self) -> windows_core::Result<SocketProtectionLevel> {
        let this = &windows_core::Interface::cast::<IStreamSocketControl4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinProtectionLevel(&self, value: SocketProtectionLevel) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStreamSocketControl4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinProtectionLevel)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for StreamSocketControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStreamSocketControl>();
}
unsafe impl windows_core::Interface for StreamSocketControl {
    type Vtable = IStreamSocketControl_Vtbl;
    const IID: windows_core::GUID = <IStreamSocketControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StreamSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketControl";
}
unsafe impl Send for StreamSocketControl {}
unsafe impl Sync for StreamSocketControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StreamSocketInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StreamSocketInformation, windows_core::IUnknown, windows_core::IInspectable);
impl StreamSocketInformation {
    pub fn LocalAddress(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalPort(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalPort)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoteHostName(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteHostName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoteAddress(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoteServiceName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteServiceName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemotePort(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemotePort)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RoundTripTimeStatistics(&self) -> windows_core::Result<RoundTripTimeStatistics> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RoundTripTimeStatistics)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BandwidthStatistics(&self) -> windows_core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BandwidthStatistics)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProtectionLevel(&self) -> windows_core::Result<SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SessionKey(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SessionKey)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ServerCertificateErrorSeverity(&self) -> windows_core::Result<SocketSslErrorSeverity> {
        let this = &windows_core::Interface::cast::<IStreamSocketInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificateErrorSeverity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &windows_core::Interface::cast::<IStreamSocketInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificateErrors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &windows_core::Interface::cast::<IStreamSocketInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = &windows_core::Interface::cast::<IStreamSocketInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerIntermediateCertificates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StreamSocketInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStreamSocketInformation>();
}
unsafe impl windows_core::Interface for StreamSocketInformation {
    type Vtable = IStreamSocketInformation_Vtbl;
    const IID: windows_core::GUID = <IStreamSocketInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StreamSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketInformation";
}
unsafe impl Send for StreamSocketInformation {}
unsafe impl Sync for StreamSocketInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StreamSocketListener(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StreamSocketListener, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StreamSocketListener, super::super::Foundation::IClosable);
impl StreamSocketListener {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StreamSocketListener, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Control(&self) -> windows_core::Result<StreamSocketListenerControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Information(&self) -> windows_core::Result<StreamSocketListenerInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Information)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BindServiceNameAsync(&self, localservicename: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BindServiceNameAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localservicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BindEndpointAsync<P0>(&self, localhostname: P0, localservicename: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::HostName>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BindEndpointAsync)(windows_core::Interface::as_raw(this), localhostname.param().abi(), core::mem::transmute_copy(localservicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectionReceived<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<StreamSocketListener, StreamSocketListenerConnectionReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionReceived)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveConnectionReceived(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveConnectionReceived)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn BindServiceNameWithProtectionLevelAsync(&self, localservicename: &windows_core::HSTRING, protectionlevel: SocketProtectionLevel) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IStreamSocketListener2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BindServiceNameWithProtectionLevelAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localservicename), protectionlevel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Networking_Connectivity")]
    pub fn BindServiceNameWithProtectionLevelAndAdapterAsync<P0>(&self, localservicename: &windows_core::HSTRING, protectionlevel: SocketProtectionLevel, adapter: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::Connectivity::NetworkAdapter>,
    {
        let this = &windows_core::Interface::cast::<IStreamSocketListener2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BindServiceNameWithProtectionLevelAndAdapterAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(localservicename), protectionlevel, adapter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CancelIOAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IStreamSocketListener3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelIOAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EnableTransferOwnership(&self, taskid: windows_core::GUID) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStreamSocketListener3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableTransferOwnership)(windows_core::Interface::as_raw(this), taskid).ok() }
    }
    pub fn EnableTransferOwnershipWithConnectedStandbyAction(&self, taskid: windows_core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStreamSocketListener3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).EnableTransferOwnershipWithConnectedStandbyAction)(windows_core::Interface::as_raw(this), taskid, connectedstandbyaction).ok() }
    }
    pub fn TransferOwnership(&self, socketid: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStreamSocketListener3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TransferOwnership)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(socketid)).ok() }
    }
    pub fn TransferOwnershipWithContext<P0>(&self, socketid: &windows_core::HSTRING, data: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SocketActivityContext>,
    {
        let this = &windows_core::Interface::cast::<IStreamSocketListener3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TransferOwnershipWithContext)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(socketid), data.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for StreamSocketListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStreamSocketListener>();
}
unsafe impl windows_core::Interface for StreamSocketListener {
    type Vtable = IStreamSocketListener_Vtbl;
    const IID: windows_core::GUID = <IStreamSocketListener as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StreamSocketListener {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketListener";
}
unsafe impl Send for StreamSocketListener {}
unsafe impl Sync for StreamSocketListener {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StreamSocketListenerConnectionReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StreamSocketListenerConnectionReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl StreamSocketListenerConnectionReceivedEventArgs {
    pub fn Socket(&self) -> windows_core::Result<StreamSocket> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Socket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StreamSocketListenerConnectionReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStreamSocketListenerConnectionReceivedEventArgs>();
}
unsafe impl windows_core::Interface for StreamSocketListenerConnectionReceivedEventArgs {
    type Vtable = IStreamSocketListenerConnectionReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IStreamSocketListenerConnectionReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StreamSocketListenerConnectionReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketListenerConnectionReceivedEventArgs";
}
unsafe impl Send for StreamSocketListenerConnectionReceivedEventArgs {}
unsafe impl Sync for StreamSocketListenerConnectionReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StreamSocketListenerControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StreamSocketListenerControl, windows_core::IUnknown, windows_core::IInspectable);
impl StreamSocketListenerControl {
    pub fn QualityOfService(&self) -> windows_core::Result<SocketQualityOfService> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QualityOfService)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetQualityOfService(&self, value: SocketQualityOfService) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQualityOfService)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NoDelay(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NoDelay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNoDelay(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNoDelay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeepAlive(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeepAlive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeepAlive(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeepAlive)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutboundBufferSizeInBytes(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutboundBufferSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutboundBufferSizeInBytes)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutboundUnicastHopLimit(&self) -> windows_core::Result<u8> {
        let this = &windows_core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutboundUnicastHopLimit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOutboundUnicastHopLimit(&self, value: u8) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStreamSocketListenerControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutboundUnicastHopLimit)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for StreamSocketListenerControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStreamSocketListenerControl>();
}
unsafe impl windows_core::Interface for StreamSocketListenerControl {
    type Vtable = IStreamSocketListenerControl_Vtbl;
    const IID: windows_core::GUID = <IStreamSocketListenerControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StreamSocketListenerControl {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketListenerControl";
}
unsafe impl Send for StreamSocketListenerControl {}
unsafe impl Sync for StreamSocketListenerControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StreamSocketListenerInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StreamSocketListenerInformation, windows_core::IUnknown, windows_core::IInspectable);
impl StreamSocketListenerInformation {
    pub fn LocalPort(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalPort)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StreamSocketListenerInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStreamSocketListenerInformation>();
}
unsafe impl windows_core::Interface for StreamSocketListenerInformation {
    type Vtable = IStreamSocketListenerInformation_Vtbl;
    const IID: windows_core::GUID = <IStreamSocketListenerInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StreamSocketListenerInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamSocketListenerInformation";
}
unsafe impl Send for StreamSocketListenerInformation {}
unsafe impl Sync for StreamSocketListenerInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StreamWebSocket(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StreamWebSocket, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StreamWebSocket, super::super::Foundation::IClosable, IWebSocket);
impl StreamWebSocket {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StreamWebSocket, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Control(&self) -> windows_core::Result<StreamWebSocketControl> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Control)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Information(&self) -> windows_core::Result<StreamWebSocketInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Information)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ServerCustomValidationRequested<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<StreamWebSocket, WebSocketServerCustomValidationRequestedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IStreamWebSocket2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCustomValidationRequested)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveServerCustomValidationRequested(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStreamWebSocket2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveServerCustomValidationRequested)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &windows_core::Interface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectAsync<P0>(&self, uri: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRequestHeader(&self, headername: &windows_core::HSTRING, headervalue: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebSocket>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRequestHeader)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(headername), core::mem::transmute_copy(headervalue)).ok() }
    }
    pub fn Closed<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<IWebSocket>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebSocket>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn CloseWithStatus(&self, code: u16, reason: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebSocket>(self)?;
        unsafe { (windows_core::Interface::vtable(this).CloseWithStatus)(windows_core::Interface::as_raw(this), code, core::mem::transmute_copy(reason)).ok() }
    }
}
impl windows_core::RuntimeType for StreamWebSocket {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStreamWebSocket>();
}
unsafe impl windows_core::Interface for StreamWebSocket {
    type Vtable = IStreamWebSocket_Vtbl;
    const IID: windows_core::GUID = <IStreamWebSocket as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StreamWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamWebSocket";
}
unsafe impl Send for StreamWebSocket {}
unsafe impl Sync for StreamWebSocket {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StreamWebSocketControl(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StreamWebSocketControl, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StreamWebSocketControl, IWebSocketControl, IWebSocketControl2);
impl StreamWebSocketControl {
    pub fn NoDelay(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NoDelay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNoDelay(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNoDelay)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredUnsolicitedPongInterval(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IStreamWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredUnsolicitedPongInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredUnsolicitedPongInterval(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IStreamWebSocketControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredUnsolicitedPongInterval)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualUnsolicitedPongInterval(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &windows_core::Interface::cast::<IStreamWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualUnsolicitedPongInterval)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ClientCertificate(&self) -> windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &windows_core::Interface::cast::<IStreamWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ClientCertificate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn SetClientCertificate<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Cryptography::Certificates::Certificate>,
    {
        let this = &windows_core::Interface::cast::<IStreamWebSocketControl2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetClientCertificate)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn OutboundBufferSizeInBytes(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutboundBufferSizeInBytes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOutboundBufferSizeInBytes(&self, value: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOutboundBufferSizeInBytes)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetServerCredential)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProxyCredential)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProxyCredential)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedProtocols(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IWebSocketControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedProtocols)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn IgnorableServerCertificateErrors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &windows_core::Interface::cast::<IWebSocketControl2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IgnorableServerCertificateErrors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StreamWebSocketControl {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStreamWebSocketControl>();
}
unsafe impl windows_core::Interface for StreamWebSocketControl {
    type Vtable = IStreamWebSocketControl_Vtbl;
    const IID: windows_core::GUID = <IStreamWebSocketControl as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StreamWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamWebSocketControl";
}
unsafe impl Send for StreamWebSocketControl {}
unsafe impl Sync for StreamWebSocketControl {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StreamWebSocketInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StreamWebSocketInformation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StreamWebSocketInformation, IWebSocketInformation, IWebSocketInformation2);
impl StreamWebSocketInformation {
    pub fn LocalAddress(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BandwidthStatistics(&self) -> windows_core::Result<BandwidthStatistics> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BandwidthStatistics)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Protocol(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Protocol)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = &windows_core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ServerCertificateErrorSeverity(&self) -> windows_core::Result<SocketSslErrorSeverity> {
        let this = &windows_core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificateErrorSeverity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = &windows_core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificateErrors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = &windows_core::Interface::cast::<IWebSocketInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerIntermediateCertificates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StreamWebSocketInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebSocketInformation>();
}
unsafe impl windows_core::Interface for StreamWebSocketInformation {
    type Vtable = IWebSocketInformation_Vtbl;
    const IID: windows_core::GUID = <IWebSocketInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StreamWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.StreamWebSocketInformation";
}
unsafe impl Send for StreamWebSocketInformation {}
unsafe impl Sync for StreamWebSocketInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WebSocketClosedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebSocketClosedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebSocketClosedEventArgs {
    pub fn Code(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Code)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Reason(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebSocketClosedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebSocketClosedEventArgs>();
}
unsafe impl windows_core::Interface for WebSocketClosedEventArgs {
    type Vtable = IWebSocketClosedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWebSocketClosedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebSocketClosedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.WebSocketClosedEventArgs";
}
unsafe impl Send for WebSocketClosedEventArgs {}
unsafe impl Sync for WebSocketClosedEventArgs {}
pub struct WebSocketError;
impl WebSocketError {
    #[cfg(feature = "Web")]
    pub fn GetStatus(hresult: i32) -> windows_core::Result<super::super::Web::WebErrorStatus> {
        Self::IWebSocketErrorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStatus)(windows_core::Interface::as_raw(this), hresult, &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebSocketErrorStatics<R, F: FnOnce(&IWebSocketErrorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebSocketError, IWebSocketErrorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for WebSocketError {
    const NAME: &'static str = "Windows.Networking.Sockets.WebSocketError";
}
#[cfg(feature = "ApplicationModel_Background")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WebSocketKeepAlive(windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Background")]
windows_core::imp::interface_hierarchy!(WebSocketKeepAlive, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Background")]
windows_core::imp::required_hierarchy!(WebSocketKeepAlive, super::super::ApplicationModel::Background::IBackgroundTask);
#[cfg(feature = "ApplicationModel_Background")]
impl WebSocketKeepAlive {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WebSocketKeepAlive, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn Run<P0>(&self, taskinstance: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::ApplicationModel::Background::IBackgroundTaskInstance>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Run)(windows_core::Interface::as_raw(this), taskinstance.param().abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl windows_core::RuntimeType for WebSocketKeepAlive {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, super::super::ApplicationModel::Background::IBackgroundTask>();
}
#[cfg(feature = "ApplicationModel_Background")]
unsafe impl windows_core::Interface for WebSocketKeepAlive {
    type Vtable = super::super::ApplicationModel::Background::IBackgroundTask_Vtbl;
    const IID: windows_core::GUID = <super::super::ApplicationModel::Background::IBackgroundTask as windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Background")]
impl windows_core::RuntimeName for WebSocketKeepAlive {
    const NAME: &'static str = "Windows.Networking.Sockets.WebSocketKeepAlive";
}
#[cfg(feature = "ApplicationModel_Background")]
unsafe impl Send for WebSocketKeepAlive {}
#[cfg(feature = "ApplicationModel_Background")]
unsafe impl Sync for WebSocketKeepAlive {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WebSocketServerCustomValidationRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebSocketServerCustomValidationRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WebSocketServerCustomValidationRequestedEventArgs {
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn ServerCertificate(&self) -> windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificate)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ServerCertificateErrorSeverity(&self) -> windows_core::Result<SocketSslErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificateErrorSeverity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerCertificateErrors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerCertificateErrors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ServerIntermediateCertificates(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServerIntermediateCertificates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Reject(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Reject)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WebSocketServerCustomValidationRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebSocketServerCustomValidationRequestedEventArgs>();
}
unsafe impl windows_core::Interface for WebSocketServerCustomValidationRequestedEventArgs {
    type Vtable = IWebSocketServerCustomValidationRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IWebSocketServerCustomValidationRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebSocketServerCustomValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.WebSocketServerCustomValidationRequestedEventArgs";
}
unsafe impl Send for WebSocketServerCustomValidationRequestedEventArgs {}
unsafe impl Sync for WebSocketServerCustomValidationRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ControlChannelTriggerResetReason(pub i32);
impl ControlChannelTriggerResetReason {
    pub const FastUserSwitched: Self = Self(0i32);
    pub const LowPowerExit: Self = Self(1i32);
    pub const QuietHoursExit: Self = Self(2i32);
    pub const ApplicationRestart: Self = Self(3i32);
}
impl windows_core::TypeKind for ControlChannelTriggerResetReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ControlChannelTriggerResetReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ControlChannelTriggerResetReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ControlChannelTriggerResetReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.ControlChannelTriggerResetReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ControlChannelTriggerResourceType(pub i32);
impl ControlChannelTriggerResourceType {
    pub const RequestSoftwareSlot: Self = Self(0i32);
    pub const RequestHardwareSlot: Self = Self(1i32);
}
impl windows_core::TypeKind for ControlChannelTriggerResourceType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ControlChannelTriggerResourceType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ControlChannelTriggerResourceType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ControlChannelTriggerResourceType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.ControlChannelTriggerResourceType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ControlChannelTriggerStatus(pub i32);
impl ControlChannelTriggerStatus {
    pub const HardwareSlotRequested: Self = Self(0i32);
    pub const SoftwareSlotAllocated: Self = Self(1i32);
    pub const HardwareSlotAllocated: Self = Self(2i32);
    pub const PolicyError: Self = Self(3i32);
    pub const SystemError: Self = Self(4i32);
    pub const TransportDisconnected: Self = Self(5i32);
    pub const ServiceUnavailable: Self = Self(6i32);
}
impl windows_core::TypeKind for ControlChannelTriggerStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ControlChannelTriggerStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ControlChannelTriggerStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ControlChannelTriggerStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.ControlChannelTriggerStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MessageWebSocketReceiveMode(pub i32);
impl MessageWebSocketReceiveMode {
    pub const FullMessage: Self = Self(0i32);
    pub const PartialMessage: Self = Self(1i32);
}
impl windows_core::TypeKind for MessageWebSocketReceiveMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MessageWebSocketReceiveMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MessageWebSocketReceiveMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MessageWebSocketReceiveMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.MessageWebSocketReceiveMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SocketActivityConnectedStandbyAction(pub i32);
impl SocketActivityConnectedStandbyAction {
    pub const DoNotWake: Self = Self(0i32);
    pub const Wake: Self = Self(1i32);
}
impl windows_core::TypeKind for SocketActivityConnectedStandbyAction {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SocketActivityConnectedStandbyAction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SocketActivityConnectedStandbyAction").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SocketActivityConnectedStandbyAction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketActivityConnectedStandbyAction;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SocketActivityKind(pub i32);
impl SocketActivityKind {
    pub const None: Self = Self(0i32);
    pub const StreamSocketListener: Self = Self(1i32);
    pub const DatagramSocket: Self = Self(2i32);
    pub const StreamSocket: Self = Self(3i32);
}
impl windows_core::TypeKind for SocketActivityKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SocketActivityKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SocketActivityKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SocketActivityKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketActivityKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SocketActivityTriggerReason(pub i32);
impl SocketActivityTriggerReason {
    pub const None: Self = Self(0i32);
    pub const SocketActivity: Self = Self(1i32);
    pub const ConnectionAccepted: Self = Self(2i32);
    pub const KeepAliveTimerExpired: Self = Self(3i32);
    pub const SocketClosed: Self = Self(4i32);
}
impl windows_core::TypeKind for SocketActivityTriggerReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SocketActivityTriggerReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SocketActivityTriggerReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SocketActivityTriggerReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketActivityTriggerReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SocketErrorStatus(pub i32);
impl SocketErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const OperationAborted: Self = Self(1i32);
    pub const HttpInvalidServerResponse: Self = Self(2i32);
    pub const ConnectionTimedOut: Self = Self(3i32);
    pub const AddressFamilyNotSupported: Self = Self(4i32);
    pub const SocketTypeNotSupported: Self = Self(5i32);
    pub const HostNotFound: Self = Self(6i32);
    pub const NoDataRecordOfRequestedType: Self = Self(7i32);
    pub const NonAuthoritativeHostNotFound: Self = Self(8i32);
    pub const ClassTypeNotFound: Self = Self(9i32);
    pub const AddressAlreadyInUse: Self = Self(10i32);
    pub const CannotAssignRequestedAddress: Self = Self(11i32);
    pub const ConnectionRefused: Self = Self(12i32);
    pub const NetworkIsUnreachable: Self = Self(13i32);
    pub const UnreachableHost: Self = Self(14i32);
    pub const NetworkIsDown: Self = Self(15i32);
    pub const NetworkDroppedConnectionOnReset: Self = Self(16i32);
    pub const SoftwareCausedConnectionAbort: Self = Self(17i32);
    pub const ConnectionResetByPeer: Self = Self(18i32);
    pub const HostIsDown: Self = Self(19i32);
    pub const NoAddressesFound: Self = Self(20i32);
    pub const TooManyOpenFiles: Self = Self(21i32);
    pub const MessageTooLong: Self = Self(22i32);
    pub const CertificateExpired: Self = Self(23i32);
    pub const CertificateUntrustedRoot: Self = Self(24i32);
    pub const CertificateCommonNameIsIncorrect: Self = Self(25i32);
    pub const CertificateWrongUsage: Self = Self(26i32);
    pub const CertificateRevoked: Self = Self(27i32);
    pub const CertificateNoRevocationCheck: Self = Self(28i32);
    pub const CertificateRevocationServerOffline: Self = Self(29i32);
    pub const CertificateIsInvalid: Self = Self(30i32);
}
impl windows_core::TypeKind for SocketErrorStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SocketErrorStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SocketErrorStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SocketErrorStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketErrorStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SocketMessageType(pub i32);
impl SocketMessageType {
    pub const Binary: Self = Self(0i32);
    pub const Utf8: Self = Self(1i32);
}
impl windows_core::TypeKind for SocketMessageType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SocketMessageType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SocketMessageType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SocketMessageType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketMessageType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SocketProtectionLevel(pub i32);
impl SocketProtectionLevel {
    pub const PlainSocket: Self = Self(0i32);
    pub const Ssl: Self = Self(1i32);
    pub const SslAllowNullEncryption: Self = Self(2i32);
    pub const BluetoothEncryptionAllowNullAuthentication: Self = Self(3i32);
    pub const BluetoothEncryptionWithAuthentication: Self = Self(4i32);
    pub const Ssl3AllowWeakEncryption: Self = Self(5i32);
    pub const Tls10: Self = Self(6i32);
    pub const Tls11: Self = Self(7i32);
    pub const Tls12: Self = Self(8i32);
    pub const Unspecified: Self = Self(9i32);
    pub const Tls13: Self = Self(10i32);
}
impl windows_core::TypeKind for SocketProtectionLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SocketProtectionLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SocketProtectionLevel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SocketProtectionLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketProtectionLevel;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SocketQualityOfService(pub i32);
impl SocketQualityOfService {
    pub const Normal: Self = Self(0i32);
    pub const LowLatency: Self = Self(1i32);
}
impl windows_core::TypeKind for SocketQualityOfService {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SocketQualityOfService {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SocketQualityOfService").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SocketQualityOfService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketQualityOfService;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SocketSslErrorSeverity(pub i32);
impl SocketSslErrorSeverity {
    pub const None: Self = Self(0i32);
    pub const Ignorable: Self = Self(1i32);
    pub const Fatal: Self = Self(2i32);
}
impl windows_core::TypeKind for SocketSslErrorSeverity {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SocketSslErrorSeverity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SocketSslErrorSeverity").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for SocketSslErrorSeverity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Sockets.SocketSslErrorSeverity;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BandwidthStatistics {
    pub OutboundBitsPerSecond: u64,
    pub InboundBitsPerSecond: u64,
    pub OutboundBitsPerSecondInstability: u64,
    pub InboundBitsPerSecondInstability: u64,
    pub OutboundBandwidthPeaked: bool,
    pub InboundBandwidthPeaked: bool,
}
impl windows_core::TypeKind for BandwidthStatistics {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for BandwidthStatistics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Networking.Sockets.BandwidthStatistics;u8;u8;u8;u8;b1;b1)");
}
impl Default for BandwidthStatistics {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoundTripTimeStatistics {
    pub Variance: u32,
    pub Max: u32,
    pub Min: u32,
    pub Sum: u32,
}
impl windows_core::TypeKind for RoundTripTimeStatistics {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for RoundTripTimeStatistics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Networking.Sockets.RoundTripTimeStatistics;u4;u4;u4;u4)");
}
impl Default for RoundTripTimeStatistics {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
