windows_core::imp::define_interface!(IAttributedNetworkUsage, IAttributedNetworkUsage_Vtbl, 0xf769b039_eca2_45eb_ade1_b0368b756c49);
impl windows_core::RuntimeType for IAttributedNetworkUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAttributedNetworkUsage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BytesSent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub AttributionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AttributionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AttributionThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AttributionThumbnail: usize,
}
windows_core::imp::define_interface!(ICellularApnContext, ICellularApnContext_Vtbl, 0x6fa529f4_effd_4542_9ab2_705bbf94943a);
impl windows_core::RuntimeType for ICellularApnContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICellularApnContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AccessPointName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAccessPointName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsCompressionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCompressionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AuthenticationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CellularApnAuthenticationType) -> windows_core::HRESULT,
    pub SetAuthenticationType: unsafe extern "system" fn(*mut core::ffi::c_void, CellularApnAuthenticationType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICellularApnContext2, ICellularApnContext2_Vtbl, 0x76b0eb1a_ac49_4350_b1e5_dc4763bc69c7);
impl windows_core::RuntimeType for ICellularApnContext2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICellularApnContext2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProfileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetProfileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConnectionCost, IConnectionCost_Vtbl, 0xbad7d829_3416_4b10_a202_bac0b075bdae);
impl windows_core::RuntimeType for IConnectionCost {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConnectionCost_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NetworkCostType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NetworkCostType) -> windows_core::HRESULT,
    pub Roaming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub OverDataLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ApproachingDataLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConnectionCost2, IConnectionCost2_Vtbl, 0x8e113a05_e209_4549_bb25_5e0db691cb05);
impl windows_core::RuntimeType for IConnectionCost2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConnectionCost2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BackgroundDataUsageRestricted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConnectionProfile, IConnectionProfile_Vtbl, 0x71ba143c_598e_49d0_84eb_8febaedcc195);
impl windows_core::RuntimeType for IConnectionProfile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConnectionProfile_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProfileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetNetworkConnectivityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NetworkConnectivityLevel) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNetworkNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNetworkNames: usize,
    pub GetConnectionCost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDataPlanStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NetworkAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub GetLocalUsage: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetLocalUsage: usize,
    #[cfg(feature = "deprecated")]
    pub GetLocalUsagePerRoamingStates: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::DateTime, RoamingStates, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetLocalUsagePerRoamingStates: usize,
    pub NetworkSecuritySettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConnectionProfile2, IConnectionProfile2_Vtbl, 0xe2045145_4c9f_400c_9150_7ec7d6e2888a);
impl windows_core::RuntimeType for IConnectionProfile2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConnectionProfile2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsWwanConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsWlanConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub WwanConnectionProfileDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WlanConnectionProfileDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceProviderGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSignalBars: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDomainConnectivityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DomainConnectivityLevel) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNetworkUsageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::DateTime, DataUsageGranularity, NetworkUsageStates, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNetworkUsageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConnectivityIntervalsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::DateTime, NetworkUsageStates, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConnectivityIntervalsAsync: usize,
}
windows_core::imp::define_interface!(IConnectionProfile3, IConnectionProfile3_Vtbl, 0x578c2528_4cd9_4161_8045_201cfd5b115c);
impl windows_core::RuntimeType for IConnectionProfile3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConnectionProfile3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAttributedNetworkUsageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::DateTime, NetworkUsageStates, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAttributedNetworkUsageAsync: usize,
}
windows_core::imp::define_interface!(IConnectionProfile4, IConnectionProfile4_Vtbl, 0x7a2d42cd_81e0_4ae6_abed_ab9ca13eb714);
impl windows_core::RuntimeType for IConnectionProfile4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConnectionProfile4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetProviderNetworkUsageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, super::super::Foundation::DateTime, NetworkUsageStates, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetProviderNetworkUsageAsync: usize,
}
windows_core::imp::define_interface!(IConnectionProfile5, IConnectionProfile5_Vtbl, 0x85361ec7_9c73_4be0_8f14_578eec71ee0e);
impl windows_core::RuntimeType for IConnectionProfile5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConnectionProfile5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanDelete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TryDeleteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConnectionProfile6, IConnectionProfile6_Vtbl, 0xdc27dfe2_7a6f_5d0e_9589_2fe2e5b6f9aa);
impl windows_core::RuntimeType for IConnectionProfile6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConnectionProfile6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsDomainAuthenticatedBy: unsafe extern "system" fn(*mut core::ffi::c_void, DomainAuthenticationKind, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConnectionProfileFilter, IConnectionProfileFilter_Vtbl, 0x204c7cc8_bd2d_4e8d_a4b3_455ec337388a);
impl windows_core::RuntimeType for IConnectionProfileFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConnectionProfileFilter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetIsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsWwanConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsWwanConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsWlanConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsWlanConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetNetworkCostType: unsafe extern "system" fn(*mut core::ffi::c_void, NetworkCostType) -> windows_core::HRESULT,
    pub NetworkCostType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NetworkCostType) -> windows_core::HRESULT,
    pub SetServiceProviderGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServiceProviderGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConnectionProfileFilter2, IConnectionProfileFilter2_Vtbl, 0xcd068ee1_c3fc_4fad_9ddc_593faa4b7885);
