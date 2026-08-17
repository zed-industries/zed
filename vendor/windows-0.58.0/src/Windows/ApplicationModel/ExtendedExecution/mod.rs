#[cfg(feature = "ApplicationModel_ExtendedExecution_Foreground")]
pub mod Foreground;
windows_core::imp::define_interface!(IExtendedExecutionRevokedEventArgs, IExtendedExecutionRevokedEventArgs_Vtbl, 0xbfbc9f16_63b5_4c0b_aad6_828af5373ec3);
impl windows_core::RuntimeType for IExtendedExecutionRevokedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IExtendedExecutionRevokedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ExtendedExecutionRevokedReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IExtendedExecutionSession, IExtendedExecutionSession_Vtbl, 0xaf908a2d_118b_48f1_9308_0c4fc41e200f);
impl windows_core::RuntimeType for IExtendedExecutionSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IExtendedExecutionSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ExtendedExecutionReason) -> windows_core::HRESULT,
    pub SetReason: unsafe extern "system" fn(*mut core::ffi::c_void, ExtendedExecutionReason) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PercentProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetPercentProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Revoked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveRevoked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RequestExtensionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ExtendedExecutionRevokedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ExtendedExecutionRevokedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ExtendedExecutionRevokedEventArgs {
    pub fn Reason(&self) -> windows_core::Result<ExtendedExecutionRevokedReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ExtendedExecutionRevokedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IExtendedExecutionRevokedEventArgs>();
}
unsafe impl windows_core::Interface for ExtendedExecutionRevokedEventArgs {
    type Vtable = IExtendedExecutionRevokedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IExtendedExecutionRevokedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ExtendedExecutionRevokedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionRevokedEventArgs";
}
unsafe impl Send for ExtendedExecutionRevokedEventArgs {}
unsafe impl Sync for ExtendedExecutionRevokedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ExtendedExecutionSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ExtendedExecutionSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ExtendedExecutionSession, super::super::Foundation::IClosable);
impl ExtendedExecutionSession {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ExtendedExecutionSession, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Reason(&self) -> windows_core::Result<ExtendedExecutionReason> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReason(&self, value: ExtendedExecutionReason) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetReason)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDescription(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDescription)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn PercentProgress(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PercentProgress)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPercentProgress(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPercentProgress)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Revoked<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<windows_core::IInspectable, ExtendedExecutionRevokedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Revoked)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRevoked(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveRevoked)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RequestExtensionAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<ExtendedExecutionResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestExtensionAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ExtendedExecutionSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IExtendedExecutionSession>();
}
unsafe impl windows_core::Interface for ExtendedExecutionSession {
    type Vtable = IExtendedExecutionSession_Vtbl;
    const IID: windows_core::GUID = <IExtendedExecutionSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ExtendedExecutionSession {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionSession";
}
unsafe impl Send for ExtendedExecutionSession {}
unsafe impl Sync for ExtendedExecutionSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ExtendedExecutionReason(pub i32);
impl ExtendedExecutionReason {
    pub const Unspecified: Self = Self(0i32);
    pub const LocationTracking: Self = Self(1i32);
    pub const SavingData: Self = Self(2i32);
}
impl windows_core::TypeKind for ExtendedExecutionReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ExtendedExecutionReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ExtendedExecutionReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ExtendedExecutionReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionReason;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ExtendedExecutionResult(pub i32);
impl ExtendedExecutionResult {
    pub const Allowed: Self = Self(0i32);
    pub const Denied: Self = Self(1i32);
}
impl windows_core::TypeKind for ExtendedExecutionResult {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ExtendedExecutionResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ExtendedExecutionResult").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ExtendedExecutionResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionResult;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ExtendedExecutionRevokedReason(pub i32);
impl ExtendedExecutionRevokedReason {
    pub const Resumed: Self = Self(0i32);
    pub const SystemPolicy: Self = Self(1i32);
}
impl windows_core::TypeKind for ExtendedExecutionRevokedReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ExtendedExecutionRevokedReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ExtendedExecutionRevokedReason").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ExtendedExecutionRevokedReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionRevokedReason;i4)");
}
