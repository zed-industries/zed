windows_core::imp::define_interface!(IUserDataAvailabilityStateChangedEventArgs, IUserDataAvailabilityStateChangedEventArgs_Vtbl, 0xa76582c9_06a2_4273_a803_834c9f87fbeb);
impl windows_core::RuntimeType for IUserDataAvailabilityStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataAvailabilityStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataBufferUnprotectResult, IUserDataBufferUnprotectResult_Vtbl, 0x8efd0e90_fa9a_46a4_a377_01cebf1e74d8);
impl windows_core::RuntimeType for IUserDataBufferUnprotectResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataBufferUnprotectResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataBufferUnprotectStatus) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub UnprotectedBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    UnprotectedBuffer: usize,
}
windows_core::imp::define_interface!(IUserDataProtectionManager, IUserDataProtectionManager_Vtbl, 0x1f13237d_b42e_4a88_9480_0f240924c876);
impl windows_core::RuntimeType for IUserDataProtectionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataProtectionManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub ProtectStorageItemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, UserDataAvailability, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    ProtectStorageItemAsync: usize,
    #[cfg(feature = "Storage")]
    pub GetStorageItemProtectionInfoAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    GetStorageItemProtectionInfoAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ProtectBufferAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, UserDataAvailability, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ProtectBufferAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub UnprotectBufferAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    UnprotectBufferAsync: usize,
    pub IsContinuedDataAvailabilityExpected: unsafe extern "system" fn(*mut core::ffi::c_void, UserDataAvailability, *mut bool) -> windows_core::HRESULT,
    pub DataAvailabilityStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDataAvailabilityStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUserDataProtectionManagerStatics, IUserDataProtectionManagerStatics_Vtbl, 0x977780e8_6dce_4fae_af85_782ac2cf4572);
