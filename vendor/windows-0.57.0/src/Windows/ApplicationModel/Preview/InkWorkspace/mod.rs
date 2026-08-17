windows_core::imp::define_interface!(IInkWorkspaceHostedAppManager, IInkWorkspaceHostedAppManager_Vtbl, 0xfe0a7990_5e59_4bb7_8a63_7d218cd96300);
impl windows_core::RuntimeType for IInkWorkspaceHostedAppManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkWorkspaceHostedAppManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetThumbnailAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetThumbnailAsync: usize,
}
windows_core::imp::define_interface!(IInkWorkspaceHostedAppManagerStatics, IInkWorkspaceHostedAppManagerStatics_Vtbl, 0xcbfd8cc5_a162_4bc4_84ee_e8716d5233c5);
impl windows_core::RuntimeType for IInkWorkspaceHostedAppManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInkWorkspaceHostedAppManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentApp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct InkWorkspaceHostedAppManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InkWorkspaceHostedAppManager, windows_core::IUnknown, windows_core::IInspectable);
impl InkWorkspaceHostedAppManager {
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetThumbnailAsync<P0>(&self, bitmap: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetThumbnailAsync)(windows_core::Interface::as_raw(this), bitmap.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetForCurrentApp() -> windows_core::Result<InkWorkspaceHostedAppManager> {
        Self::IInkWorkspaceHostedAppManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentApp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IInkWorkspaceHostedAppManagerStatics<R, F: FnOnce(&IInkWorkspaceHostedAppManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InkWorkspaceHostedAppManager, IInkWorkspaceHostedAppManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InkWorkspaceHostedAppManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInkWorkspaceHostedAppManager>();
}
unsafe impl windows_core::Interface for InkWorkspaceHostedAppManager {
    type Vtable = IInkWorkspaceHostedAppManager_Vtbl;
    const IID: windows_core::GUID = <IInkWorkspaceHostedAppManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InkWorkspaceHostedAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.InkWorkspace.InkWorkspaceHostedAppManager";
}
unsafe impl Send for InkWorkspaceHostedAppManager {}
unsafe impl Sync for InkWorkspaceHostedAppManager {}