impl windows_core::RuntimeType for IConnectionProfileFilter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConnectionProfileFilter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetIsRoaming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsRoaming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIsOverDataLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOverDataLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIsBackgroundDataUsageRestricted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsBackgroundDataUsageRestricted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub RawData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RawData: usize,
}
windows_core::imp::define_interface!(IConnectionProfileFilter3, IConnectionProfileFilter3_Vtbl, 0x0aaa09c0_5014_447c_8809_aee4cb0af94a);
impl windows_core::RuntimeType for IConnectionProfileFilter3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConnectionProfileFilter3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetPurposeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PurposeGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConnectionSession, IConnectionSession_Vtbl, 0xff905d4c_f83b_41b0_8a0c_1462d9c56b73);
impl windows_core::RuntimeType for IConnectionSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConnectionSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConnectivityInterval, IConnectivityInterval_Vtbl, 0x4faa3fff_6746_4824_a964_eed8e87f8709);
impl windows_core::RuntimeType for IConnectivityInterval {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConnectivityInterval_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub ConnectionDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConnectivityManagerStatics, IConnectivityManagerStatics_Vtbl, 0x5120d4b1_4fb1_48b0_afc9_42e0092a8164);
impl windows_core::RuntimeType for IConnectivityManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IConnectivityManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AcquireConnectionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddHttpRoutePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveHttpRoutePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataPlanStatus, IDataPlanStatus_Vtbl, 0x977a8b8c_3885_40f3_8851_42cd2bd568bb);
impl windows_core::RuntimeType for IDataPlanStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDataPlanStatus_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DataPlanUsage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DataLimitInMegabytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InboundBitsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutboundBitsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NextBillingCycle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxTransferSizeInMegabytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataPlanUsage, IDataPlanUsage_Vtbl, 0xb921492d_3b44_47ff_b361_be59e69ed1b0);
impl windows_core::RuntimeType for IDataPlanUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDataPlanUsage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MegabytesUsed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub LastSyncTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IDataUsage, IDataUsage_Vtbl, 0xc1431dd3_b146_4d39_b959_0c69b096c512);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IDataUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IDataUsage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub BytesSent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BytesSent: usize,
    #[cfg(feature = "deprecated")]
    pub BytesReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BytesReceived: usize,
}
windows_core::imp::define_interface!(IIPInformation, IIPInformation_Vtbl, 0xd85145e0_138f_47d7_9b3a_36bb488cef33);
impl windows_core::RuntimeType for IIPInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IIPInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NetworkAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrefixLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILanIdentifier, ILanIdentifier_Vtbl, 0x48aa53aa_1108_4546_a6cb_9a74da4b7ba0);
impl windows_core::RuntimeType for ILanIdentifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILanIdentifier_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InfrastructureId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PortId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NetworkAdapterId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILanIdentifierData, ILanIdentifierData_Vtbl, 0xa74e83c3_d639_45be_a36a_c4e4aeaf6d9b);
impl windows_core::RuntimeType for ILanIdentifierData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILanIdentifierData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Value: usize,
}
windows_core::imp::define_interface!(INetworkAdapter, INetworkAdapter_Vtbl, 0x3b542e03_5388_496c_a8a3_affd39aec2e6);
impl windows_core::RuntimeType for INetworkAdapter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkAdapter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OutboundMaxBitsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub InboundMaxBitsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub IanaInterfaceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub NetworkItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NetworkAdapterId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetConnectedProfileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkInformationStatics, INetworkInformationStatics_Vtbl, 0x5074f851_950d_4165_9c15_365619481eea);
impl windows_core::RuntimeType for INetworkInformationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkInformationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConnectionProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConnectionProfiles: usize,
    pub GetInternetConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetLanIdentifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetLanIdentifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetHostNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetHostNames: usize,
    pub GetProxyConfigurationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSortedEndpointPairs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::HostNameSortOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSortedEndpointPairs: usize,
    pub NetworkStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveNetworkStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkInformationStatics2, INetworkInformationStatics2_Vtbl, 0x459ced14_2832_49b6_ba6e_e265f04786a8);
impl windows_core::RuntimeType for INetworkInformationStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkInformationStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindConnectionProfilesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindConnectionProfilesAsync: usize,
}
windows_core::imp::define_interface!(INetworkItem, INetworkItem_Vtbl, 0x01bc4d39_f5e0_4567_a28c_42080c831b2b);
impl windows_core::RuntimeType for INetworkItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NetworkId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetNetworkTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NetworkTypes) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkSecuritySettings, INetworkSecuritySettings_Vtbl, 0x7ca07e8d_917b_4b5f_b84d_28f7a5ac5402);
impl windows_core::RuntimeType for INetworkSecuritySettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkSecuritySettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NetworkAuthenticationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NetworkAuthenticationType) -> windows_core::HRESULT,
    pub NetworkEncryptionType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NetworkEncryptionType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkStateChangeEventDetails, INetworkStateChangeEventDetails_Vtbl, 0x1f0cf333_d7a6_44dd_a4e9_687c476b903d);
impl windows_core::RuntimeType for INetworkStateChangeEventDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkStateChangeEventDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HasNewInternetConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasNewConnectionCost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasNewNetworkConnectivityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasNewDomainConnectivityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasNewHostNameList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasNewWwanRegistrationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkStateChangeEventDetails2, INetworkStateChangeEventDetails2_Vtbl, 0xd643c0e8_30d3_4f6a_ad47_6a1873ceb3c1);
impl windows_core::RuntimeType for INetworkStateChangeEventDetails2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkStateChangeEventDetails2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HasNewTetheringOperationalState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HasNewTetheringClientCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetworkUsage, INetworkUsage_Vtbl, 0x49da8fce_9985_4927_bf5b_072b5c65f8d9);
impl windows_core::RuntimeType for INetworkUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct INetworkUsage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BytesSent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub ConnectionDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProviderNetworkUsage, IProviderNetworkUsage_Vtbl, 0x5ec69e04_7931_48c8_b8f3_46300fa42728);
impl windows_core::RuntimeType for IProviderNetworkUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProviderNetworkUsage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BytesSent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub ProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProxyConfiguration, IProxyConfiguration_Vtbl, 0xef3a60b4_9004_4dd6_b7d8_b3e502f4aad0);
impl windows_core::RuntimeType for IProxyConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProxyConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ProxyUris: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProxyUris: usize,
    pub CanConnectDirectly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRoutePolicy, IRoutePolicy_Vtbl, 0x11abc4ac_0fc7_42e4_8742_569923b1ca11);
