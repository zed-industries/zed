windows_core::imp::define_interface!(IGameBarStatics, IGameBarStatics_Vtbl, 0x1db9a292_cc78_4173_be45_b61e67283ea7);
impl windows_core::RuntimeType for IGameBarStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameBarStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VisibilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVisibilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub IsInputRedirectedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveIsInputRedirectedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInputRedirected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameChatMessageReceivedEventArgs, IGameChatMessageReceivedEventArgs_Vtbl, 0xa28201f1_3fb9_4e42_a403_7afce2023b1e);
impl windows_core::RuntimeType for IGameChatMessageReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameChatMessageReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AppId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AppDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SenderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Origin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameChatMessageOrigin) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameChatOverlay, IGameChatOverlay_Vtbl, 0xfbc64865_f6fc_4a48_ae07_03ac6ed43704);
impl windows_core::RuntimeType for IGameChatOverlay {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameChatOverlay_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DesiredPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GameChatOverlayPosition) -> windows_core::HRESULT,
    pub SetDesiredPosition: unsafe extern "system" fn(*mut core::ffi::c_void, GameChatOverlayPosition) -> windows_core::HRESULT,
    pub AddMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, GameChatMessageOrigin) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameChatOverlayMessageSource, IGameChatOverlayMessageSource_Vtbl, 0x1e177397_59fb_4f4f_8e9a_80acf817743c);
