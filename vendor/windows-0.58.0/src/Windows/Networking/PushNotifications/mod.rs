windows_core::imp::define_interface!(IPushNotificationChannel, IPushNotificationChannel_Vtbl, 0x2b28102e_ef0b_4f39_9b8a_a3c194de7081);
impl windows_core::RuntimeType for IPushNotificationChannel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPushNotificationChannel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PushNotificationReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePushNotificationReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPushNotificationChannelManagerForUser, IPushNotificationChannelManagerForUser_Vtbl, 0xa4c45704_1182_42c7_8890_f563c4890dc4);
impl windows_core::RuntimeType for IPushNotificationChannelManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPushNotificationChannelManagerForUser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreatePushNotificationChannelForApplicationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePushNotificationChannelForApplicationAsyncWithId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePushNotificationChannelForSecondaryTileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
windows_core::imp::define_interface!(IPushNotificationChannelManagerForUser2, IPushNotificationChannelManagerForUser2_Vtbl, 0xc38b066a_7cc1_4dac_87fd_be6e920414a4);
impl windows_core::RuntimeType for IPushNotificationChannelManagerForUser2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPushNotificationChannelManagerForUser2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId: usize,
}
windows_core::imp::define_interface!(IPushNotificationChannelManagerStatics, IPushNotificationChannelManagerStatics_Vtbl, 0x8baf9b65_77a1_4588_bd19_861529a9dcf0);
impl windows_core::RuntimeType for IPushNotificationChannelManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPushNotificationChannelManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreatePushNotificationChannelForApplicationAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePushNotificationChannelForApplicationAsyncWithId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePushNotificationChannelForSecondaryTileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPushNotificationChannelManagerStatics2, IPushNotificationChannelManagerStatics2_Vtbl, 0xb444a65d_a7e9_4b28_950e_f375a907f9df);
impl windows_core::RuntimeType for IPushNotificationChannelManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPushNotificationChannelManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
windows_core::imp::define_interface!(IPushNotificationChannelManagerStatics3, IPushNotificationChannelManagerStatics3_Vtbl, 0x4701fefe_0ede_4a3f_ae78_bfa471496925);
impl windows_core::RuntimeType for IPushNotificationChannelManagerStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPushNotificationChannelManagerStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPushNotificationChannelManagerStatics4, IPushNotificationChannelManagerStatics4_Vtbl, 0xbc540efb_7820_5a5b_9c01_b4757f774025);
impl windows_core::RuntimeType for IPushNotificationChannelManagerStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPushNotificationChannelManagerStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChannelsRevoked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveChannelsRevoked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPushNotificationChannelsRevokedEventArgs, IPushNotificationChannelsRevokedEventArgs_Vtbl, 0x20e1a24c_1a34_5beb_aae2_40c232c8c140);
impl windows_core::RuntimeType for IPushNotificationChannelsRevokedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPushNotificationChannelsRevokedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPushNotificationReceivedEventArgs, IPushNotificationReceivedEventArgs_Vtbl, 0xd1065e0c_36cd_484c_b935_0a99b753cf00);
impl windows_core::RuntimeType for IPushNotificationReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPushNotificationReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetCancel: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub NotificationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PushNotificationType) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Notifications")]
    pub ToastNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    ToastNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub TileNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    TileNotification: usize,
    #[cfg(feature = "UI_Notifications")]
    pub BadgeNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    BadgeNotification: usize,
    pub RawNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRawNotification, IRawNotification_Vtbl, 0x1a227281_3b79_42ac_9963_22ab00d4f0b7);
impl windows_core::RuntimeType for IRawNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRawNotification_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRawNotification2, IRawNotification2_Vtbl, 0xe6d0cf19_0c6f_4cdd_9424_eec5be014d26);
impl windows_core::RuntimeType for IRawNotification2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRawNotification2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Headers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Headers: usize,
    pub ChannelId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRawNotification3, IRawNotification3_Vtbl, 0x62737dde_8a73_424c_ab44_5635f40a96e5);