impl windows_core::RuntimeType for IRoutePolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRoutePolicy_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ConnectionProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HostName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HostNameType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DomainNameType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRoutePolicyFactory, IRoutePolicyFactory_Vtbl, 0x36027933_a18e_4db5_a697_f58fa7364e44);
impl windows_core::RuntimeType for IRoutePolicyFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRoutePolicyFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateRoutePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::DomainNameType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWlanConnectionProfileDetails, IWlanConnectionProfileDetails_Vtbl, 0x562098cb_b35a_4bf1_a884_b7557e88ff86);
impl windows_core::RuntimeType for IWlanConnectionProfileDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWlanConnectionProfileDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetConnectedSsid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWwanConnectionProfileDetails, IWwanConnectionProfileDetails_Vtbl, 0x0e4da8fe_835f_4df3_82fd_df556ebc09ef);
impl windows_core::RuntimeType for IWwanConnectionProfileDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWwanConnectionProfileDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HomeProviderId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AccessPointName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetNetworkRegistrationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WwanNetworkRegistrationState) -> windows_core::HRESULT,
    pub GetCurrentDataClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WwanDataClass) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWwanConnectionProfileDetails2, IWwanConnectionProfileDetails2_Vtbl, 0x7a754ede_a1ed_48b2_8e92_b460033d52e2);
