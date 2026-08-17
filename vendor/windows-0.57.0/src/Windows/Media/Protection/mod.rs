#[cfg(feature = "Media_Protection_PlayReady")]
pub mod PlayReady;
windows_core::imp::define_interface!(IComponentLoadFailedEventArgs, IComponentLoadFailedEventArgs_Vtbl, 0x95972e93_7746_417e_8495_f031bbc5862c);
impl windows_core::RuntimeType for IComponentLoadFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IComponentLoadFailedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Information: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Completion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComponentRenewalStatics, IComponentRenewalStatics_Vtbl, 0x6ffbcd67_b795_48c5_8b7b_a7c4efe202e3);
impl windows_core::RuntimeType for IComponentRenewalStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IComponentRenewalStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RenewSystemComponentsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHdcpSession, IHdcpSession_Vtbl, 0x718845e9_64d7_426d_809b_1be461941a2a);
impl windows_core::RuntimeType for IHdcpSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHdcpSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsEffectiveProtectionAtLeast: unsafe extern "system" fn(*mut core::ffi::c_void, HdcpProtection, *mut bool) -> windows_core::HRESULT,
    pub GetEffectiveProtection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDesiredMinProtectionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, HdcpProtection, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProtectionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveProtectionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaProtectionManager, IMediaProtectionManager_Vtbl, 0x45694947_c741_434b_a79e_474c12d93d2f);
impl windows_core::RuntimeType for IMediaProtectionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaProtectionManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServiceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveServiceRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RebootNeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRebootNeeded: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ComponentLoadFailed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveComponentLoadFailed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IMediaProtectionPMPServer, IMediaProtectionPMPServer_Vtbl, 0x0c111226_7b26_4d31_95bb_9c1b08ef7fc0);
impl windows_core::RuntimeType for IMediaProtectionPMPServer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaProtectionPMPServer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
windows_core::imp::define_interface!(IMediaProtectionPMPServerFactory, IMediaProtectionPMPServerFactory_Vtbl, 0x602c8e5e_f7d2_487e_af91_dbc4252b2182);
impl windows_core::RuntimeType for IMediaProtectionPMPServerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaProtectionPMPServerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreatePMPServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreatePMPServer: usize,
}
windows_core::imp::define_interface!(IMediaProtectionServiceCompletion, IMediaProtectionServiceCompletion_Vtbl, 0x8b5cca18_cfd5_44ee_a2ed_df76010c14b5);
impl windows_core::RuntimeType for IMediaProtectionServiceCompletion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaProtectionServiceCompletion_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaProtectionServiceRequest, IMediaProtectionServiceRequest_Vtbl, 0xb1de0ea6_2094_478d_87a4_8b95200f85c6);
impl core::ops::Deref for IMediaProtectionServiceRequest {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMediaProtectionServiceRequest, windows_core::IUnknown, windows_core::IInspectable);
impl IMediaProtectionServiceRequest {
    pub fn ProtectionSystem(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionSystem)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Type(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IMediaProtectionServiceRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaProtectionServiceRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ProtectionSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProtectionCapabilities, IProtectionCapabilities_Vtbl, 0xc7ac5d7e_7480_4d29_a464_7bcd913dd8e4);
impl windows_core::RuntimeType for IProtectionCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IProtectionCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsTypeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut ProtectionCapabilityResult) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRevocationAndRenewalInformation, IRevocationAndRenewalInformation_Vtbl, 0xf3a1937b_2501_439e_a6e7_6fc95e175fcf);
impl windows_core::RuntimeType for IRevocationAndRenewalInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRevocationAndRenewalInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
windows_core::imp::define_interface!(IRevocationAndRenewalItem, IRevocationAndRenewalItem_Vtbl, 0x3099c20c_3cf0_49ea_902d_caf32d2dde2c);
impl windows_core::RuntimeType for IRevocationAndRenewalItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRevocationAndRenewalItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reasons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RevocationAndRenewalReasons) -> windows_core::HRESULT,
    pub HeaderHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PublicKeyHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RenewalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServiceRequestedEventArgs, IServiceRequestedEventArgs_Vtbl, 0x34283baf_abb4_4fc1_bd89_93f106573a49);
