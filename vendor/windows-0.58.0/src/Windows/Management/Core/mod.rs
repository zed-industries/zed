windows_core::imp::define_interface!(IApplicationDataManager, IApplicationDataManager_Vtbl, 0x74d10432_2e99_4000_9a3a_64307e858129);
impl windows_core::RuntimeType for IApplicationDataManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationDataManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IApplicationDataManagerStatics, IApplicationDataManagerStatics_Vtbl, 0x1e1862e3_698e_49a1_9752_dee94925b9b3);
impl windows_core::RuntimeType for IApplicationDataManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationDataManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub CreateForPackageFamily: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateForPackageFamily: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ApplicationDataManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ApplicationDataManager, windows_core::IUnknown, windows_core::IInspectable);
impl ApplicationDataManager {
    #[cfg(feature = "Storage")]
    pub fn CreateForPackageFamily(packagefamilyname: &windows_core::HSTRING) -> windows_core::Result<super::super::Storage::ApplicationData> {
        Self::IApplicationDataManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForPackageFamily)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(packagefamilyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IApplicationDataManagerStatics<R, F: FnOnce(&IApplicationDataManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ApplicationDataManager, IApplicationDataManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ApplicationDataManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IApplicationDataManager>();
}
unsafe impl windows_core::Interface for ApplicationDataManager {
    type Vtable = IApplicationDataManager_Vtbl;
    const IID: windows_core::GUID = <IApplicationDataManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ApplicationDataManager {
    const NAME: &'static str = "Windows.Management.Core.ApplicationDataManager";
}
unsafe impl Send for ApplicationDataManager {}
unsafe impl Sync for ApplicationDataManager {}
