windows_core::imp::define_interface!(IXboxLiveDeviceAddress, IXboxLiveDeviceAddress_Vtbl, 0xf5bbd279_3c86_4b57_a31a_b9462408fd01);
impl windows_core::RuntimeType for IXboxLiveDeviceAddress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXboxLiveDeviceAddress_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SnapshotChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSnapshotChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub GetSnapshotAsBase64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetSnapshotAsBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetSnapshotAsBuffer: usize,
    pub GetSnapshotAsBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub NetworkAccessKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XboxLiveNetworkAccessKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXboxLiveDeviceAddressStatics, IXboxLiveDeviceAddressStatics_Vtbl, 0x5954a819_4a79_4931_827c_7f503e963263);
impl windows_core::RuntimeType for IXboxLiveDeviceAddressStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXboxLiveDeviceAddressStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromSnapshotBase64: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromSnapshotBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromSnapshotBuffer: usize,
    pub CreateFromSnapshotBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxSnapshotBytesSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXboxLiveEndpointPair, IXboxLiveEndpointPair_Vtbl, 0x1e9a839b_813e_44e0_b87f_c87a093475e4);
impl windows_core::RuntimeType for IXboxLiveEndpointPair {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXboxLiveEndpointPair_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRemoteSocketAddressBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    pub GetLocalSocketAddressBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XboxLiveEndpointPairState) -> windows_core::HRESULT,
    pub Template: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteDeviceAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteHostName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemotePort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LocalHostName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXboxLiveEndpointPairCreationResult, IXboxLiveEndpointPairCreationResult_Vtbl, 0xd9a8bb95_2aab_4d1e_9794_33ecc0dcf0fe);
impl windows_core::RuntimeType for IXboxLiveEndpointPairCreationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXboxLiveEndpointPairCreationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DeviceAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XboxLiveEndpointPairCreationStatus) -> windows_core::HRESULT,
    pub IsExistingPathEvaluation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub EndpointPair: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXboxLiveEndpointPairStateChangedEventArgs, IXboxLiveEndpointPairStateChangedEventArgs_Vtbl, 0x592e3b55_de08_44e7_ac3b_b9b9a169583a);
impl windows_core::RuntimeType for IXboxLiveEndpointPairStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXboxLiveEndpointPairStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OldState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XboxLiveEndpointPairState) -> windows_core::HRESULT,
    pub NewState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XboxLiveEndpointPairState) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXboxLiveEndpointPairStatics, IXboxLiveEndpointPairStatics_Vtbl, 0x64316b30_217a_4243_8ee1_6729281d27db);
