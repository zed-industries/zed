windows_core::imp::define_interface!(IRfcommDeviceService, IRfcommDeviceService_Vtbl, 0xae81ff1f_c5a1_4c40_8c28_f3efd69062f3);
impl windows_core::RuntimeType for IRfcommDeviceService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRfcommDeviceService_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking")]
    pub ConnectionHostName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    ConnectionHostName: usize,
    pub ConnectionServiceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub ProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Networking::Sockets::SocketProtectionLevel) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    ProtectionLevel: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub MaxProtectionLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Networking::Sockets::SocketProtectionLevel) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    MaxProtectionLevel: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetSdpRawAttributesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetSdpRawAttributesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetSdpRawAttributesWithCacheModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::BluetoothCacheMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetSdpRawAttributesWithCacheModeAsync: usize,
}
windows_core::imp::define_interface!(IRfcommDeviceService2, IRfcommDeviceService2_Vtbl, 0x536ced14_ebcd_49fe_bf9f_40efc689b20d);
impl windows_core::RuntimeType for IRfcommDeviceService2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRfcommDeviceService2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRfcommDeviceService3, IRfcommDeviceService3_Vtbl, 0x1c22ace6_dd44_4d23_866d_8f3486ee6490);
impl windows_core::RuntimeType for IRfcommDeviceService3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRfcommDeviceService3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceAccessInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceAccessInformation: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    RequestAccessAsync: usize,
}
windows_core::imp::define_interface!(IRfcommDeviceServiceStatics, IRfcommDeviceServiceStatics_Vtbl, 0xa4a149ef_626d_41ac_b253_87ac5c27e28a);
impl windows_core::RuntimeType for IRfcommDeviceServiceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRfcommDeviceServiceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRfcommDeviceServiceStatics2, IRfcommDeviceServiceStatics2_Vtbl, 0xaa8cb1c9_e78d_4be4_8076_0a3d87a0a05f);
impl windows_core::RuntimeType for IRfcommDeviceServiceStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRfcommDeviceServiceStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelectorForBluetoothDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceWithCacheMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::BluetoothCacheMode, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceAndServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::BluetoothCacheMode, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRfcommDeviceServicesResult, IRfcommDeviceServicesResult_Vtbl, 0x3b48388c_7ccf_488e_9625_d259a5732d55);
impl windows_core::RuntimeType for IRfcommDeviceServicesResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRfcommDeviceServicesResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BluetoothError) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Services: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Services: usize,
}
windows_core::imp::define_interface!(IRfcommServiceId, IRfcommServiceId_Vtbl, 0x22629204_7e02_4017_8136_da1b6a1b9bbf);
impl windows_core::RuntimeType for IRfcommServiceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRfcommServiceId_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub AsShortId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AsString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRfcommServiceIdStatics, IRfcommServiceIdStatics_Vtbl, 0x2a179eba_a975_46e3_b56b_08ffd783a5fe);
impl windows_core::RuntimeType for IRfcommServiceIdStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRfcommServiceIdStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromUuid: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromShortId: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SerialPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ObexObjectPush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ObexFileTransfer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PhoneBookAccessPce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PhoneBookAccessPse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GenericFileTransfer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRfcommServiceProvider, IRfcommServiceProvider_Vtbl, 0xeadbfdc4_b1f6_44ff_9f7c_e7a82ab86821);
impl windows_core::RuntimeType for IRfcommServiceProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRfcommServiceProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServiceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub SdpRawAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    SdpRawAttributes: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub StartAdvertising: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    StartAdvertising: usize,
    pub StopAdvertising: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRfcommServiceProvider2, IRfcommServiceProvider2_Vtbl, 0x736bdfc6_3c81_4d1e_baf2_ddbb81284512);