impl windows_core::RuntimeType for IUserDataProtectionManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataProtectionManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryGetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub TryGetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    TryGetForUser: usize,
}
windows_core::imp::define_interface!(IUserDataStorageItemProtectionInfo, IUserDataStorageItemProtectionInfo_Vtbl, 0x5b6680f6_e87f_40a1_b19d_a6187a0c662f);
impl windows_core::RuntimeType for IUserDataStorageItemProtectionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUserDataStorageItemProtectionInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Availability: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UserDataAvailability) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UserDataAvailabilityStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataAvailabilityStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataAvailabilityStateChangedEventArgs {
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserDataAvailabilityStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataAvailabilityStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for UserDataAvailabilityStateChangedEventArgs {
    type Vtable = IUserDataAvailabilityStateChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IUserDataAvailabilityStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataAvailabilityStateChangedEventArgs {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataAvailabilityStateChangedEventArgs";
}
unsafe impl Send for UserDataAvailabilityStateChangedEventArgs {}
unsafe impl Sync for UserDataAvailabilityStateChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UserDataBufferUnprotectResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataBufferUnprotectResult, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataBufferUnprotectResult {
    pub fn Status(&self) -> windows_core::Result<UserDataBufferUnprotectStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn UnprotectedBuffer(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnprotectedBuffer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UserDataBufferUnprotectResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataBufferUnprotectResult>();
}
unsafe impl windows_core::Interface for UserDataBufferUnprotectResult {
    type Vtable = IUserDataBufferUnprotectResult_Vtbl;
    const IID: windows_core::GUID = <IUserDataBufferUnprotectResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataBufferUnprotectResult {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataBufferUnprotectResult";
}
unsafe impl Send for UserDataBufferUnprotectResult {}
unsafe impl Sync for UserDataBufferUnprotectResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UserDataProtectionManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataProtectionManager, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataProtectionManager {
    #[cfg(feature = "Storage")]
    pub fn ProtectStorageItemAsync<P0>(&self, storageitem: P0, availability: UserDataAvailability) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionStatus>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectStorageItemAsync)(windows_core::Interface::as_raw(this), storageitem.param().abi(), availability, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn GetStorageItemProtectionInfoAsync<P0>(&self, storageitem: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataStorageItemProtectionInfo>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStorageItemProtectionInfoAsync)(windows_core::Interface::as_raw(this), storageitem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ProtectBufferAsync<P0>(&self, unprotectedbuffer: P0, availability: UserDataAvailability) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectBufferAsync)(windows_core::Interface::as_raw(this), unprotectedbuffer.param().abi(), availability, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn UnprotectBufferAsync<P0>(&self, protectedbuffer: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<UserDataBufferUnprotectResult>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnprotectBufferAsync)(windows_core::Interface::as_raw(this), protectedbuffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsContinuedDataAvailabilityExpected(&self, availability: UserDataAvailability) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsContinuedDataAvailabilityExpected)(windows_core::Interface::as_raw(this), availability, &mut result__).map(|| result__)
        }
    }
    pub fn DataAvailabilityStateChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<UserDataProtectionManager, UserDataAvailabilityStateChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataAvailabilityStateChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDataAvailabilityStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDataAvailabilityStateChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TryGetDefault() -> windows_core::Result<UserDataProtectionManager> {
        Self::IUserDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn TryGetForUser<P0>(user: P0) -> windows_core::Result<UserDataProtectionManager>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IUserDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IUserDataProtectionManagerStatics<R, F: FnOnce(&IUserDataProtectionManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UserDataProtectionManager, IUserDataProtectionManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UserDataProtectionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataProtectionManager>();
}
unsafe impl windows_core::Interface for UserDataProtectionManager {
    type Vtable = IUserDataProtectionManager_Vtbl;
    const IID: windows_core::GUID = <IUserDataProtectionManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataProtectionManager {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataProtectionManager";
}
unsafe impl Send for UserDataProtectionManager {}
unsafe impl Sync for UserDataProtectionManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct UserDataStorageItemProtectionInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UserDataStorageItemProtectionInfo, windows_core::IUnknown, windows_core::IInspectable);
impl UserDataStorageItemProtectionInfo {
    pub fn Availability(&self) -> windows_core::Result<UserDataAvailability> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Availability)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for UserDataStorageItemProtectionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUserDataStorageItemProtectionInfo>();
}
unsafe impl windows_core::Interface for UserDataStorageItemProtectionInfo {
    type Vtable = IUserDataStorageItemProtectionInfo_Vtbl;
    const IID: windows_core::GUID = <IUserDataStorageItemProtectionInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UserDataStorageItemProtectionInfo {
    const NAME: &'static str = "Windows.Security.DataProtection.UserDataStorageItemProtectionInfo";
}
unsafe impl Send for UserDataStorageItemProtectionInfo {}
unsafe impl Sync for UserDataStorageItemProtectionInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataAvailability(pub i32);
impl UserDataAvailability {
    pub const Always: Self = Self(0i32);
    pub const AfterFirstUnlock: Self = Self(1i32);
    pub const WhileUnlocked: Self = Self(2i32);
}
impl windows_core::TypeKind for UserDataAvailability {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataAvailability {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataAvailability").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataAvailability {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.DataProtection.UserDataAvailability;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataBufferUnprotectStatus(pub i32);
impl UserDataBufferUnprotectStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Unavailable: Self = Self(1i32);
}
impl windows_core::TypeKind for UserDataBufferUnprotectStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataBufferUnprotectStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataBufferUnprotectStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataBufferUnprotectStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.DataProtection.UserDataBufferUnprotectStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UserDataStorageItemProtectionStatus(pub i32);
impl UserDataStorageItemProtectionStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NotProtectable: Self = Self(1i32);
    pub const DataUnavailable: Self = Self(2i32);
}
impl windows_core::TypeKind for UserDataStorageItemProtectionStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UserDataStorageItemProtectionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UserDataStorageItemProtectionStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for UserDataStorageItemProtectionStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.DataProtection.UserDataStorageItemProtectionStatus;i4)");
}