impl windows_core::RuntimeType for IXboxLiveEndpointPairStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXboxLiveEndpointPairStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FindEndpointPairBySocketAddressBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindEndpointPairByHostNamesAndPorts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXboxLiveEndpointPairTemplate, IXboxLiveEndpointPairTemplate_Vtbl, 0x6b286ecf_3457_40ce_b9a1_c0cfe0213ea7);
impl windows_core::RuntimeType for IXboxLiveEndpointPairTemplate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXboxLiveEndpointPairTemplate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InboundEndpointPairCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveInboundEndpointPairCreated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CreateEndpointPairDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEndpointPairWithBehaviorsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, XboxLiveEndpointPairCreationBehaviors, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEndpointPairForPortsDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEndpointPairForPortsWithBehaviorsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, XboxLiveEndpointPairCreationBehaviors, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SocketKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XboxLiveSocketKind) -> windows_core::HRESULT,
    pub InitiatorBoundPortRangeLower: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub InitiatorBoundPortRangeUpper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub AcceptorBoundPortRangeLower: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub AcceptorBoundPortRangeUpper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EndpointPairs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EndpointPairs: usize,
}
windows_core::imp::define_interface!(IXboxLiveEndpointPairTemplateStatics, IXboxLiveEndpointPairTemplateStatics_Vtbl, 0x1e13137b_737b_4a23_bc64_0870f75655ba);
impl windows_core::RuntimeType for IXboxLiveEndpointPairTemplateStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXboxLiveEndpointPairTemplateStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetTemplateByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Templates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Templates: usize,
}
windows_core::imp::define_interface!(IXboxLiveInboundEndpointPairCreatedEventArgs, IXboxLiveInboundEndpointPairCreatedEventArgs_Vtbl, 0xdc183b62_22ba_48d2_80de_c23968bd198b);
impl windows_core::RuntimeType for IXboxLiveInboundEndpointPairCreatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXboxLiveInboundEndpointPairCreatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EndpointPair: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXboxLiveQualityOfServiceMeasurement, IXboxLiveQualityOfServiceMeasurement_Vtbl, 0x4d682bce_a5d6_47e6_a236_cfde5fbdf2ed);
impl windows_core::RuntimeType for IXboxLiveQualityOfServiceMeasurement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXboxLiveQualityOfServiceMeasurement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MeasureAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMetricResultsForDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMetricResultsForDevice: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMetricResultsForMetric: unsafe extern "system" fn(*mut core::ffi::c_void, XboxLiveQualityOfServiceMetric, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMetricResultsForMetric: usize,
    pub GetMetricResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, XboxLiveQualityOfServiceMetric, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrivatePayloadResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Metrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Metrics: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DeviceAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeviceAddresses: usize,
    pub ShouldRequestPrivatePayloads: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShouldRequestPrivatePayloads: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub TimeoutInMilliseconds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTimeoutInMilliseconds: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub NumberOfProbesToAttempt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetNumberOfProbesToAttempt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub NumberOfResultsPending: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MetricResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MetricResults: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PrivatePayloadResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PrivatePayloadResults: usize,
}
windows_core::imp::define_interface!(IXboxLiveQualityOfServiceMeasurementStatics, IXboxLiveQualityOfServiceMeasurementStatics_Vtbl, 0x6e352dca_23cf_440a_b077_5e30857a8234);
impl windows_core::RuntimeType for IXboxLiveQualityOfServiceMeasurementStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXboxLiveQualityOfServiceMeasurementStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PublishPrivatePayloadBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub ClearPrivatePayload: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxSimultaneousProbeConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxSimultaneousProbeConnections: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsSystemOutboundBandwidthConstrained: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsSystemOutboundBandwidthConstrained: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsSystemInboundBandwidthConstrained: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsSystemInboundBandwidthConstrained: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PublishedPrivatePayload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PublishedPrivatePayload: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetPublishedPrivatePayload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPublishedPrivatePayload: usize,
    pub MaxPrivatePayloadSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXboxLiveQualityOfServiceMetricResult, IXboxLiveQualityOfServiceMetricResult_Vtbl, 0xaeec53d1_3561_4782_b0cf_d3ae29d9fa87);
impl windows_core::RuntimeType for IXboxLiveQualityOfServiceMetricResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXboxLiveQualityOfServiceMetricResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XboxLiveQualityOfServiceMeasurementStatus) -> windows_core::HRESULT,
    pub DeviceAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Metric: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XboxLiveQualityOfServiceMetric) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXboxLiveQualityOfServicePrivatePayloadResult, IXboxLiveQualityOfServicePrivatePayloadResult_Vtbl, 0x5a6302ae_6f38_41c0_9fcc_ea6cb978cafc);
impl windows_core::RuntimeType for IXboxLiveQualityOfServicePrivatePayloadResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXboxLiveQualityOfServicePrivatePayloadResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XboxLiveQualityOfServiceMeasurementStatus) -> windows_core::HRESULT,
    pub DeviceAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XboxLiveDeviceAddress(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XboxLiveDeviceAddress, windows_core::IUnknown, windows_core::IInspectable);