impl windows_core::RuntimeType for IRawNotification3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRawNotification3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub ContentBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ContentBytes: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PushNotificationChannel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PushNotificationChannel, windows_core::IUnknown, windows_core::IInspectable);
impl PushNotificationChannel {
    pub fn Uri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ExpirationTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExpirationTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PushNotificationReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<PushNotificationChannel, PushNotificationReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PushNotificationReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePushNotificationReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePushNotificationReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for PushNotificationChannel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPushNotificationChannel>();
}
unsafe impl windows_core::Interface for PushNotificationChannel {
    type Vtable = IPushNotificationChannel_Vtbl;
    const IID: windows_core::GUID = <IPushNotificationChannel as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PushNotificationChannel {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannel";
}
unsafe impl Send for PushNotificationChannel {}
unsafe impl Sync for PushNotificationChannel {}
pub struct PushNotificationChannelManager;
impl PushNotificationChannelManager {
    pub fn CreatePushNotificationChannelForApplicationAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        Self::IPushNotificationChannelManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreatePushNotificationChannelForApplicationAsyncWithId(applicationid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        Self::IPushNotificationChannelManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsyncWithId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreatePushNotificationChannelForSecondaryTileAsync(tileid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        Self::IPushNotificationChannelManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePushNotificationChannelForSecondaryTileAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tileid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> windows_core::Result<PushNotificationChannelManagerForUser>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IPushNotificationChannelManagerStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUser)(windows_core::Interface::as_raw(this), user.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefault() -> windows_core::Result<PushNotificationChannelManagerForUser> {
        Self::IPushNotificationChannelManagerStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ChannelsRevoked<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<PushNotificationChannelsRevokedEventArgs>>,
    {
        Self::IPushNotificationChannelManagerStatics4(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChannelsRevoked)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveChannelsRevoked(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IPushNotificationChannelManagerStatics4(|this| unsafe { (windows_core::Interface::vtable(this).RemoveChannelsRevoked)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics<R, F: FnOnce(&IPushNotificationChannelManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics2<R, F: FnOnce(&IPushNotificationChannelManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics3<R, F: FnOnce(&IPushNotificationChannelManagerStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPushNotificationChannelManagerStatics4<R, F: FnOnce(&IPushNotificationChannelManagerStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PushNotificationChannelManager, IPushNotificationChannelManagerStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PushNotificationChannelManager {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannelManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PushNotificationChannelManagerForUser(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PushNotificationChannelManagerForUser, windows_core::IUnknown, windows_core::IInspectable);
impl PushNotificationChannelManagerForUser {
    pub fn CreatePushNotificationChannelForApplicationAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreatePushNotificationChannelForApplicationAsyncWithId(&self, applicationid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePushNotificationChannelForApplicationAsyncWithId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(applicationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreatePushNotificationChannelForSecondaryTileAsync(&self, tileid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePushNotificationChannelForSecondaryTileAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tileid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).User)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync<P0>(&self, appserverkey: P0, channelid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IPushNotificationChannelManagerForUser2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync)(windows_core::Interface::as_raw(this), appserverkey.param().abi(), core::mem::transmute_copy(channelid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId<P0>(&self, appserverkey: P0, channelid: &windows_core::HSTRING, appid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<PushNotificationChannel>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IPushNotificationChannelManagerForUser2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId)(windows_core::Interface::as_raw(this), appserverkey.param().abi(), core::mem::transmute_copy(channelid), core::mem::transmute_copy(appid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PushNotificationChannelManagerForUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPushNotificationChannelManagerForUser>();
}
unsafe impl windows_core::Interface for PushNotificationChannelManagerForUser {
    type Vtable = IPushNotificationChannelManagerForUser_Vtbl;
    const IID: windows_core::GUID = <IPushNotificationChannelManagerForUser as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PushNotificationChannelManagerForUser {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannelManagerForUser";
}
unsafe impl Send for PushNotificationChannelManagerForUser {}
unsafe impl Sync for PushNotificationChannelManagerForUser {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PushNotificationChannelsRevokedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PushNotificationChannelsRevokedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PushNotificationChannelsRevokedEventArgs {}
impl windows_core::RuntimeType for PushNotificationChannelsRevokedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPushNotificationChannelsRevokedEventArgs>();
}
unsafe impl windows_core::Interface for PushNotificationChannelsRevokedEventArgs {
    type Vtable = IPushNotificationChannelsRevokedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPushNotificationChannelsRevokedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PushNotificationChannelsRevokedEventArgs {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationChannelsRevokedEventArgs";
}
unsafe impl Send for PushNotificationChannelsRevokedEventArgs {}
unsafe impl Sync for PushNotificationChannelsRevokedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PushNotificationReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PushNotificationReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PushNotificationReceivedEventArgs {
    pub fn SetCancel(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCancel)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Cancel(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NotificationType(&self) -> windows_core::Result<PushNotificationType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotificationType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn ToastNotification(&self) -> windows_core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToastNotification)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn TileNotification(&self) -> windows_core::Result<super::super::UI::Notifications::TileNotification> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileNotification)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Notifications")]
    pub fn BadgeNotification(&self) -> windows_core::Result<super::super::UI::Notifications::BadgeNotification> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BadgeNotification)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RawNotification(&self) -> windows_core::Result<RawNotification> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawNotification)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PushNotificationReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPushNotificationReceivedEventArgs>();
}
unsafe impl windows_core::Interface for PushNotificationReceivedEventArgs {
    type Vtable = IPushNotificationReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPushNotificationReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PushNotificationReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.PushNotifications.PushNotificationReceivedEventArgs";
}
unsafe impl Send for PushNotificationReceivedEventArgs {}
unsafe impl Sync for PushNotificationReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RawNotification(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RawNotification, windows_core::IUnknown, windows_core::IInspectable);
impl RawNotification {
    pub fn Content(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Headers(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = &windows_core::Interface::cast::<IRawNotification2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Headers)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChannelId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IRawNotification2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChannelId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentBytes(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &windows_core::Interface::cast::<IRawNotification3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentBytes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RawNotification {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRawNotification>();
}
unsafe impl windows_core::Interface for RawNotification {
    type Vtable = IRawNotification_Vtbl;
    const IID: windows_core::GUID = <IRawNotification as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RawNotification {
    const NAME: &'static str = "Windows.Networking.PushNotifications.RawNotification";
}
unsafe impl Send for RawNotification {}
unsafe impl Sync for RawNotification {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PushNotificationType(pub i32);
impl PushNotificationType {
    pub const Toast: Self = Self(0i32);
    pub const Tile: Self = Self(1i32);
    pub const Badge: Self = Self(2i32);
    pub const Raw: Self = Self(3i32);
    pub const TileFlyout: Self = Self(4i32);
}
impl windows_core::TypeKind for PushNotificationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PushNotificationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PushNotificationType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for PushNotificationType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.PushNotifications.PushNotificationType;i4)");
}