impl windows_core::RuntimeType for IWwanConnectionProfileDetails2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWwanConnectionProfileDetails2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IPKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WwanNetworkIPKind) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PurposeGuids: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PurposeGuids: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AttributedNetworkUsage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AttributedNetworkUsage, windows_core::IUnknown, windows_core::IInspectable);
impl AttributedNetworkUsage {
    pub fn BytesSent(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BytesSent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BytesReceived(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BytesReceived)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AttributionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttributionId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AttributionName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttributionName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AttributionThumbnail(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AttributionThumbnail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for AttributedNetworkUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAttributedNetworkUsage>();
}
unsafe impl windows_core::Interface for AttributedNetworkUsage {
    type Vtable = IAttributedNetworkUsage_Vtbl;
    const IID: windows_core::GUID = <IAttributedNetworkUsage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AttributedNetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.AttributedNetworkUsage";
}
unsafe impl Send for AttributedNetworkUsage {}
unsafe impl Sync for AttributedNetworkUsage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CellularApnContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CellularApnContext, windows_core::IUnknown, windows_core::IInspectable);
impl CellularApnContext {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CellularApnContext, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProviderId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProviderId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn AccessPointName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessPointName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAccessPointName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAccessPointName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn UserName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUserName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUserName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Password(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Password)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPassword(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPassword)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsCompressionEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCompressionEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCompressionEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsCompressionEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AuthenticationType(&self) -> windows_core::Result<CellularApnAuthenticationType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AuthenticationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAuthenticationType(&self, value: CellularApnAuthenticationType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAuthenticationType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProfileName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICellularApnContext2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProfileName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetProfileName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICellularApnContext2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProfileName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for CellularApnContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICellularApnContext>();
}
unsafe impl windows_core::Interface for CellularApnContext {
    type Vtable = ICellularApnContext_Vtbl;
    const IID: windows_core::GUID = <ICellularApnContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CellularApnContext {
    const NAME: &'static str = "Windows.Networking.Connectivity.CellularApnContext";
}
unsafe impl Send for CellularApnContext {}
unsafe impl Sync for CellularApnContext {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ConnectionCost(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConnectionCost, windows_core::IUnknown, windows_core::IInspectable);
impl ConnectionCost {
    pub fn NetworkCostType(&self) -> windows_core::Result<NetworkCostType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkCostType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Roaming(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Roaming)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OverDataLimit(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OverDataLimit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ApproachingDataLimit(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApproachingDataLimit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BackgroundDataUsageRestricted(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IConnectionCost2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundDataUsageRestricted)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ConnectionCost {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IConnectionCost>();
}
unsafe impl windows_core::Interface for ConnectionCost {
    type Vtable = IConnectionCost_Vtbl;
    const IID: windows_core::GUID = <IConnectionCost as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConnectionCost {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionCost";
}
unsafe impl Send for ConnectionCost {}
unsafe impl Sync for ConnectionCost {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ConnectionProfile(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConnectionProfile, windows_core::IUnknown, windows_core::IInspectable);
impl ConnectionProfile {
    pub fn ProfileName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProfileName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNetworkConnectivityLevel(&self) -> windows_core::Result<NetworkConnectivityLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNetworkConnectivityLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNetworkNames(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNetworkNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetConnectionCost(&self) -> windows_core::Result<ConnectionCost> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConnectionCost)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDataPlanStatus(&self) -> windows_core::Result<DataPlanStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDataPlanStatus)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NetworkAdapter(&self) -> windows_core::Result<NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkAdapter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn GetLocalUsage(&self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime) -> windows_core::Result<DataUsage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetLocalUsage)(windows_core::Interface::as_raw(this), starttime, endtime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn GetLocalUsagePerRoamingStates(&self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: RoamingStates) -> windows_core::Result<DataUsage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetLocalUsagePerRoamingStates)(windows_core::Interface::as_raw(this), starttime, endtime, states, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NetworkSecuritySettings(&self) -> windows_core::Result<NetworkSecuritySettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkSecuritySettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsWwanConnectionProfile(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWwanConnectionProfile)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsWlanConnectionProfile(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWlanConnectionProfile)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WwanConnectionProfileDetails(&self) -> windows_core::Result<WwanConnectionProfileDetails> {
        let this = &windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WwanConnectionProfileDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WlanConnectionProfileDetails(&self) -> windows_core::Result<WlanConnectionProfileDetails> {
        let this = &windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WlanConnectionProfileDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ServiceProviderGuid(&self) -> windows_core::Result<super::super::Foundation::IReference<windows_core::GUID>> {
        let this = &windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceProviderGuid)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSignalBars(&self) -> windows_core::Result<super::super::Foundation::IReference<u8>> {
        let this = &windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSignalBars)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDomainConnectivityLevel(&self) -> windows_core::Result<DomainConnectivityLevel> {
        let this = &windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDomainConnectivityLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNetworkUsageAsync(&self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, granularity: DataUsageGranularity, states: NetworkUsageStates) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<NetworkUsage>>> {
        let this = &windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNetworkUsageAsync)(windows_core::Interface::as_raw(this), starttime, endtime, granularity, states, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConnectivityIntervalsAsync(&self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectivityInterval>>> {
        let this = &windows_core::Interface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConnectivityIntervalsAsync)(windows_core::Interface::as_raw(this), starttime, endtime, states, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAttributedNetworkUsageAsync(&self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AttributedNetworkUsage>>> {
        let this = &windows_core::Interface::cast::<IConnectionProfile3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAttributedNetworkUsageAsync)(windows_core::Interface::as_raw(this), starttime, endtime, states, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetProviderNetworkUsageAsync(&self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ProviderNetworkUsage>>> {
        let this = &windows_core::Interface::cast::<IConnectionProfile4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetProviderNetworkUsageAsync)(windows_core::Interface::as_raw(this), starttime, endtime, states, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanDelete(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IConnectionProfile5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDelete)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryDeleteAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ConnectionProfileDeleteStatus>> {
        let this = &windows_core::Interface::cast::<IConnectionProfile5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryDeleteAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsDomainAuthenticatedBy(&self, kind: DomainAuthenticationKind) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IConnectionProfile6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDomainAuthenticatedBy)(windows_core::Interface::as_raw(this), kind, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ConnectionProfile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IConnectionProfile>();
}
unsafe impl windows_core::Interface for ConnectionProfile {
    type Vtable = IConnectionProfile_Vtbl;
    const IID: windows_core::GUID = <IConnectionProfile as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConnectionProfile {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionProfile";
}
unsafe impl Send for ConnectionProfile {}
unsafe impl Sync for ConnectionProfile {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ConnectionProfileFilter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConnectionProfileFilter, windows_core::IUnknown, windows_core::IInspectable);
impl ConnectionProfileFilter {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ConnectionProfileFilter, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetIsConnected(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsConnected)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsConnected(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsConnected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsWwanConnectionProfile(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsWwanConnectionProfile)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsWwanConnectionProfile(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWwanConnectionProfile)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsWlanConnectionProfile(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsWlanConnectionProfile)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsWlanConnectionProfile(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWlanConnectionProfile)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetNetworkCostType(&self, value: NetworkCostType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNetworkCostType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NetworkCostType(&self) -> windows_core::Result<NetworkCostType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkCostType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetServiceProviderGuid<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<windows_core::GUID>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetServiceProviderGuid)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ServiceProviderGuid(&self) -> windows_core::Result<super::super::Foundation::IReference<windows_core::GUID>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceProviderGuid)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsRoaming<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<bool>>,
    {
        let this = &windows_core::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsRoaming)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsRoaming(&self) -> windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = &windows_core::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRoaming)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsOverDataLimit<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<bool>>,
    {
        let this = &windows_core::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsOverDataLimit)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsOverDataLimit(&self) -> windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = &windows_core::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOverDataLimit)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsBackgroundDataUsageRestricted<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<bool>>,
    {
        let this = &windows_core::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsBackgroundDataUsageRestricted)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsBackgroundDataUsageRestricted(&self) -> windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = &windows_core::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBackgroundDataUsageRestricted)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &windows_core::Interface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPurposeGuid<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<windows_core::GUID>>,
    {
        let this = &windows_core::Interface::cast::<IConnectionProfileFilter3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPurposeGuid)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn PurposeGuid(&self) -> windows_core::Result<super::super::Foundation::IReference<windows_core::GUID>> {
        let this = &windows_core::Interface::cast::<IConnectionProfileFilter3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PurposeGuid)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ConnectionProfileFilter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IConnectionProfileFilter>();
}
unsafe impl windows_core::Interface for ConnectionProfileFilter {
    type Vtable = IConnectionProfileFilter_Vtbl;
    const IID: windows_core::GUID = <IConnectionProfileFilter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConnectionProfileFilter {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionProfileFilter";
}
unsafe impl Send for ConnectionProfileFilter {}
unsafe impl Sync for ConnectionProfileFilter {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ConnectionSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConnectionSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ConnectionSession, super::super::Foundation::IClosable);
impl ConnectionSession {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ConnectionProfile(&self) -> windows_core::Result<ConnectionProfile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionProfile)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ConnectionSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IConnectionSession>();
}
unsafe impl windows_core::Interface for ConnectionSession {
    type Vtable = IConnectionSession_Vtbl;
    const IID: windows_core::GUID = <IConnectionSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConnectionSession {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionSession";
}
unsafe impl Send for ConnectionSession {}
unsafe impl Sync for ConnectionSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ConnectivityInterval(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ConnectivityInterval, windows_core::IUnknown, windows_core::IInspectable);
impl ConnectivityInterval {
    pub fn StartTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ConnectionDuration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ConnectivityInterval {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IConnectivityInterval>();
}
unsafe impl windows_core::Interface for ConnectivityInterval {
    type Vtable = IConnectivityInterval_Vtbl;
    const IID: windows_core::GUID = <IConnectivityInterval as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ConnectivityInterval {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectivityInterval";
}
unsafe impl Send for ConnectivityInterval {}
unsafe impl Sync for ConnectivityInterval {}
pub struct ConnectivityManager;
impl ConnectivityManager {
    pub fn AcquireConnectionAsync<P0>(cellularapncontext: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ConnectionSession>>
    where
        P0: windows_core::Param<CellularApnContext>,
    {
        Self::IConnectivityManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AcquireConnectionAsync)(windows_core::Interface::as_raw(this), cellularapncontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AddHttpRoutePolicy<P0>(routepolicy: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutePolicy>,
    {
        Self::IConnectivityManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).AddHttpRoutePolicy)(windows_core::Interface::as_raw(this), routepolicy.param().abi()).ok() })
    }
    pub fn RemoveHttpRoutePolicy<P0>(routepolicy: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<RoutePolicy>,
    {
        Self::IConnectivityManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveHttpRoutePolicy)(windows_core::Interface::as_raw(this), routepolicy.param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IConnectivityManagerStatics<R, F: FnOnce(&IConnectivityManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ConnectivityManager, IConnectivityManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ConnectivityManager {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectivityManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DataPlanStatus(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DataPlanStatus, windows_core::IUnknown, windows_core::IInspectable);
impl DataPlanStatus {
    pub fn DataPlanUsage(&self) -> windows_core::Result<DataPlanUsage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataPlanUsage)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DataLimitInMegabytes(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataLimitInMegabytes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InboundBitsPerSecond(&self) -> windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InboundBitsPerSecond)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OutboundBitsPerSecond(&self) -> windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutboundBitsPerSecond)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextBillingCycle(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextBillingCycle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaxTransferSizeInMegabytes(&self) -> windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxTransferSizeInMegabytes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DataPlanStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDataPlanStatus>();
}
unsafe impl windows_core::Interface for DataPlanStatus {
    type Vtable = IDataPlanStatus_Vtbl;
    const IID: windows_core::GUID = <IDataPlanStatus as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DataPlanStatus {
    const NAME: &'static str = "Windows.Networking.Connectivity.DataPlanStatus";
}
unsafe impl Send for DataPlanStatus {}
unsafe impl Sync for DataPlanStatus {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DataPlanUsage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DataPlanUsage, windows_core::IUnknown, windows_core::IInspectable);
impl DataPlanUsage {
    pub fn MegabytesUsed(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MegabytesUsed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LastSyncTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastSyncTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DataPlanUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDataPlanUsage>();
}
unsafe impl windows_core::Interface for DataPlanUsage {
    type Vtable = IDataPlanUsage_Vtbl;
    const IID: windows_core::GUID = <IDataPlanUsage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DataPlanUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.DataPlanUsage";
}
unsafe impl Send for DataPlanUsage {}
unsafe impl Sync for DataPlanUsage {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DataUsage(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(DataUsage, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl DataUsage {
    #[cfg(feature = "deprecated")]
    pub fn BytesSent(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BytesSent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn BytesReceived(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BytesReceived)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for DataUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDataUsage>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for DataUsage {
    type Vtable = IDataUsage_Vtbl;
    const IID: windows_core::GUID = <IDataUsage as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for DataUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.DataUsage";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for DataUsage {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for DataUsage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IPInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IPInformation, windows_core::IUnknown, windows_core::IInspectable);
impl IPInformation {
    pub fn NetworkAdapter(&self) -> windows_core::Result<NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkAdapter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PrefixLength(&self) -> windows_core::Result<super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrefixLength)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IPInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIPInformation>();
}
unsafe impl windows_core::Interface for IPInformation {
    type Vtable = IIPInformation_Vtbl;
    const IID: windows_core::GUID = <IIPInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IPInformation {
    const NAME: &'static str = "Windows.Networking.Connectivity.IPInformation";
}
unsafe impl Send for IPInformation {}
unsafe impl Sync for IPInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LanIdentifier(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LanIdentifier, windows_core::IUnknown, windows_core::IInspectable);
impl LanIdentifier {
    pub fn InfrastructureId(&self) -> windows_core::Result<LanIdentifierData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InfrastructureId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PortId(&self) -> windows_core::Result<LanIdentifierData> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PortId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NetworkAdapterId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkAdapterId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for LanIdentifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILanIdentifier>();
}
unsafe impl windows_core::Interface for LanIdentifier {
    type Vtable = ILanIdentifier_Vtbl;
    const IID: windows_core::GUID = <ILanIdentifier as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LanIdentifier {
    const NAME: &'static str = "Windows.Networking.Connectivity.LanIdentifier";
}
unsafe impl Send for LanIdentifier {}
unsafe impl Sync for LanIdentifier {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct LanIdentifierData(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LanIdentifierData, windows_core::IUnknown, windows_core::IInspectable);
impl LanIdentifierData {
    pub fn Type(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Value(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LanIdentifierData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILanIdentifierData>();
}
unsafe impl windows_core::Interface for LanIdentifierData {
    type Vtable = ILanIdentifierData_Vtbl;
    const IID: windows_core::GUID = <ILanIdentifierData as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LanIdentifierData {
    const NAME: &'static str = "Windows.Networking.Connectivity.LanIdentifierData";
}
unsafe impl Send for LanIdentifierData {}
unsafe impl Sync for LanIdentifierData {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NetworkAdapter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NetworkAdapter, windows_core::IUnknown, windows_core::IInspectable);
impl NetworkAdapter {
    pub fn OutboundMaxBitsPerSecond(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutboundMaxBitsPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn InboundMaxBitsPerSecond(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InboundMaxBitsPerSecond)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IanaInterfaceType(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IanaInterfaceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NetworkItem(&self) -> windows_core::Result<NetworkItem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkItem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NetworkAdapterId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkAdapterId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetConnectedProfileAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ConnectionProfile>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConnectedProfileAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for NetworkAdapter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INetworkAdapter>();
}
unsafe impl windows_core::Interface for NetworkAdapter {
    type Vtable = INetworkAdapter_Vtbl;
    const IID: windows_core::GUID = <INetworkAdapter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NetworkAdapter {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkAdapter";
}
unsafe impl Send for NetworkAdapter {}
unsafe impl Sync for NetworkAdapter {}
pub struct NetworkInformation;
impl NetworkInformation {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConnectionProfiles() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ConnectionProfile>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConnectionProfiles)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetInternetConnectionProfile() -> windows_core::Result<ConnectionProfile> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInternetConnectionProfile)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetLanIdentifiers() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<LanIdentifier>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetLanIdentifiers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetHostNames() -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetHostNames)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetProxyConfigurationAsync<P0>(uri: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ProxyConfiguration>>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetProxyConfigurationAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSortedEndpointPairs<P0>(destinationlist: P0, sortoptions: super::HostNameSortOptions) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<super::EndpointPair>>,
    {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSortedEndpointPairs)(windows_core::Interface::as_raw(this), destinationlist.param().abi(), sortoptions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn NetworkStatusChanged<P0>(networkstatushandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<NetworkStatusChangedEventHandler>,
    {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkStatusChanged)(windows_core::Interface::as_raw(this), networkstatushandler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveNetworkStatusChanged(eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::INetworkInformationStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveNetworkStatusChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindConnectionProfilesAsync<P0>(pprofilefilter: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectionProfile>>>
    where
        P0: windows_core::Param<ConnectionProfileFilter>,
    {
        Self::INetworkInformationStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindConnectionProfilesAsync)(windows_core::Interface::as_raw(this), pprofilefilter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn INetworkInformationStatics<R, F: FnOnce(&INetworkInformationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NetworkInformation, INetworkInformationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn INetworkInformationStatics2<R, F: FnOnce(&INetworkInformationStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<NetworkInformation, INetworkInformationStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for NetworkInformation {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkInformation";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NetworkItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NetworkItem, windows_core::IUnknown, windows_core::IInspectable);
impl NetworkItem {
    pub fn NetworkId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetNetworkTypes(&self) -> windows_core::Result<NetworkTypes> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNetworkTypes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for NetworkItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INetworkItem>();
}
unsafe impl windows_core::Interface for NetworkItem {
    type Vtable = INetworkItem_Vtbl;
    const IID: windows_core::GUID = <INetworkItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NetworkItem {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkItem";
}
unsafe impl Send for NetworkItem {}
unsafe impl Sync for NetworkItem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NetworkSecuritySettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NetworkSecuritySettings, windows_core::IUnknown, windows_core::IInspectable);
impl NetworkSecuritySettings {
    pub fn NetworkAuthenticationType(&self) -> windows_core::Result<NetworkAuthenticationType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkAuthenticationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NetworkEncryptionType(&self) -> windows_core::Result<NetworkEncryptionType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NetworkEncryptionType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for NetworkSecuritySettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INetworkSecuritySettings>();
}
unsafe impl windows_core::Interface for NetworkSecuritySettings {
    type Vtable = INetworkSecuritySettings_Vtbl;
    const IID: windows_core::GUID = <INetworkSecuritySettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NetworkSecuritySettings {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkSecuritySettings";
}
unsafe impl Send for NetworkSecuritySettings {}
unsafe impl Sync for NetworkSecuritySettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NetworkStateChangeEventDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NetworkStateChangeEventDetails, windows_core::IUnknown, windows_core::IInspectable);
impl NetworkStateChangeEventDetails {
    pub fn HasNewInternetConnectionProfile(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasNewInternetConnectionProfile)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasNewConnectionCost(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasNewConnectionCost)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasNewNetworkConnectivityLevel(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasNewNetworkConnectivityLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasNewDomainConnectivityLevel(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasNewDomainConnectivityLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasNewHostNameList(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasNewHostNameList)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasNewWwanRegistrationState(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasNewWwanRegistrationState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasNewTetheringOperationalState(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INetworkStateChangeEventDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasNewTetheringOperationalState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasNewTetheringClientCount(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<INetworkStateChangeEventDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasNewTetheringClientCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for NetworkStateChangeEventDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INetworkStateChangeEventDetails>();
}
unsafe impl windows_core::Interface for NetworkStateChangeEventDetails {
    type Vtable = INetworkStateChangeEventDetails_Vtbl;
    const IID: windows_core::GUID = <INetworkStateChangeEventDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NetworkStateChangeEventDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkStateChangeEventDetails";
}
unsafe impl Send for NetworkStateChangeEventDetails {}
unsafe impl Sync for NetworkStateChangeEventDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NetworkUsage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NetworkUsage, windows_core::IUnknown, windows_core::IInspectable);
impl NetworkUsage {
    pub fn BytesSent(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BytesSent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BytesReceived(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BytesReceived)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ConnectionDuration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for NetworkUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INetworkUsage>();
}
unsafe impl windows_core::Interface for NetworkUsage {
    type Vtable = INetworkUsage_Vtbl;
    const IID: windows_core::GUID = <INetworkUsage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkUsage";
}
unsafe impl Send for NetworkUsage {}
unsafe impl Sync for NetworkUsage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProviderNetworkUsage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProviderNetworkUsage, windows_core::IUnknown, windows_core::IInspectable);
impl ProviderNetworkUsage {
    pub fn BytesSent(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BytesSent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BytesReceived(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BytesReceived)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ProviderNetworkUsage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProviderNetworkUsage>();
}
unsafe impl windows_core::Interface for ProviderNetworkUsage {
    type Vtable = IProviderNetworkUsage_Vtbl;
    const IID: windows_core::GUID = <IProviderNetworkUsage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProviderNetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.ProviderNetworkUsage";
}
unsafe impl Send for ProviderNetworkUsage {}
unsafe impl Sync for ProviderNetworkUsage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ProxyConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProxyConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl ProxyConfiguration {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProxyUris(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProxyUris)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanConnectDirectly(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanConnectDirectly)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ProxyConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProxyConfiguration>();
}
unsafe impl windows_core::Interface for ProxyConfiguration {
    type Vtable = IProxyConfiguration_Vtbl;
    const IID: windows_core::GUID = <IProxyConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProxyConfiguration {
    const NAME: &'static str = "Windows.Networking.Connectivity.ProxyConfiguration";
}
unsafe impl Send for ProxyConfiguration {}
unsafe impl Sync for ProxyConfiguration {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RoutePolicy(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RoutePolicy, windows_core::IUnknown, windows_core::IInspectable);
impl RoutePolicy {
    pub fn ConnectionProfile(&self) -> windows_core::Result<ConnectionProfile> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionProfile)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HostName(&self) -> windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HostName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HostNameType(&self) -> windows_core::Result<super::DomainNameType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HostNameType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateRoutePolicy<P0, P1>(connectionprofile: P0, hostname: P1, r#type: super::DomainNameType) -> windows_core::Result<RoutePolicy>
    where
        P0: windows_core::Param<ConnectionProfile>,
        P1: windows_core::Param<super::HostName>,
    {
        Self::IRoutePolicyFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateRoutePolicy)(windows_core::Interface::as_raw(this), connectionprofile.param().abi(), hostname.param().abi(), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRoutePolicyFactory<R, F: FnOnce(&IRoutePolicyFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RoutePolicy, IRoutePolicyFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RoutePolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRoutePolicy>();
}
unsafe impl windows_core::Interface for RoutePolicy {
    type Vtable = IRoutePolicy_Vtbl;
    const IID: windows_core::GUID = <IRoutePolicy as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RoutePolicy {
    const NAME: &'static str = "Windows.Networking.Connectivity.RoutePolicy";
}
unsafe impl Send for RoutePolicy {}
unsafe impl Sync for RoutePolicy {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WlanConnectionProfileDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WlanConnectionProfileDetails, windows_core::IUnknown, windows_core::IInspectable);
impl WlanConnectionProfileDetails {
    pub fn GetConnectedSsid(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetConnectedSsid)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WlanConnectionProfileDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWlanConnectionProfileDetails>();
}
unsafe impl windows_core::Interface for WlanConnectionProfileDetails {
    type Vtable = IWlanConnectionProfileDetails_Vtbl;
    const IID: windows_core::GUID = <IWlanConnectionProfileDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WlanConnectionProfileDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.WlanConnectionProfileDetails";
}
unsafe impl Send for WlanConnectionProfileDetails {}
unsafe impl Sync for WlanConnectionProfileDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WwanConnectionProfileDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WwanConnectionProfileDetails, windows_core::IUnknown, windows_core::IInspectable);
impl WwanConnectionProfileDetails {
    pub fn HomeProviderId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HomeProviderId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AccessPointName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessPointName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNetworkRegistrationState(&self) -> windows_core::Result<WwanNetworkRegistrationState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNetworkRegistrationState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetCurrentDataClass(&self) -> windows_core::Result<WwanDataClass> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentDataClass)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IPKind(&self) -> windows_core::Result<WwanNetworkIPKind> {
        let this = &windows_core::Interface::cast::<IWwanConnectionProfileDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IPKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PurposeGuids(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::GUID>> {
        let this = &windows_core::Interface::cast::<IWwanConnectionProfileDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PurposeGuids)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for WwanConnectionProfileDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWwanConnectionProfileDetails>();
}
unsafe impl windows_core::Interface for WwanConnectionProfileDetails {
    type Vtable = IWwanConnectionProfileDetails_Vtbl;
    const IID: windows_core::GUID = <IWwanConnectionProfileDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WwanConnectionProfileDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.WwanConnectionProfileDetails";
}
unsafe impl Send for WwanConnectionProfileDetails {}
unsafe impl Sync for WwanConnectionProfileDetails {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CellularApnAuthenticationType(pub i32);
impl CellularApnAuthenticationType {
    pub const None: Self = Self(0i32);
    pub const Pap: Self = Self(1i32);
    pub const Chap: Self = Self(2i32);
    pub const Mschapv2: Self = Self(3i32);
}
impl windows_core::TypeKind for CellularApnAuthenticationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CellularApnAuthenticationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CellularApnAuthenticationType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CellularApnAuthenticationType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.CellularApnAuthenticationType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ConnectionProfileDeleteStatus(pub i32);
impl ConnectionProfileDeleteStatus {
    pub const Success: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl windows_core::TypeKind for ConnectionProfileDeleteStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ConnectionProfileDeleteStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ConnectionProfileDeleteStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ConnectionProfileDeleteStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.ConnectionProfileDeleteStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DataUsageGranularity(pub i32);
impl DataUsageGranularity {
    pub const PerMinute: Self = Self(0i32);
    pub const PerHour: Self = Self(1i32);
    pub const PerDay: Self = Self(2i32);
    pub const Total: Self = Self(3i32);
}
impl windows_core::TypeKind for DataUsageGranularity {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DataUsageGranularity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DataUsageGranularity").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DataUsageGranularity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.DataUsageGranularity;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DomainAuthenticationKind(pub i32);
impl DomainAuthenticationKind {
    pub const None: Self = Self(0i32);
    pub const Ldap: Self = Self(1i32);
    pub const Tls: Self = Self(2i32);
}
impl windows_core::TypeKind for DomainAuthenticationKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DomainAuthenticationKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DomainAuthenticationKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DomainAuthenticationKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.DomainAuthenticationKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DomainConnectivityLevel(pub i32);
impl DomainConnectivityLevel {
    pub const None: Self = Self(0i32);
    pub const Unauthenticated: Self = Self(1i32);
    pub const Authenticated: Self = Self(2i32);
}
impl windows_core::TypeKind for DomainConnectivityLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DomainConnectivityLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DomainConnectivityLevel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DomainConnectivityLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.DomainConnectivityLevel;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NetworkAuthenticationType(pub i32);
impl NetworkAuthenticationType {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const Open80211: Self = Self(2i32);
    pub const SharedKey80211: Self = Self(3i32);
    pub const Wpa: Self = Self(4i32);
    pub const WpaPsk: Self = Self(5i32);
    pub const WpaNone: Self = Self(6i32);
    pub const Rsna: Self = Self(7i32);
    pub const RsnaPsk: Self = Self(8i32);
    pub const Ihv: Self = Self(9i32);
    pub const Wpa3: Self = Self(10i32);
    pub const Wpa3Enterprise192Bits: Self = Self(10i32);
    pub const Wpa3Sae: Self = Self(11i32);
    pub const Owe: Self = Self(12i32);
    pub const Wpa3Enterprise: Self = Self(13i32);
}
impl windows_core::TypeKind for NetworkAuthenticationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NetworkAuthenticationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NetworkAuthenticationType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for NetworkAuthenticationType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkAuthenticationType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NetworkConnectivityLevel(pub i32);
impl NetworkConnectivityLevel {
    pub const None: Self = Self(0i32);
    pub const LocalAccess: Self = Self(1i32);
    pub const ConstrainedInternetAccess: Self = Self(2i32);
    pub const InternetAccess: Self = Self(3i32);
}
impl windows_core::TypeKind for NetworkConnectivityLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NetworkConnectivityLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NetworkConnectivityLevel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for NetworkConnectivityLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkConnectivityLevel;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NetworkCostType(pub i32);
impl NetworkCostType {
    pub const Unknown: Self = Self(0i32);
    pub const Unrestricted: Self = Self(1i32);
    pub const Fixed: Self = Self(2i32);
    pub const Variable: Self = Self(3i32);
}
impl windows_core::TypeKind for NetworkCostType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NetworkCostType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NetworkCostType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for NetworkCostType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkCostType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NetworkEncryptionType(pub i32);
impl NetworkEncryptionType {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const Wep: Self = Self(2i32);
    pub const Wep40: Self = Self(3i32);
    pub const Wep104: Self = Self(4i32);
    pub const Tkip: Self = Self(5i32);
    pub const Ccmp: Self = Self(6i32);
    pub const WpaUseGroup: Self = Self(7i32);
    pub const RsnUseGroup: Self = Self(8i32);
    pub const Ihv: Self = Self(9i32);
    pub const Gcmp: Self = Self(10i32);
    pub const Gcmp256: Self = Self(11i32);
}
impl windows_core::TypeKind for NetworkEncryptionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NetworkEncryptionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NetworkEncryptionType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for NetworkEncryptionType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkEncryptionType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NetworkTypes(pub u32);
impl NetworkTypes {
    pub const None: Self = Self(0u32);
    pub const Internet: Self = Self(1u32);
    pub const PrivateNetwork: Self = Self(2u32);
}
impl windows_core::TypeKind for NetworkTypes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NetworkTypes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NetworkTypes").field(&self.0).finish()
    }
}
impl NetworkTypes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NetworkTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NetworkTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NetworkTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NetworkTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NetworkTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for NetworkTypes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkTypes;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RoamingStates(pub u32);
impl RoamingStates {
    pub const None: Self = Self(0u32);
    pub const NotRoaming: Self = Self(1u32);
    pub const Roaming: Self = Self(2u32);
}
impl windows_core::TypeKind for RoamingStates {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RoamingStates {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RoamingStates").field(&self.0).finish()
    }
}
impl RoamingStates {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RoamingStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RoamingStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RoamingStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RoamingStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RoamingStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for RoamingStates {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.RoamingStates;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TriStates(pub i32);
impl TriStates {
    pub const DoNotCare: Self = Self(0i32);
    pub const No: Self = Self(1i32);
    pub const Yes: Self = Self(2i32);
}
impl windows_core::TypeKind for TriStates {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TriStates {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TriStates").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TriStates {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.TriStates;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WwanDataClass(pub u32);
impl WwanDataClass {
    pub const None: Self = Self(0u32);
    pub const Gprs: Self = Self(1u32);
    pub const Edge: Self = Self(2u32);
    pub const Umts: Self = Self(4u32);
    pub const Hsdpa: Self = Self(8u32);
    pub const Hsupa: Self = Self(16u32);
    pub const LteAdvanced: Self = Self(32u32);
    pub const NewRadioNonStandalone: Self = Self(64u32);
    pub const NewRadioStandalone: Self = Self(128u32);
    pub const Cdma1xRtt: Self = Self(65536u32);
    pub const Cdma1xEvdo: Self = Self(131072u32);
    pub const Cdma1xEvdoRevA: Self = Self(262144u32);
    pub const Cdma1xEvdv: Self = Self(524288u32);
    pub const Cdma3xRtt: Self = Self(1048576u32);
    pub const Cdma1xEvdoRevB: Self = Self(2097152u32);
    pub const CdmaUmb: Self = Self(4194304u32);
    pub const Custom: Self = Self(2147483648u32);
}
impl windows_core::TypeKind for WwanDataClass {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WwanDataClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WwanDataClass").field(&self.0).finish()
    }
}
impl WwanDataClass {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WwanDataClass {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WwanDataClass {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WwanDataClass {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WwanDataClass {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WwanDataClass {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for WwanDataClass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.WwanDataClass;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WwanNetworkIPKind(pub i32);
impl WwanNetworkIPKind {
    pub const None: Self = Self(0i32);
    pub const Ipv4: Self = Self(1i32);
    pub const Ipv6: Self = Self(2i32);
    pub const Ipv4v6: Self = Self(3i32);
    pub const Ipv4v6v4Xlat: Self = Self(4i32);
}
impl windows_core::TypeKind for WwanNetworkIPKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WwanNetworkIPKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WwanNetworkIPKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WwanNetworkIPKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.WwanNetworkIPKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WwanNetworkRegistrationState(pub i32);
impl WwanNetworkRegistrationState {
    pub const None: Self = Self(0i32);
    pub const Deregistered: Self = Self(1i32);
    pub const Searching: Self = Self(2i32);
    pub const Home: Self = Self(3i32);
    pub const Roaming: Self = Self(4i32);
    pub const Partner: Self = Self(5i32);
    pub const Denied: Self = Self(6i32);
}
impl windows_core::TypeKind for WwanNetworkRegistrationState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WwanNetworkRegistrationState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WwanNetworkRegistrationState").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for WwanNetworkRegistrationState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.WwanNetworkRegistrationState;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NetworkUsageStates {
    pub Roaming: TriStates,
    pub Shared: TriStates,
}
impl windows_core::TypeKind for NetworkUsageStates {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for NetworkUsageStates {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Networking.Connectivity.NetworkUsageStates;enum(Windows.Networking.Connectivity.TriStates;i4);enum(Windows.Networking.Connectivity.TriStates;i4))");
}
impl Default for NetworkUsageStates {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(NetworkStatusChangedEventHandler, NetworkStatusChangedEventHandler_Vtbl, 0x71ba143f_598e_49d0_84eb_8febaedcc195);
impl NetworkStatusChangedEventHandler {
    pub fn new<F: FnMut(Option<&windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = NetworkStatusChangedEventHandlerBox::<F> { vtable: &NetworkStatusChangedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi()).ok() }
    }
}
#[repr(C)]
struct NetworkStatusChangedEventHandlerBox<F: FnMut(Option<&windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const NetworkStatusChangedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static> NetworkStatusChangedEventHandlerBox<F> {
    const VTABLE: NetworkStatusChangedEventHandler_Vtbl = NetworkStatusChangedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <NetworkStatusChangedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
impl windows_core::RuntimeType for NetworkStatusChangedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct NetworkStatusChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