impl XboxLiveDeviceAddress {
    pub fn SnapshotChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<XboxLiveDeviceAddress, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SnapshotChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSnapshotChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSnapshotChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetSnapshotAsBase64(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSnapshotAsBase64)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetSnapshotAsBuffer(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSnapshotAsBuffer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSnapshotAsBytes(&self, buffer: &mut [u8], byteswritten: &mut u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetSnapshotAsBytes)(windows_core::Interface::as_raw(this), buffer.len().try_into().unwrap(), buffer.as_mut_ptr(), byteswritten).ok() }
    }
    pub fn Compare<P0>(&self, otherdeviceaddress: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<XboxLiveDeviceAddress>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compare)(windows_core::Interface::as_raw(this), otherdeviceaddress.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn IsValid(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsValid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsLocal(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLocal)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NetworkAccessKind(&self) -> windows_core::Result<XboxLiveNetworkAccessKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkAccessKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateFromSnapshotBase64(base64: &windows_core::HSTRING) -> windows_core::Result<XboxLiveDeviceAddress> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromSnapshotBase64)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(base64), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromSnapshotBuffer<P0>(buffer: P0) -> windows_core::Result<XboxLiveDeviceAddress>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromSnapshotBuffer)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromSnapshotBytes(buffer: &[u8]) -> windows_core::Result<XboxLiveDeviceAddress> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromSnapshotBytes)(windows_core::Interface::as_raw(this), buffer.len().try_into().unwrap(), buffer.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetLocal() -> windows_core::Result<XboxLiveDeviceAddress> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetLocal)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MaxSnapshotBytesSize() -> windows_core::Result<u32> {
        Self::IXboxLiveDeviceAddressStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxSnapshotBytesSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IXboxLiveDeviceAddressStatics<R, F: FnOnce(&IXboxLiveDeviceAddressStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XboxLiveDeviceAddress, IXboxLiveDeviceAddressStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XboxLiveDeviceAddress {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXboxLiveDeviceAddress>();
}
unsafe impl windows_core::Interface for XboxLiveDeviceAddress {
    type Vtable = IXboxLiveDeviceAddress_Vtbl;
    const IID: windows_core::GUID = <IXboxLiveDeviceAddress as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XboxLiveDeviceAddress {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveDeviceAddress";
}
unsafe impl Send for XboxLiveDeviceAddress {}
unsafe impl Sync for XboxLiveDeviceAddress {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XboxLiveEndpointPair(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XboxLiveEndpointPair, windows_core::IUnknown, windows_core::IInspectable);
impl XboxLiveEndpointPair {
    pub fn StateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<XboxLiveEndpointPair, XboxLiveEndpointPairStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DeleteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetRemoteSocketAddressBytes(&self, socketaddress: &mut [u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetRemoteSocketAddressBytes)(windows_core::Interface::as_raw(this), socketaddress.len().try_into().unwrap(), socketaddress.as_mut_ptr()).ok() }
    }
    pub fn GetLocalSocketAddressBytes(&self, socketaddress: &mut [u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetLocalSocketAddressBytes)(windows_core::Interface::as_raw(this), socketaddress.len().try_into().unwrap(), socketaddress.as_mut_ptr()).ok() }
    }
    pub fn State(&self) -> windows_core::Result<XboxLiveEndpointPairState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Template(&self) -> windows_core::Result<XboxLiveEndpointPairTemplate> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Template)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoteDeviceAddress(&self) -> windows_core::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteDeviceAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoteHostName(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteHostName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemotePort(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemotePort)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalHostName(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalHostName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalPort(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalPort)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FindEndpointPairBySocketAddressBytes(localsocketaddress: &[u8], remotesocketaddress: &[u8]) -> windows_core::Result<XboxLiveEndpointPair> {
        Self::IXboxLiveEndpointPairStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindEndpointPairBySocketAddressBytes)(windows_core::Interface::as_raw(this), localsocketaddress.len().try_into().unwrap(), localsocketaddress.as_ptr(), remotesocketaddress.len().try_into().unwrap(), remotesocketaddress.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FindEndpointPairByHostNamesAndPorts<P0, P1>(localhostname: P0, localport: &windows_core::HSTRING, remotehostname: P1, remoteport: &windows_core::HSTRING) -> windows_core::Result<XboxLiveEndpointPair>
    where
        P0: windows_core::Param<super::HostName>,
        P1: windows_core::Param<super::HostName>,
    {
        Self::IXboxLiveEndpointPairStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindEndpointPairByHostNamesAndPorts)(windows_core::Interface::as_raw(this), localhostname.param().abi(), core::mem::transmute_copy(localport), remotehostname.param().abi(), core::mem::transmute_copy(remoteport), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IXboxLiveEndpointPairStatics<R, F: FnOnce(&IXboxLiveEndpointPairStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XboxLiveEndpointPair, IXboxLiveEndpointPairStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XboxLiveEndpointPair {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXboxLiveEndpointPair>();
}
unsafe impl windows_core::Interface for XboxLiveEndpointPair {
    type Vtable = IXboxLiveEndpointPair_Vtbl;
    const IID: windows_core::GUID = <IXboxLiveEndpointPair as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XboxLiveEndpointPair {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPair";
}
unsafe impl Send for XboxLiveEndpointPair {}
unsafe impl Sync for XboxLiveEndpointPair {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XboxLiveEndpointPairCreationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XboxLiveEndpointPairCreationResult, windows_core::IUnknown, windows_core::IInspectable);
impl XboxLiveEndpointPairCreationResult {
    pub fn DeviceAddress(&self) -> windows_core::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<XboxLiveEndpointPairCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsExistingPathEvaluation(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsExistingPathEvaluation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EndpointPair(&self) -> windows_core::Result<XboxLiveEndpointPair> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndpointPair)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for XboxLiveEndpointPairCreationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXboxLiveEndpointPairCreationResult>();
}
unsafe impl windows_core::Interface for XboxLiveEndpointPairCreationResult {
    type Vtable = IXboxLiveEndpointPairCreationResult_Vtbl;
    const IID: windows_core::GUID = <IXboxLiveEndpointPairCreationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XboxLiveEndpointPairCreationResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPairCreationResult";
}
unsafe impl Send for XboxLiveEndpointPairCreationResult {}
unsafe impl Sync for XboxLiveEndpointPairCreationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XboxLiveEndpointPairStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XboxLiveEndpointPairStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl XboxLiveEndpointPairStateChangedEventArgs {
    pub fn OldState(&self) -> windows_core::Result<XboxLiveEndpointPairState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OldState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NewState(&self) -> windows_core::Result<XboxLiveEndpointPairState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for XboxLiveEndpointPairStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXboxLiveEndpointPairStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for XboxLiveEndpointPairStateChangedEventArgs {
    type Vtable = IXboxLiveEndpointPairStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IXboxLiveEndpointPairStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XboxLiveEndpointPairStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPairStateChangedEventArgs";
}
unsafe impl Send for XboxLiveEndpointPairStateChangedEventArgs {}
unsafe impl Sync for XboxLiveEndpointPairStateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XboxLiveEndpointPairTemplate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XboxLiveEndpointPairTemplate, windows_core::IUnknown, windows_core::IInspectable);
impl XboxLiveEndpointPairTemplate {
    pub fn InboundEndpointPairCreated<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<XboxLiveEndpointPairTemplate, XboxLiveInboundEndpointPairCreatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InboundEndpointPairCreated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveInboundEndpointPairCreated(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveInboundEndpointPairCreated)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateEndpointPairDefaultAsync<P0>(&self, deviceaddress: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>
    where
        P0: windows_core::Param<XboxLiveDeviceAddress>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateEndpointPairDefaultAsync)(windows_core::Interface::as_raw(this), deviceaddress.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateEndpointPairWithBehaviorsAsync<P0>(&self, deviceaddress: P0, behaviors: XboxLiveEndpointPairCreationBehaviors) -> windows_core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>
    where
        P0: windows_core::Param<XboxLiveDeviceAddress>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateEndpointPairWithBehaviorsAsync)(windows_core::Interface::as_raw(this), deviceaddress.param().abi(), behaviors, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateEndpointPairForPortsDefaultAsync<P0>(&self, deviceaddress: P0, initiatorport: &windows_core::HSTRING, acceptorport: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>
    where
        P0: windows_core::Param<XboxLiveDeviceAddress>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateEndpointPairForPortsDefaultAsync)(windows_core::Interface::as_raw(this), deviceaddress.param().abi(), core::mem::transmute_copy(initiatorport), core::mem::transmute_copy(acceptorport), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateEndpointPairForPortsWithBehaviorsAsync<P0>(&self, deviceaddress: P0, initiatorport: &windows_core::HSTRING, acceptorport: &windows_core::HSTRING, behaviors: XboxLiveEndpointPairCreationBehaviors) -> windows_core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>
    where
        P0: windows_core::Param<XboxLiveDeviceAddress>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateEndpointPairForPortsWithBehaviorsAsync)(windows_core::Interface::as_raw(this), deviceaddress.param().abi(), core::mem::transmute_copy(initiatorport), core::mem::transmute_copy(acceptorport), behaviors, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SocketKind(&self) -> windows_core::Result<XboxLiveSocketKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SocketKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InitiatorBoundPortRangeLower(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InitiatorBoundPortRangeLower)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InitiatorBoundPortRangeUpper(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InitiatorBoundPortRangeUpper)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AcceptorBoundPortRangeLower(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcceptorBoundPortRangeLower)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AcceptorBoundPortRangeUpper(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcceptorBoundPortRangeUpper)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EndpointPairs(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPair>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndpointPairs)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetTemplateByName(name: &windows_core::HSTRING) -> windows_core::Result<XboxLiveEndpointPairTemplate> {
        Self::IXboxLiveEndpointPairTemplateStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateByName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Templates() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPairTemplate>> {
        Self::IXboxLiveEndpointPairTemplateStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Templates)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IXboxLiveEndpointPairTemplateStatics<R, F: FnOnce(&IXboxLiveEndpointPairTemplateStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XboxLiveEndpointPairTemplate, IXboxLiveEndpointPairTemplateStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XboxLiveEndpointPairTemplate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXboxLiveEndpointPairTemplate>();
}
unsafe impl windows_core::Interface for XboxLiveEndpointPairTemplate {
    type Vtable = IXboxLiveEndpointPairTemplate_Vtbl;
    const IID: windows_core::GUID = <IXboxLiveEndpointPairTemplate as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XboxLiveEndpointPairTemplate {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveEndpointPairTemplate";
}
unsafe impl Send for XboxLiveEndpointPairTemplate {}
unsafe impl Sync for XboxLiveEndpointPairTemplate {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XboxLiveInboundEndpointPairCreatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XboxLiveInboundEndpointPairCreatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl XboxLiveInboundEndpointPairCreatedEventArgs {
    pub fn EndpointPair(&self) -> windows_core::Result<XboxLiveEndpointPair> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndpointPair)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for XboxLiveInboundEndpointPairCreatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXboxLiveInboundEndpointPairCreatedEventArgs>();
}
unsafe impl windows_core::Interface for XboxLiveInboundEndpointPairCreatedEventArgs {
    type Vtable = IXboxLiveInboundEndpointPairCreatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IXboxLiveInboundEndpointPairCreatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XboxLiveInboundEndpointPairCreatedEventArgs {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveInboundEndpointPairCreatedEventArgs";
}
unsafe impl Send for XboxLiveInboundEndpointPairCreatedEventArgs {}
unsafe impl Sync for XboxLiveInboundEndpointPairCreatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XboxLiveQualityOfServiceMeasurement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XboxLiveQualityOfServiceMeasurement, windows_core::IUnknown, windows_core::IInspectable);
impl XboxLiveQualityOfServiceMeasurement {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XboxLiveQualityOfServiceMeasurement, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MeasureAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMetricResultsForDevice<P0>(&self, deviceaddress: P0) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>
    where
        P0: windows_core::Param<XboxLiveDeviceAddress>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMetricResultsForDevice)(windows_core::Interface::as_raw(this), deviceaddress.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMetricResultsForMetric(&self, metric: XboxLiveQualityOfServiceMetric) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMetricResultsForMetric)(windows_core::Interface::as_raw(this), metric, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetMetricResult<P0>(&self, deviceaddress: P0, metric: XboxLiveQualityOfServiceMetric) -> windows_core::Result<XboxLiveQualityOfServiceMetricResult>
    where
        P0: windows_core::Param<XboxLiveDeviceAddress>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMetricResult)(windows_core::Interface::as_raw(this), deviceaddress.param().abi(), metric, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPrivatePayloadResult<P0>(&self, deviceaddress: P0) -> windows_core::Result<XboxLiveQualityOfServicePrivatePayloadResult>
    where
        P0: windows_core::Param<XboxLiveDeviceAddress>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPrivatePayloadResult)(windows_core::Interface::as_raw(this), deviceaddress.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Metrics(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<XboxLiveQualityOfServiceMetric>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Metrics)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeviceAddresses(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<XboxLiveDeviceAddress>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceAddresses)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShouldRequestPrivatePayloads(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldRequestPrivatePayloads)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetShouldRequestPrivatePayloads(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetShouldRequestPrivatePayloads)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TimeoutInMilliseconds(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeoutInMilliseconds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTimeoutInMilliseconds(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTimeoutInMilliseconds)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumberOfProbesToAttempt(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberOfProbesToAttempt)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNumberOfProbesToAttempt(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNumberOfProbesToAttempt)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NumberOfResultsPending(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberOfResultsPending)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MetricResults(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MetricResults)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PrivatePayloadResults(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServicePrivatePayloadResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrivatePayloadResults)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PublishPrivatePayloadBytes(payload: &[u8]) -> windows_core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (windows_core::Interface::vtable(this).PublishPrivatePayloadBytes)(windows_core::Interface::as_raw(this), payload.len().try_into().unwrap(), payload.as_ptr()).ok() })
    }
    pub fn ClearPrivatePayload() -> windows_core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (windows_core::Interface::vtable(this).ClearPrivatePayload)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn MaxSimultaneousProbeConnections() -> windows_core::Result<u32> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxSimultaneousProbeConnections)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetMaxSimultaneousProbeConnections(value: u32) -> windows_core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetMaxSimultaneousProbeConnections)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn IsSystemOutboundBandwidthConstrained() -> windows_core::Result<bool> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSystemOutboundBandwidthConstrained)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetIsSystemOutboundBandwidthConstrained(value: bool) -> windows_core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetIsSystemOutboundBandwidthConstrained)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn IsSystemInboundBandwidthConstrained() -> windows_core::Result<bool> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSystemInboundBandwidthConstrained)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetIsSystemInboundBandwidthConstrained(value: bool) -> windows_core::Result<()> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetIsSystemInboundBandwidthConstrained)(windows_core::Interface::as_raw(this), value).ok() })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PublishedPrivatePayload() -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PublishedPrivatePayload)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPublishedPrivatePayload<P0>(value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetPublishedPrivatePayload)(windows_core::Interface::as_raw(this), value.param().abi()).ok() })
    }
    pub fn MaxPrivatePayloadSize() -> windows_core::Result<u32> {
        Self::IXboxLiveQualityOfServiceMeasurementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxPrivatePayloadSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IXboxLiveQualityOfServiceMeasurementStatics<R, F: FnOnce(&IXboxLiveQualityOfServiceMeasurementStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XboxLiveQualityOfServiceMeasurement, IXboxLiveQualityOfServiceMeasurementStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XboxLiveQualityOfServiceMeasurement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXboxLiveQualityOfServiceMeasurement>();
}
unsafe impl windows_core::Interface for XboxLiveQualityOfServiceMeasurement {
    type Vtable = IXboxLiveQualityOfServiceMeasurement_Vtbl;
    const IID: windows_core::GUID = <IXboxLiveQualityOfServiceMeasurement as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XboxLiveQualityOfServiceMeasurement {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurement";
}
unsafe impl Send for XboxLiveQualityOfServiceMeasurement {}
unsafe impl Sync for XboxLiveQualityOfServiceMeasurement {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XboxLiveQualityOfServiceMetricResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XboxLiveQualityOfServiceMetricResult, windows_core::IUnknown, windows_core::IInspectable);
impl XboxLiveQualityOfServiceMetricResult {
    pub fn Status(&self) -> windows_core::Result<XboxLiveQualityOfServiceMeasurementStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceAddress(&self) -> windows_core::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Metric(&self) -> windows_core::Result<XboxLiveQualityOfServiceMetric> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Metric)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Value(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for XboxLiveQualityOfServiceMetricResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXboxLiveQualityOfServiceMetricResult>();
}
unsafe impl windows_core::Interface for XboxLiveQualityOfServiceMetricResult {
    type Vtable = IXboxLiveQualityOfServiceMetricResult_Vtbl;
    const IID: windows_core::GUID = <IXboxLiveQualityOfServiceMetricResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XboxLiveQualityOfServiceMetricResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetricResult";
}
unsafe impl Send for XboxLiveQualityOfServiceMetricResult {}
unsafe impl Sync for XboxLiveQualityOfServiceMetricResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XboxLiveQualityOfServicePrivatePayloadResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XboxLiveQualityOfServicePrivatePayloadResult, windows_core::IUnknown, windows_core::IInspectable);
impl XboxLiveQualityOfServicePrivatePayloadResult {
    pub fn Status(&self) -> windows_core::Result<XboxLiveQualityOfServiceMeasurementStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceAddress(&self) -> windows_core::Result<XboxLiveDeviceAddress> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceAddress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for XboxLiveQualityOfServicePrivatePayloadResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXboxLiveQualityOfServicePrivatePayloadResult>();
}
unsafe impl windows_core::Interface for XboxLiveQualityOfServicePrivatePayloadResult {
    type Vtable = IXboxLiveQualityOfServicePrivatePayloadResult_Vtbl;
    const IID: windows_core::GUID = <IXboxLiveQualityOfServicePrivatePayloadResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XboxLiveQualityOfServicePrivatePayloadResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.XboxLiveQualityOfServicePrivatePayloadResult";
}
unsafe impl Send for XboxLiveQualityOfServicePrivatePayloadResult {}
unsafe impl Sync for XboxLiveQualityOfServicePrivatePayloadResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XboxLiveEndpointPairCreationBehaviors(pub u32);
impl XboxLiveEndpointPairCreationBehaviors {
    pub const None: Self = Self(0u32);
    pub const ReevaluatePath: Self = Self(1u32);
}
impl windows_core::TypeKind for XboxLiveEndpointPairCreationBehaviors {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XboxLiveEndpointPairCreationBehaviors {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPairCreationBehaviors").field(&self.0).finish()
    }
}
impl XboxLiveEndpointPairCreationBehaviors {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for XboxLiveEndpointPairCreationBehaviors {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for XboxLiveEndpointPairCreationBehaviors {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for XboxLiveEndpointPairCreationBehaviors {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for XboxLiveEndpointPairCreationBehaviors {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for XboxLiveEndpointPairCreationBehaviors {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for XboxLiveEndpointPairCreationBehaviors {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveEndpointPairCreationBehaviors;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XboxLiveEndpointPairCreationStatus(pub i32);
impl XboxLiveEndpointPairCreationStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoLocalNetworks: Self = Self(1i32);
    pub const NoCompatibleNetworkPaths: Self = Self(2i32);
    pub const LocalSystemNotAuthorized: Self = Self(3i32);
    pub const Canceled: Self = Self(4i32);
    pub const TimedOut: Self = Self(5i32);
    pub const RemoteSystemNotAuthorized: Self = Self(6i32);
    pub const RefusedDueToConfiguration: Self = Self(7i32);
    pub const UnexpectedInternalError: Self = Self(8i32);
}
impl windows_core::TypeKind for XboxLiveEndpointPairCreationStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XboxLiveEndpointPairCreationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPairCreationStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for XboxLiveEndpointPairCreationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveEndpointPairCreationStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XboxLiveEndpointPairState(pub i32);
impl XboxLiveEndpointPairState {
    pub const Invalid: Self = Self(0i32);
    pub const CreatingOutbound: Self = Self(1i32);
    pub const CreatingInbound: Self = Self(2i32);
    pub const Ready: Self = Self(3i32);
    pub const DeletingLocally: Self = Self(4i32);
    pub const RemoteEndpointTerminating: Self = Self(5i32);
    pub const Deleted: Self = Self(6i32);
}
impl windows_core::TypeKind for XboxLiveEndpointPairState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XboxLiveEndpointPairState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XboxLiveEndpointPairState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for XboxLiveEndpointPairState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveEndpointPairState;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XboxLiveNetworkAccessKind(pub i32);
impl XboxLiveNetworkAccessKind {
    pub const Open: Self = Self(0i32);
    pub const Moderate: Self = Self(1i32);
    pub const Strict: Self = Self(2i32);
}
impl windows_core::TypeKind for XboxLiveNetworkAccessKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XboxLiveNetworkAccessKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XboxLiveNetworkAccessKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for XboxLiveNetworkAccessKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveNetworkAccessKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XboxLiveQualityOfServiceMeasurementStatus(pub i32);
impl XboxLiveQualityOfServiceMeasurementStatus {
    pub const NotStarted: Self = Self(0i32);
    pub const InProgress: Self = Self(1i32);
    pub const InProgressWithProvisionalResults: Self = Self(2i32);
    pub const Succeeded: Self = Self(3i32);
    pub const NoLocalNetworks: Self = Self(4i32);
    pub const NoCompatibleNetworkPaths: Self = Self(5i32);
    pub const LocalSystemNotAuthorized: Self = Self(6i32);
    pub const Canceled: Self = Self(7i32);
    pub const TimedOut: Self = Self(8i32);
    pub const RemoteSystemNotAuthorized: Self = Self(9i32);
    pub const RefusedDueToConfiguration: Self = Self(10i32);
    pub const UnexpectedInternalError: Self = Self(11i32);
}
impl windows_core::TypeKind for XboxLiveQualityOfServiceMeasurementStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XboxLiveQualityOfServiceMeasurementStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XboxLiveQualityOfServiceMeasurementStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for XboxLiveQualityOfServiceMeasurementStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveQualityOfServiceMeasurementStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XboxLiveQualityOfServiceMetric(pub i32);
impl XboxLiveQualityOfServiceMetric {
    pub const AverageLatencyInMilliseconds: Self = Self(0i32);
    pub const MinLatencyInMilliseconds: Self = Self(1i32);
    pub const MaxLatencyInMilliseconds: Self = Self(2i32);
    pub const AverageOutboundBitsPerSecond: Self = Self(3i32);
    pub const MinOutboundBitsPerSecond: Self = Self(4i32);
    pub const MaxOutboundBitsPerSecond: Self = Self(5i32);
    pub const AverageInboundBitsPerSecond: Self = Self(6i32);
    pub const MinInboundBitsPerSecond: Self = Self(7i32);
    pub const MaxInboundBitsPerSecond: Self = Self(8i32);
}
impl windows_core::TypeKind for XboxLiveQualityOfServiceMetric {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XboxLiveQualityOfServiceMetric {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XboxLiveQualityOfServiceMetric").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for XboxLiveQualityOfServiceMetric {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveQualityOfServiceMetric;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XboxLiveSocketKind(pub i32);
impl XboxLiveSocketKind {
    pub const None: Self = Self(0i32);
    pub const Datagram: Self = Self(1i32);
    pub const Stream: Self = Self(2i32);
}
impl windows_core::TypeKind for XboxLiveSocketKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XboxLiveSocketKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XboxLiveSocketKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for XboxLiveSocketKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.XboxLive.XboxLiveSocketKind;i4)");
}
