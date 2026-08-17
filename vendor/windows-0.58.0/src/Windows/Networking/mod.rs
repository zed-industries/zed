#[cfg(feature = "Networking_BackgroundTransfer")]
pub mod BackgroundTransfer;
#[cfg(feature = "Networking_Connectivity")]
pub mod Connectivity;
#[cfg(feature = "Networking_NetworkOperators")]
pub mod NetworkOperators;
#[cfg(feature = "Networking_Proximity")]
pub mod Proximity;
#[cfg(feature = "Networking_PushNotifications")]
pub mod PushNotifications;
#[cfg(feature = "Networking_ServiceDiscovery")]
pub mod ServiceDiscovery;
#[cfg(feature = "Networking_Sockets")]
pub mod Sockets;
#[cfg(feature = "Networking_Vpn")]
pub mod Vpn;
#[cfg(feature = "Networking_XboxLive")]
pub mod XboxLive;
windows_core::imp::define_interface!(IEndpointPair, IEndpointPair_Vtbl, 0x33a0aa36_f8fa_4b30_b856_76517c3bd06d);
impl windows_core::RuntimeType for IEndpointPair {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEndpointPair_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LocalHostName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalHostName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetLocalServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RemoteHostName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteHostName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoteServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetRemoteServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEndpointPairFactory, IEndpointPairFactory_Vtbl, 0xb609d971_64e0_442b_aa6f_cc8c8f181f78);
impl windows_core::RuntimeType for IEndpointPairFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEndpointPairFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateEndpointPair: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostName, IHostName_Vtbl, 0xbf8ecaad_ed96_49a7_9084_d416cae88dcb);
impl windows_core::RuntimeType for IHostName {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHostName_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub IPInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    IPInformation: usize,
    pub RawName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CanonicalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HostNameType) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostNameFactory, IHostNameFactory_Vtbl, 0x458c23ed_712f_4576_adf1_c20b2c643558);
impl windows_core::RuntimeType for IHostNameFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHostNameFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateHostName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHostNameStatics, IHostNameStatics_Vtbl, 0xf68cd4bf_a388_4e8b_91ea_54dd6dd901c0);
impl windows_core::RuntimeType for IHostNameStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHostNameStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut i32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EndpointPair(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EndpointPair, windows_core::IUnknown, windows_core::IInspectable);
impl EndpointPair {
    pub fn LocalHostName(&self) -> windows_core::Result<HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalHostName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLocalHostName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HostName>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLocalHostName)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn LocalServiceName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalServiceName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLocalServiceName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLocalServiceName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn RemoteHostName(&self) -> windows_core::Result<HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteHostName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRemoteHostName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HostName>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRemoteHostName)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoteServiceName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoteServiceName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetRemoteServiceName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRemoteServiceName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateEndpointPair<P0, P1>(localhostname: P0, localservicename: &windows_core::HSTRING, remotehostname: P1, remoteservicename: &windows_core::HSTRING) -> windows_core::Result<EndpointPair>
    where
        P0: windows_core::Param<HostName>,
        P1: windows_core::Param<HostName>,
    {
        Self::IEndpointPairFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateEndpointPair)(windows_core::Interface::as_raw(this), localhostname.param().abi(), core::mem::transmute_copy(localservicename), remotehostname.param().abi(), core::mem::transmute_copy(remoteservicename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IEndpointPairFactory<R, F: FnOnce(&IEndpointPairFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<EndpointPair, IEndpointPairFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for EndpointPair {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEndpointPair>();
}
unsafe impl windows_core::Interface for EndpointPair {
    type Vtable = IEndpointPair_Vtbl;
    const IID: windows_core::GUID = <IEndpointPair as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EndpointPair {
    const NAME: &'static str = "Windows.Networking.EndpointPair";
}
unsafe impl Send for EndpointPair {}
unsafe impl Sync for EndpointPair {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct HostName(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HostName, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HostName, super::Foundation::IStringable);
impl HostName {
    #[cfg(feature = "Networking_Connectivity")]
    pub fn IPInformation(&self) -> windows_core::Result<Connectivity::IPInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IPInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RawName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanonicalName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanonicalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<HostNameType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsEqual<P0>(&self, hostname: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<HostName>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEqual)(windows_core::Interface::as_raw(this), hostname.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn CreateHostName(hostname: &windows_core::HSTRING) -> windows_core::Result<HostName> {
        Self::IHostNameFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateHostName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(hostname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Compare(value1: &windows_core::HSTRING, value2: &windows_core::HSTRING) -> windows_core::Result<i32> {
        Self::IHostNameStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compare)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value1), core::mem::transmute_copy(value2), &mut result__).map(|| result__)
        })
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[doc(hidden)]
    pub fn IHostNameFactory<R, F: FnOnce(&IHostNameFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HostName, IHostNameFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IHostNameStatics<R, F: FnOnce(&IHostNameStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HostName, IHostNameStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HostName {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHostName>();
}
unsafe impl windows_core::Interface for HostName {
    type Vtable = IHostName_Vtbl;
    const IID: windows_core::GUID = <IHostName as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HostName {
    const NAME: &'static str = "Windows.Networking.HostName";
}
unsafe impl Send for HostName {}
unsafe impl Sync for HostName {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DomainNameType(pub i32);
impl DomainNameType {
    pub const Suffix: Self = Self(0i32);
    pub const FullyQualified: Self = Self(1i32);
}
impl windows_core::TypeKind for DomainNameType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DomainNameType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DomainNameType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DomainNameType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.DomainNameType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HostNameSortOptions(pub u32);
impl HostNameSortOptions {
    pub const None: Self = Self(0u32);
    pub const OptimizeForLongConnections: Self = Self(2u32);
}
impl windows_core::TypeKind for HostNameSortOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HostNameSortOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HostNameSortOptions").field(&self.0).finish()
    }
}
impl HostNameSortOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for HostNameSortOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for HostNameSortOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for HostNameSortOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for HostNameSortOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for HostNameSortOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for HostNameSortOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.HostNameSortOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HostNameType(pub i32);
impl HostNameType {
    pub const DomainName: Self = Self(0i32);
    pub const Ipv4: Self = Self(1i32);
    pub const Ipv6: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
}
impl windows_core::TypeKind for HostNameType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HostNameType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HostNameType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HostNameType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.HostNameType;i4)");
}