impl windows_core::RuntimeType for IRfcommServiceProvider2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRfcommServiceProvider2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Sockets")]
    pub StartAdvertisingWithRadioDiscoverability: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    StartAdvertisingWithRadioDiscoverability: usize,
}
windows_core::imp::define_interface!(IRfcommServiceProviderStatics, IRfcommServiceProviderStatics_Vtbl, 0x98888303_69ca_413a_84f7_4344c7292997);
impl windows_core::RuntimeType for IRfcommServiceProviderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRfcommServiceProviderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RfcommDeviceService(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RfcommDeviceService, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RfcommDeviceService, super::super::super::Foundation::IClosable);
impl RfcommDeviceService {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Networking")]
    pub fn ConnectionHostName(&self) -> windows_core::Result<super::super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionHostName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ConnectionServiceName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ConnectionServiceName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ServiceId(&self) -> windows_core::Result<RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn ProtectionLevel(&self) -> windows_core::Result<super::super::super::Networking::Sockets::SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn MaxProtectionLevel(&self) -> windows_core::Result<super::super::super::Networking::Sockets::SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxProtectionLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetSdpRawAttributesAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSdpRawAttributesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetSdpRawAttributesWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<u32, super::super::super::Storage::Streams::IBuffer>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSdpRawAttributesWithCacheModeAsync)(windows_core::Interface::as_raw(this), cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Device(&self) -> windows_core::Result<super::BluetoothDevice> {
        let this = &windows_core::Interface::cast::<IRfcommDeviceService2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Device)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceAccessInformation(&self) -> windows_core::Result<super::super::Enumeration::DeviceAccessInformation> {
        let this = &windows_core::Interface::cast::<IRfcommDeviceService3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceAccessInformation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn RequestAccessAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>> {
        let this = &windows_core::Interface::cast::<IRfcommDeviceService3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<RfcommDeviceService>> {
        Self::IRfcommDeviceServiceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector<P0>(serviceid: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<RfcommServiceId>,
    {
        Self::IRfcommDeviceServiceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), serviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorForBluetoothDevice<P0>(bluetoothdevice: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::BluetoothDevice>,
    {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorForBluetoothDevice)(windows_core::Interface::as_raw(this), bluetoothdevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceWithCacheMode<P0>(bluetoothdevice: P0, cachemode: super::BluetoothCacheMode) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::BluetoothDevice>,
    {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorForBluetoothDeviceWithCacheMode)(windows_core::Interface::as_raw(this), bluetoothdevice.param().abi(), cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceAndServiceId<P0, P1>(bluetoothdevice: P0, serviceid: P1) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::BluetoothDevice>,
        P1: windows_core::Param<RfcommServiceId>,
    {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorForBluetoothDeviceAndServiceId)(windows_core::Interface::as_raw(this), bluetoothdevice.param().abi(), serviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode<P0, P1>(bluetoothdevice: P0, serviceid: P1, cachemode: super::BluetoothCacheMode) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<super::BluetoothDevice>,
        P1: windows_core::Param<RfcommServiceId>,
    {
        Self::IRfcommDeviceServiceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelectorForBluetoothDeviceAndServiceIdWithCacheMode)(windows_core::Interface::as_raw(this), bluetoothdevice.param().abi(), serviceid.param().abi(), cachemode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRfcommDeviceServiceStatics<R, F: FnOnce(&IRfcommDeviceServiceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RfcommDeviceService, IRfcommDeviceServiceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRfcommDeviceServiceStatics2<R, F: FnOnce(&IRfcommDeviceServiceStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RfcommDeviceService, IRfcommDeviceServiceStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RfcommDeviceService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRfcommDeviceService>();
}
unsafe impl windows_core::Interface for RfcommDeviceService {
    type Vtable = IRfcommDeviceService_Vtbl;
    const IID: windows_core::GUID = <IRfcommDeviceService as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RfcommDeviceService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceService";
}
unsafe impl Send for RfcommDeviceService {}
unsafe impl Sync for RfcommDeviceService {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RfcommDeviceServicesResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RfcommDeviceServicesResult, windows_core::IUnknown, windows_core::IInspectable);
impl RfcommDeviceServicesResult {
    pub fn Error(&self) -> windows_core::Result<super::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Services(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<RfcommDeviceService>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Services)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RfcommDeviceServicesResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRfcommDeviceServicesResult>();
}
unsafe impl windows_core::Interface for RfcommDeviceServicesResult {
    type Vtable = IRfcommDeviceServicesResult_Vtbl;
    const IID: windows_core::GUID = <IRfcommDeviceServicesResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RfcommDeviceServicesResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommDeviceServicesResult";
}
unsafe impl Send for RfcommDeviceServicesResult {}
unsafe impl Sync for RfcommDeviceServicesResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RfcommServiceId(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RfcommServiceId, windows_core::IUnknown, windows_core::IInspectable);
impl RfcommServiceId {
    pub fn Uuid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AsShortId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AsShortId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AsString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AsString)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FromUuid(uuid: windows_core::GUID) -> windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromUuid)(windows_core::Interface::as_raw(this), uuid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromShortId(shortid: u32) -> windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromShortId)(windows_core::Interface::as_raw(this), shortid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SerialPort() -> windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SerialPort)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ObexObjectPush() -> windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ObexObjectPush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ObexFileTransfer() -> windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ObexFileTransfer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PhoneBookAccessPce() -> windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneBookAccessPce)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn PhoneBookAccessPse() -> windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhoneBookAccessPse)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GenericFileTransfer() -> windows_core::Result<RfcommServiceId> {
        Self::IRfcommServiceIdStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GenericFileTransfer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRfcommServiceIdStatics<R, F: FnOnce(&IRfcommServiceIdStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RfcommServiceId, IRfcommServiceIdStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RfcommServiceId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRfcommServiceId>();
}
unsafe impl windows_core::Interface for RfcommServiceId {
    type Vtable = IRfcommServiceId_Vtbl;
    const IID: windows_core::GUID = <IRfcommServiceId as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RfcommServiceId {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommServiceId";
}
unsafe impl Send for RfcommServiceId {}
unsafe impl Sync for RfcommServiceId {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RfcommServiceProvider(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RfcommServiceProvider, windows_core::IUnknown, windows_core::IInspectable);
impl RfcommServiceProvider {
    pub fn ServiceId(&self) -> windows_core::Result<RfcommServiceId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn SdpRawAttributes(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMap<u32, super::super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SdpRawAttributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn StartAdvertising<P0>(&self, listener: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Networking::Sockets::StreamSocketListener>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartAdvertising)(windows_core::Interface::as_raw(this), listener.param().abi()).ok() }
    }
    pub fn StopAdvertising(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopAdvertising)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Networking_Sockets")]
    pub fn StartAdvertisingWithRadioDiscoverability<P0>(&self, listener: P0, radiodiscoverable: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Networking::Sockets::StreamSocketListener>,
    {
        let this = &windows_core::Interface::cast::<IRfcommServiceProvider2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartAdvertisingWithRadioDiscoverability)(windows_core::Interface::as_raw(this), listener.param().abi(), radiodiscoverable).ok() }
    }
    pub fn CreateAsync<P0>(serviceid: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<RfcommServiceProvider>>
    where
        P0: windows_core::Param<RfcommServiceId>,
    {
        Self::IRfcommServiceProviderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAsync)(windows_core::Interface::as_raw(this), serviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRfcommServiceProviderStatics<R, F: FnOnce(&IRfcommServiceProviderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RfcommServiceProvider, IRfcommServiceProviderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RfcommServiceProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRfcommServiceProvider>();
}
unsafe impl windows_core::Interface for RfcommServiceProvider {
    type Vtable = IRfcommServiceProvider_Vtbl;
    const IID: windows_core::GUID = <IRfcommServiceProvider as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RfcommServiceProvider {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Rfcomm.RfcommServiceProvider";
}
unsafe impl Send for RfcommServiceProvider {}
unsafe impl Sync for RfcommServiceProvider {}