impl windows_core::RuntimeType for IGameChatOverlayMessageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameChatOverlayMessageSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SetDelayBeforeClosingAfterMessageReceived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameChatOverlayStatics, IGameChatOverlayStatics_Vtbl, 0x89acf614_7867_49f7_9687_25d9dbf444d1);
impl windows_core::RuntimeType for IGameChatOverlayStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameChatOverlayStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameUIProviderActivatedEventArgs, IGameUIProviderActivatedEventArgs_Vtbl, 0xa7b3203e_caf7_4ded_bbd2_47de43bb6dd5);
impl windows_core::RuntimeType for IGameUIProviderActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameUIProviderActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GameUIArgs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GameUIArgs: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReportCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReportCompleted: usize,
}
pub struct GameBar;
impl GameBar {
    pub fn VisibilityChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VisibilityChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveVisibilityChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IGameBarStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveVisibilityChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn IsInputRedirectedChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::EventHandler<windows_core::IInspectable>>,
    {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInputRedirectedChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveIsInputRedirectedChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IGameBarStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveIsInputRedirectedChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn Visible() -> windows_core::Result<bool> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsInputRedirected() -> windows_core::Result<bool> {
        Self::IGameBarStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsInputRedirected)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameBarStatics<R, F: FnOnce(&IGameBarStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GameBar, IGameBarStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for GameBar {
    const NAME: &'static str = "Windows.Gaming.UI.GameBar";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameChatMessageReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameChatMessageReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl GameChatMessageReceivedEventArgs {
    pub fn AppId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppDisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppDisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SenderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SenderName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Message)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Origin(&self) -> windows_core::Result<GameChatMessageOrigin> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Origin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GameChatMessageReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameChatMessageReceivedEventArgs>();
}
unsafe impl windows_core::Interface for GameChatMessageReceivedEventArgs {
    type Vtable = IGameChatMessageReceivedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGameChatMessageReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameChatMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Gaming.UI.GameChatMessageReceivedEventArgs";
}
unsafe impl Send for GameChatMessageReceivedEventArgs {}
unsafe impl Sync for GameChatMessageReceivedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameChatOverlay(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameChatOverlay, windows_core::IUnknown, windows_core::IInspectable);
impl GameChatOverlay {
    pub fn DesiredPosition(&self) -> windows_core::Result<GameChatOverlayPosition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredPosition(&self, value: GameChatOverlayPosition) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDesiredPosition)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AddMessage(&self, sender: &windows_core::HSTRING, message: &windows_core::HSTRING, origin: GameChatMessageOrigin) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddMessage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sender), core::mem::transmute_copy(message), origin).ok() }
    }
    pub fn GetDefault() -> windows_core::Result<GameChatOverlay> {
        Self::IGameChatOverlayStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGameChatOverlayStatics<R, F: FnOnce(&IGameChatOverlayStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GameChatOverlay, IGameChatOverlayStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GameChatOverlay {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameChatOverlay>();
}
unsafe impl windows_core::Interface for GameChatOverlay {
    type Vtable = IGameChatOverlay_Vtbl;
    const IID: windows_core::GUID = <IGameChatOverlay as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameChatOverlay {
    const NAME: &'static str = "Windows.Gaming.UI.GameChatOverlay";
}
unsafe impl Send for GameChatOverlay {}
unsafe impl Sync for GameChatOverlay {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameChatOverlayMessageSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameChatOverlayMessageSource, windows_core::IUnknown, windows_core::IInspectable);
impl GameChatOverlayMessageSource {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GameChatOverlayMessageSource, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MessageReceived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GameChatOverlayMessageSource, GameChatMessageReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MessageReceived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMessageReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMessageReceived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetDelayBeforeClosingAfterMessageReceived(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDelayBeforeClosingAfterMessageReceived)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for GameChatOverlayMessageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameChatOverlayMessageSource>();
}
unsafe impl windows_core::Interface for GameChatOverlayMessageSource {
    type Vtable = IGameChatOverlayMessageSource_Vtbl;
    const IID: windows_core::GUID = <IGameChatOverlayMessageSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameChatOverlayMessageSource {
    const NAME: &'static str = "Windows.Gaming.UI.GameChatOverlayMessageSource";
}
unsafe impl Send for GameChatOverlayMessageSource {}
unsafe impl Sync for GameChatOverlayMessageSource {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameUIProviderActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameUIProviderActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
windows_core::imp::required_hierarchy!(GameUIProviderActivatedEventArgs, super::super::ApplicationModel::Activation::IActivatedEventArgs);
impl GameUIProviderActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousExecutionState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &windows_core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplashScreen)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GameUIArgs(&self) -> windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GameUIArgs)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReportCompleted<P0>(&self, results: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::ValueSet>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReportCompleted)(windows_core::Interface::as_raw(this), results.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for GameUIProviderActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameUIProviderActivatedEventArgs>();
}
unsafe impl windows_core::Interface for GameUIProviderActivatedEventArgs {
    type Vtable = IGameUIProviderActivatedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IGameUIProviderActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameUIProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.Gaming.UI.GameUIProviderActivatedEventArgs";
}
unsafe impl Send for GameUIProviderActivatedEventArgs {}
unsafe impl Sync for GameUIProviderActivatedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GameChatMessageOrigin(pub i32);
impl GameChatMessageOrigin {
    pub const Voice: Self = Self(0i32);
    pub const Text: Self = Self(1i32);
}
impl windows_core::TypeKind for GameChatMessageOrigin {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GameChatMessageOrigin {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GameChatMessageOrigin").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GameChatMessageOrigin {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.UI.GameChatMessageOrigin;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GameChatOverlayPosition(pub i32);
impl GameChatOverlayPosition {
    pub const BottomCenter: Self = Self(0i32);
    pub const BottomLeft: Self = Self(1i32);
    pub const BottomRight: Self = Self(2i32);
    pub const MiddleRight: Self = Self(3i32);
    pub const MiddleLeft: Self = Self(4i32);
    pub const TopCenter: Self = Self(5i32);
    pub const TopLeft: Self = Self(6i32);
    pub const TopRight: Self = Self(7i32);
}
impl windows_core::TypeKind for GameChatOverlayPosition {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GameChatOverlayPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GameChatOverlayPosition").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GameChatOverlayPosition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Gaming.UI.GameChatOverlayPosition;i4)");
}
