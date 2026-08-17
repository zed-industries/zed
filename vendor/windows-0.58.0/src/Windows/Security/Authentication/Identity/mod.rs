#[cfg(feature = "Security_Authentication_Identity_Core")]
pub mod Core;
windows_core::imp::define_interface!(IEnterpriseKeyCredentialRegistrationInfo, IEnterpriseKeyCredentialRegistrationInfo_Vtbl, 0x38321acc_672b_4823_b603_6b3c753daf97);
impl windows_core::RuntimeType for IEnterpriseKeyCredentialRegistrationInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEnterpriseKeyCredentialRegistrationInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TenantId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub TenantName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub KeyId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub KeyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnterpriseKeyCredentialRegistrationManager, IEnterpriseKeyCredentialRegistrationManager_Vtbl, 0x83f3be3f_a25f_4cba_bb8e_bdc32d03c297);
impl windows_core::RuntimeType for IEnterpriseKeyCredentialRegistrationManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEnterpriseKeyCredentialRegistrationManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRegistrationsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRegistrationsAsync: usize,
}
windows_core::imp::define_interface!(IEnterpriseKeyCredentialRegistrationManagerStatics, IEnterpriseKeyCredentialRegistrationManagerStatics_Vtbl, 0x77b85e9e_acf4_4bc0_bac2_40bb46efbb3f);
impl windows_core::RuntimeType for IEnterpriseKeyCredentialRegistrationManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EnterpriseKeyCredentialRegistrationInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EnterpriseKeyCredentialRegistrationInfo, windows_core::IUnknown, windows_core::IInspectable);
impl EnterpriseKeyCredentialRegistrationInfo {
    pub fn TenantId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TenantId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TenantName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TenantName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Subject(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subject)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KeyId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn KeyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for EnterpriseKeyCredentialRegistrationInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEnterpriseKeyCredentialRegistrationInfo>();
}
unsafe impl windows_core::Interface for EnterpriseKeyCredentialRegistrationInfo {
    type Vtable = IEnterpriseKeyCredentialRegistrationInfo_Vtbl;
    const IID: windows_core::GUID = <IEnterpriseKeyCredentialRegistrationInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EnterpriseKeyCredentialRegistrationInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo";
}
unsafe impl Send for EnterpriseKeyCredentialRegistrationInfo {}
unsafe impl Sync for EnterpriseKeyCredentialRegistrationInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct EnterpriseKeyCredentialRegistrationManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EnterpriseKeyCredentialRegistrationManager, windows_core::IUnknown, windows_core::IInspectable);
impl EnterpriseKeyCredentialRegistrationManager {
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRegistrationsAsync(&self) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRegistrationsAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Current() -> windows_core::Result<EnterpriseKeyCredentialRegistrationManager> {
        Self::IEnterpriseKeyCredentialRegistrationManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IEnterpriseKeyCredentialRegistrationManagerStatics<R, F: FnOnce(&IEnterpriseKeyCredentialRegistrationManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<EnterpriseKeyCredentialRegistrationManager, IEnterpriseKeyCredentialRegistrationManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for EnterpriseKeyCredentialRegistrationManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEnterpriseKeyCredentialRegistrationManager>();
}
unsafe impl windows_core::Interface for EnterpriseKeyCredentialRegistrationManager {
    type Vtable = IEnterpriseKeyCredentialRegistrationManager_Vtbl;
    const IID: windows_core::GUID = <IEnterpriseKeyCredentialRegistrationManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EnterpriseKeyCredentialRegistrationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager";
}
unsafe impl Send for EnterpriseKeyCredentialRegistrationManager {}
unsafe impl Sync for EnterpriseKeyCredentialRegistrationManager {}
