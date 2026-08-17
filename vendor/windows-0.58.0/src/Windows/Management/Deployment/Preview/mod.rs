windows_core::imp::define_interface!(IClassicAppManagerStatics, IClassicAppManagerStatics_Vtbl, 0xe2fad668_882c_4f33_b035_0df7b90d67e6);
impl windows_core::RuntimeType for IClassicAppManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClassicAppManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FindInstalledApp: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInstalledClassicAppInfo, IInstalledClassicAppInfo_Vtbl, 0x0a7d3da3_65d0_4086_80d6_0610d760207d);
impl windows_core::RuntimeType for IInstalledClassicAppInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInstalledClassicAppInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DisplayVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
pub struct ClassicAppManager;
impl ClassicAppManager {
    pub fn FindInstalledApp(appuninstallkey: &windows_core::HSTRING) -> windows_core::Result<InstalledClassicAppInfo> {
        Self::IClassicAppManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindInstalledApp)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appuninstallkey), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IClassicAppManagerStatics<R, F: FnOnce(&IClassicAppManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ClassicAppManager, IClassicAppManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ClassicAppManager {
    const NAME: &'static str = "Windows.Management.Deployment.Preview.ClassicAppManager";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct InstalledClassicAppInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InstalledClassicAppInfo, windows_core::IUnknown, windows_core::IInspectable);
impl InstalledClassicAppInfo {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayVersion(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayVersion)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InstalledClassicAppInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInstalledClassicAppInfo>();
}
unsafe impl windows_core::Interface for InstalledClassicAppInfo {
    type Vtable = IInstalledClassicAppInfo_Vtbl;
    const IID: windows_core::GUID = <IInstalledClassicAppInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InstalledClassicAppInfo {
    const NAME: &'static str = "Windows.Management.Deployment.Preview.InstalledClassicAppInfo";
}
unsafe impl Send for InstalledClassicAppInfo {}
unsafe impl Sync for InstalledClassicAppInfo {}
