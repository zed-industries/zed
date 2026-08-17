windows_core::imp::define_interface!(ICustomDevice, ICustomDevice_Vtbl, 0xdd30251f_c48b_43bd_bcb1_dec88f15143e);
impl windows_core::RuntimeType for ICustomDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICustomDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SendIOControlAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SendIOControlAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub TrySendIOControlAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TrySendIOControlAsync: usize,
}
windows_core::imp::define_interface!(ICustomDeviceStatics, ICustomDeviceStatics_Vtbl, 0xc8220312_ef4c_46b1_a58e_eeb308dc8917);
impl windows_core::RuntimeType for ICustomDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICustomDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, DeviceAccessMode, DeviceSharingMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIOControlCode, IIOControlCode_Vtbl, 0x0e9559e7_60c8_4375_a761_7f8808066c60);
impl core::ops::Deref for IIOControlCode {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IIOControlCode, windows_core::IUnknown, windows_core::IInspectable);
impl IIOControlCode {
    pub fn AccessMode(&self) -> windows_core::Result<IOControlAccessMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BufferingMethod(&self) -> windows_core::Result<IOControlBufferingMethod> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferingMethod)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Function(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Function)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceType(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ControlCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControlCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IIOControlCode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IIOControlCode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AccessMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IOControlAccessMode) -> windows_core::HRESULT,
    pub BufferingMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IOControlBufferingMethod) -> windows_core::HRESULT,
    pub Function: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub DeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ControlCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IIOControlCodeFactory, IIOControlCodeFactory_Vtbl, 0x856a7cf0_4c11_44ae_afc6_b8d4a212788f);
impl windows_core::RuntimeType for IIOControlCodeFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IIOControlCodeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateIOControlCode: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, IOControlAccessMode, IOControlBufferingMethod, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKnownDeviceTypesStatics, IKnownDeviceTypesStatics_Vtbl, 0xee5479c2_5448_45da_ad1b_24948c239094);
impl windows_core::RuntimeType for IKnownDeviceTypesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IKnownDeviceTypesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Unknown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CustomDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CustomDevice, windows_core::IUnknown, windows_core::IInspectable);
impl CustomDevice {
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
    #[cfg(feature = "Storage_Streams")]
    pub fn SendIOControlAsync<P0, P1, P2>(&self, iocontrolcode: P0, inputbuffer: P1, outputbuffer: P2) -> windows_core::Result<super::super::Foundation::IAsyncOperation<u32>>
    where
        P0: windows_core::Param<IIOControlCode>,
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
        P2: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SendIOControlAsync)(windows_core::Interface::as_raw(this), iocontrolcode.param().abi(), inputbuffer.param().abi(), outputbuffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TrySendIOControlAsync<P0, P1, P2>(&self, iocontrolcode: P0, inputbuffer: P1, outputbuffer: P2) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<IIOControlCode>,
        P1: windows_core::Param<super::super::Storage::Streams::IBuffer>,
        P2: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySendIOControlAsync)(windows_core::Interface::as_raw(this), iocontrolcode.param().abi(), inputbuffer.param().abi(), outputbuffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeviceSelector(classguid: windows_core::GUID) -> windows_core::Result<windows_core::HSTRING> {
        Self::ICustomDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), classguid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING, desiredaccess: DeviceAccessMode, sharingmode: DeviceSharingMode) -> windows_core::Result<super::super::Foundation::IAsyncOperation<CustomDevice>> {
        Self::ICustomDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), desiredaccess, sharingmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICustomDeviceStatics<R, F: FnOnce(&ICustomDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CustomDevice, ICustomDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CustomDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICustomDevice>();
}
unsafe impl windows_core::Interface for CustomDevice {
    type Vtable = ICustomDevice_Vtbl;
    const IID: windows_core::GUID = <ICustomDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CustomDevice {
    const NAME: &'static str = "Windows.Devices.Custom.CustomDevice";
}
unsafe impl Send for CustomDevice {}
unsafe impl Sync for CustomDevice {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IOControlCode(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(IOControlCode, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IOControlCode, IIOControlCode);
impl IOControlCode {
    pub fn AccessMode(&self) -> windows_core::Result<IOControlAccessMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BufferingMethod(&self) -> windows_core::Result<IOControlBufferingMethod> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferingMethod)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Function(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Function)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceType(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeviceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ControlCode(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ControlCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateIOControlCode(devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod) -> windows_core::Result<IOControlCode> {
        Self::IIOControlCodeFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateIOControlCode)(windows_core::Interface::as_raw(this), devicetype, function, accessmode, bufferingmethod, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IIOControlCodeFactory<R, F: FnOnce(&IIOControlCodeFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<IOControlCode, IIOControlCodeFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for IOControlCode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIOControlCode>();
}
unsafe impl windows_core::Interface for IOControlCode {
    type Vtable = IIOControlCode_Vtbl;
    const IID: windows_core::GUID = <IIOControlCode as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for IOControlCode {
    const NAME: &'static str = "Windows.Devices.Custom.IOControlCode";
}
unsafe impl Send for IOControlCode {}
unsafe impl Sync for IOControlCode {}
pub struct KnownDeviceTypes;
impl KnownDeviceTypes {
    pub fn Unknown() -> windows_core::Result<u16> {
        Self::IKnownDeviceTypesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unknown)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownDeviceTypesStatics<R, F: FnOnce(&IKnownDeviceTypesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KnownDeviceTypes, IKnownDeviceTypesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for KnownDeviceTypes {
    const NAME: &'static str = "Windows.Devices.Custom.KnownDeviceTypes";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeviceAccessMode(pub i32);
impl DeviceAccessMode {
    pub const Read: Self = Self(0i32);
    pub const Write: Self = Self(1i32);
    pub const ReadWrite: Self = Self(2i32);
}
impl windows_core::TypeKind for DeviceAccessMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeviceAccessMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeviceAccessMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeviceAccessMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.DeviceAccessMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeviceSharingMode(pub i32);
impl DeviceSharingMode {
    pub const Shared: Self = Self(0i32);
    pub const Exclusive: Self = Self(1i32);
}
impl windows_core::TypeKind for DeviceSharingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeviceSharingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeviceSharingMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeviceSharingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.DeviceSharingMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IOControlAccessMode(pub i32);
impl IOControlAccessMode {
    pub const Any: Self = Self(0i32);
    pub const Read: Self = Self(1i32);
    pub const Write: Self = Self(2i32);
    pub const ReadWrite: Self = Self(3i32);
}
impl windows_core::TypeKind for IOControlAccessMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IOControlAccessMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IOControlAccessMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for IOControlAccessMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.IOControlAccessMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IOControlBufferingMethod(pub i32);
impl IOControlBufferingMethod {
    pub const Buffered: Self = Self(0i32);
    pub const DirectInput: Self = Self(1i32);
    pub const DirectOutput: Self = Self(2i32);
    pub const Neither: Self = Self(3i32);
}
impl windows_core::TypeKind for IOControlBufferingMethod {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IOControlBufferingMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IOControlBufferingMethod").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for IOControlBufferingMethod {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.IOControlBufferingMethod;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