impl windows_core::RuntimeType for IServiceRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IServiceRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Completion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServiceRequestedEventArgs2, IServiceRequestedEventArgs2_Vtbl, 0x553c69d6_fafe_4128_8dfa_130e398a13a7);
impl windows_core::RuntimeType for IServiceRequestedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IServiceRequestedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Playback")]
    pub MediaPlaybackItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    MediaPlaybackItem: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ComponentLoadFailedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ComponentLoadFailedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ComponentLoadFailedEventArgs {
    pub fn Information(&self) -> windows_core::Result<RevocationAndRenewalInformation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Information)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Completion(&self) -> windows_core::Result<MediaProtectionServiceCompletion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ComponentLoadFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IComponentLoadFailedEventArgs>();
}
unsafe impl windows_core::Interface for ComponentLoadFailedEventArgs {
    type Vtable = IComponentLoadFailedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IComponentLoadFailedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ComponentLoadFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.ComponentLoadFailedEventArgs";
}
unsafe impl Send for ComponentLoadFailedEventArgs {}
unsafe impl Sync for ComponentLoadFailedEventArgs {}
pub struct ComponentRenewal;
impl ComponentRenewal {
    pub fn RenewSystemComponentsAsync<P0>(information: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<RenewalStatus, u32>>
    where
        P0: windows_core::Param<RevocationAndRenewalInformation>,
    {
        Self::IComponentRenewalStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenewSystemComponentsAsync)(windows_core::Interface::as_raw(this), information.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IComponentRenewalStatics<R, F: FnOnce(&IComponentRenewalStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ComponentRenewal, IComponentRenewalStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ComponentRenewal {
    const NAME: &'static str = "Windows.Media.Protection.ComponentRenewal";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HdcpSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HdcpSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HdcpSession, super::super::Foundation::IClosable);
impl HdcpSession {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HdcpSession, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsEffectiveProtectionAtLeast(&self, protection: HdcpProtection) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEffectiveProtectionAtLeast)(windows_core::Interface::as_raw(this), protection, &mut result__).map(|| result__)
        }
    }
    pub fn GetEffectiveProtection(&self) -> windows_core::Result<super::super::Foundation::IReference<HdcpProtection>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetEffectiveProtection)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDesiredMinProtectionAsync(&self, protection: HdcpProtection) -> windows_core::Result<super::super::Foundation::IAsyncOperation<HdcpSetProtectionResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetDesiredMinProtectionAsync)(windows_core::Interface::as_raw(this), protection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ProtectionChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<HdcpSession, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectionChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveProtectionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveProtectionChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for HdcpSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHdcpSession>();
}
unsafe impl windows_core::Interface for HdcpSession {
    type Vtable = IHdcpSession_Vtbl;
    const IID: windows_core::GUID = <IHdcpSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HdcpSession {
    const NAME: &'static str = "Windows.Media.Protection.HdcpSession";
}
unsafe impl Send for HdcpSession {}
unsafe impl Sync for HdcpSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaProtectionManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaProtectionManager, windows_core::IUnknown, windows_core::IInspectable);
impl MediaProtectionManager {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaProtectionManager, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ServiceRequested<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<ServiceRequestedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ServiceRequested)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveServiceRequested(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveServiceRequested)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn RebootNeeded<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<RebootNeededEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RebootNeeded)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRebootNeeded(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRebootNeeded)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn ComponentLoadFailed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<ComponentLoadFailedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ComponentLoadFailed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveComponentLoadFailed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveComponentLoadFailed)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaProtectionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaProtectionManager>();
}
unsafe impl windows_core::Interface for MediaProtectionManager {
    type Vtable = IMediaProtectionManager_Vtbl;
    const IID: windows_core::GUID = <IMediaProtectionManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaProtectionManager {
    const NAME: &'static str = "Windows.Media.Protection.MediaProtectionManager";
}
unsafe impl Send for MediaProtectionManager {}
unsafe impl Sync for MediaProtectionManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaProtectionPMPServer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaProtectionPMPServer, windows_core::IUnknown, windows_core::IInspectable);
impl MediaProtectionPMPServer {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreatePMPServer<P0>(pproperties: P0) -> windows_core::Result<MediaProtectionPMPServer>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        Self::IMediaProtectionPMPServerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePMPServer)(windows_core::Interface::as_raw(this), pproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMediaProtectionPMPServerFactory<R, F: FnOnce(&IMediaProtectionPMPServerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaProtectionPMPServer, IMediaProtectionPMPServerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MediaProtectionPMPServer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaProtectionPMPServer>();
}
unsafe impl windows_core::Interface for MediaProtectionPMPServer {
    type Vtable = IMediaProtectionPMPServer_Vtbl;
    const IID: windows_core::GUID = <IMediaProtectionPMPServer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaProtectionPMPServer {
    const NAME: &'static str = "Windows.Media.Protection.MediaProtectionPMPServer";
}
unsafe impl Send for MediaProtectionPMPServer {}
unsafe impl Sync for MediaProtectionPMPServer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MediaProtectionServiceCompletion(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaProtectionServiceCompletion, windows_core::IUnknown, windows_core::IInspectable);
impl MediaProtectionServiceCompletion {
    pub fn Complete(&self, success: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this), success).ok() }
    }
}
impl windows_core::RuntimeType for MediaProtectionServiceCompletion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaProtectionServiceCompletion>();
}
unsafe impl windows_core::Interface for MediaProtectionServiceCompletion {
    type Vtable = IMediaProtectionServiceCompletion_Vtbl;
    const IID: windows_core::GUID = <IMediaProtectionServiceCompletion as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaProtectionServiceCompletion {
    const NAME: &'static str = "Windows.Media.Protection.MediaProtectionServiceCompletion";
}
unsafe impl Send for MediaProtectionServiceCompletion {}
unsafe impl Sync for MediaProtectionServiceCompletion {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ProtectionCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProtectionCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl ProtectionCapabilities {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ProtectionCapabilities, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsTypeSupported(&self, r#type: &windows_core::HSTRING, keysystem: &windows_core::HSTRING) -> windows_core::Result<ProtectionCapabilityResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTypeSupported)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(r#type), core::mem::transmute_copy(keysystem), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ProtectionCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProtectionCapabilities>();
}
unsafe impl windows_core::Interface for ProtectionCapabilities {
    type Vtable = IProtectionCapabilities_Vtbl;
    const IID: windows_core::GUID = <IProtectionCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProtectionCapabilities {
    const NAME: &'static str = "Windows.Media.Protection.ProtectionCapabilities";
}
unsafe impl Send for ProtectionCapabilities {}
unsafe impl Sync for ProtectionCapabilities {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RevocationAndRenewalInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RevocationAndRenewalInformation, windows_core::IUnknown, windows_core::IInspectable);
impl RevocationAndRenewalInformation {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<RevocationAndRenewalItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Items)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RevocationAndRenewalInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRevocationAndRenewalInformation>();
}
unsafe impl windows_core::Interface for RevocationAndRenewalInformation {
    type Vtable = IRevocationAndRenewalInformation_Vtbl;
    const IID: windows_core::GUID = <IRevocationAndRenewalInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RevocationAndRenewalInformation {
    const NAME: &'static str = "Windows.Media.Protection.RevocationAndRenewalInformation";
}
unsafe impl Send for RevocationAndRenewalInformation {}
unsafe impl Sync for RevocationAndRenewalInformation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RevocationAndRenewalItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RevocationAndRenewalItem, windows_core::IUnknown, windows_core::IInspectable);
impl RevocationAndRenewalItem {
    pub fn Reasons(&self) -> windows_core::Result<RevocationAndRenewalReasons> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reasons)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HeaderHash(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeaderHash)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PublicKeyHash(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PublicKeyHash)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RenewalId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenewalId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RevocationAndRenewalItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRevocationAndRenewalItem>();
}
unsafe impl windows_core::Interface for RevocationAndRenewalItem {
    type Vtable = IRevocationAndRenewalItem_Vtbl;
    const IID: windows_core::GUID = <IRevocationAndRenewalItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RevocationAndRenewalItem {
    const NAME: &'static str = "Windows.Media.Protection.RevocationAndRenewalItem";
}
unsafe impl Send for RevocationAndRenewalItem {}
unsafe impl Sync for RevocationAndRenewalItem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ServiceRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ServiceRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ServiceRequestedEventArgs {
    pub fn Request(&self) -> windows_core::Result<IMediaProtectionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Completion(&self) -> windows_core::Result<MediaProtectionServiceCompletion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Playback")]
    pub fn MediaPlaybackItem(&self) -> windows_core::Result<super::Playback::MediaPlaybackItem> {
        let this = &windows_core::Interface::cast::<IServiceRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MediaPlaybackItem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ServiceRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IServiceRequestedEventArgs>();
}
unsafe impl windows_core::Interface for ServiceRequestedEventArgs {
    type Vtable = IServiceRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IServiceRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ServiceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.ServiceRequestedEventArgs";
}
unsafe impl Send for ServiceRequestedEventArgs {}
unsafe impl Sync for ServiceRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GraphicsTrustStatus(pub i32);
impl GraphicsTrustStatus {
    pub const TrustNotRequired: Self = Self(0i32);
    pub const TrustEstablished: Self = Self(1i32);
    pub const EnvironmentNotSupported: Self = Self(2i32);
    pub const DriverNotSupported: Self = Self(3i32);
    pub const DriverSigningFailure: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
impl windows_core::TypeKind for GraphicsTrustStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GraphicsTrustStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GraphicsTrustStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GraphicsTrustStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.GraphicsTrustStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HdcpProtection(pub i32);
impl HdcpProtection {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const OnWithTypeEnforcement: Self = Self(2i32);
}
impl windows_core::TypeKind for HdcpProtection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HdcpProtection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HdcpProtection").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HdcpProtection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.HdcpProtection;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HdcpSetProtectionResult(pub i32);
impl HdcpSetProtectionResult {
    pub const Success: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl windows_core::TypeKind for HdcpSetProtectionResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HdcpSetProtectionResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HdcpSetProtectionResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HdcpSetProtectionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.HdcpSetProtectionResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ProtectionCapabilityResult(pub i32);
impl ProtectionCapabilityResult {
    pub const NotSupported: Self = Self(0i32);
    pub const Maybe: Self = Self(1i32);
    pub const Probably: Self = Self(2i32);
}
impl windows_core::TypeKind for ProtectionCapabilityResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ProtectionCapabilityResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ProtectionCapabilityResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ProtectionCapabilityResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.ProtectionCapabilityResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RenewalStatus(pub i32);
impl RenewalStatus {
    pub const NotStarted: Self = Self(0i32);
    pub const UpdatesInProgress: Self = Self(1i32);
    pub const UserCancelled: Self = Self(2i32);
    pub const AppComponentsMayNeedUpdating: Self = Self(3i32);
    pub const NoComponentsFound: Self = Self(4i32);
}
impl windows_core::TypeKind for RenewalStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RenewalStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RenewalStatus").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for RenewalStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.RenewalStatus;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RevocationAndRenewalReasons(pub u32);
impl RevocationAndRenewalReasons {
    pub const UserModeComponentLoad: Self = Self(1u32);
    pub const KernelModeComponentLoad: Self = Self(2u32);
    pub const AppComponent: Self = Self(4u32);
    pub const GlobalRevocationListLoadFailed: Self = Self(16u32);
    pub const InvalidGlobalRevocationListSignature: Self = Self(32u32);
    pub const GlobalRevocationListAbsent: Self = Self(4096u32);
    pub const ComponentRevoked: Self = Self(8192u32);
    pub const InvalidComponentCertificateExtendedKeyUse: Self = Self(16384u32);
    pub const ComponentCertificateRevoked: Self = Self(32768u32);
    pub const InvalidComponentCertificateRoot: Self = Self(65536u32);
    pub const ComponentHighSecurityCertificateRevoked: Self = Self(131072u32);
    pub const ComponentLowSecurityCertificateRevoked: Self = Self(262144u32);
    pub const BootDriverVerificationFailed: Self = Self(1048576u32);
    pub const ComponentSignedWithTestCertificate: Self = Self(16777216u32);
    pub const EncryptionFailure: Self = Self(268435456u32);
}
impl windows_core::TypeKind for RevocationAndRenewalReasons {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RevocationAndRenewalReasons {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RevocationAndRenewalReasons").field(&self.0).finish()
    }
}
impl RevocationAndRenewalReasons {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RevocationAndRenewalReasons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RevocationAndRenewalReasons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RevocationAndRenewalReasons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RevocationAndRenewalReasons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RevocationAndRenewalReasons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for RevocationAndRenewalReasons {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.RevocationAndRenewalReasons;u4)");
}
windows_core::imp::define_interface!(ComponentLoadFailedEventHandler, ComponentLoadFailedEventHandler_Vtbl, 0x95da643c_6db9_424b_86ca_091af432081c);
impl ComponentLoadFailedEventHandler {
    pub fn new<F: FnMut(Option<&MediaProtectionManager>, Option<&ComponentLoadFailedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = ComponentLoadFailedEventHandlerBox::<F> { vtable: &ComponentLoadFailedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaProtectionManager>,
        P1: windows_core::Param<ComponentLoadFailedEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
struct ComponentLoadFailedEventHandlerBox<F: FnMut(Option<&MediaProtectionManager>, Option<&ComponentLoadFailedEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const ComponentLoadFailedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&MediaProtectionManager>, Option<&ComponentLoadFailedEventArgs>) -> windows_core::Result<()> + Send + 'static> ComponentLoadFailedEventHandlerBox<F> {
    const VTABLE: ComponentLoadFailedEventHandler_Vtbl = ComponentLoadFailedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <ComponentLoadFailedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&e)).into()
    }
}
impl windows_core::RuntimeType for ComponentLoadFailedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ComponentLoadFailedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(RebootNeededEventHandler, RebootNeededEventHandler_Vtbl, 0x64e12a45_973b_4a3a_b260_91898a49a82c);
impl RebootNeededEventHandler {
    pub fn new<F: FnMut(Option<&MediaProtectionManager>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = RebootNeededEventHandlerBox::<F> { vtable: &RebootNeededEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaProtectionManager>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi()).ok() }
    }
}
#[repr(C)]
struct RebootNeededEventHandlerBox<F: FnMut(Option<&MediaProtectionManager>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const RebootNeededEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&MediaProtectionManager>) -> windows_core::Result<()> + Send + 'static> RebootNeededEventHandlerBox<F> {
    const VTABLE: RebootNeededEventHandler_Vtbl = RebootNeededEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <RebootNeededEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
impl windows_core::RuntimeType for RebootNeededEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct RebootNeededEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ServiceRequestedEventHandler, ServiceRequestedEventHandler_Vtbl, 0xd2d690ba_cac9_48e1_95c0_d38495a84055);
impl ServiceRequestedEventHandler {
    pub fn new<F: FnMut(Option<&MediaProtectionManager>, Option<&ServiceRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = ServiceRequestedEventHandlerBox::<F> { vtable: &ServiceRequestedEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MediaProtectionManager>,
        P1: windows_core::Param<ServiceRequestedEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
struct ServiceRequestedEventHandlerBox<F: FnMut(Option<&MediaProtectionManager>, Option<&ServiceRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const ServiceRequestedEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&MediaProtectionManager>, Option<&ServiceRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static> ServiceRequestedEventHandlerBox<F> {
    const VTABLE: ServiceRequestedEventHandler_Vtbl = ServiceRequestedEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <ServiceRequestedEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&e)).into()
    }
}
impl windows_core::RuntimeType for ServiceRequestedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ServiceRequestedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
