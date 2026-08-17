windows_core::imp::define_interface!(IRadialControllerIndependentInputSource, IRadialControllerIndependentInputSource_Vtbl, 0x3d577ef6_4cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for IRadialControllerIndependentInputSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerIndependentInputSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Controller: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub Dispatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    Dispatcher: usize,
}
windows_core::imp::define_interface!(IRadialControllerIndependentInputSource2, IRadialControllerIndependentInputSource2_Vtbl, 0x7073aad8_35f3_4eeb_8751_be4d0a66faf4);
impl windows_core::RuntimeType for IRadialControllerIndependentInputSource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerIndependentInputSource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
windows_core::imp::define_interface!(IRadialControllerIndependentInputSourceStatics, IRadialControllerIndependentInputSourceStatics_Vtbl, 0x3d577ef5_4cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for IRadialControllerIndependentInputSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IRadialControllerIndependentInputSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Core")]
    pub CreateForView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    CreateForView: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct RadialControllerIndependentInputSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RadialControllerIndependentInputSource, windows_core::IUnknown, windows_core::IInspectable);
impl RadialControllerIndependentInputSource {
    pub fn Controller(&self) -> windows_core::Result<super::RadialController> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Controller)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<IRadialControllerIndependentInputSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn CreateForView<P0>(view: P0) -> windows_core::Result<RadialControllerIndependentInputSource>
    where
        P0: windows_core::Param<super::super::super::ApplicationModel::Core::CoreApplicationView>,
    {
        Self::IRadialControllerIndependentInputSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForView)(windows_core::Interface::as_raw(this), view.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IRadialControllerIndependentInputSourceStatics<R, F: FnOnce(&IRadialControllerIndependentInputSourceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RadialControllerIndependentInputSource, IRadialControllerIndependentInputSourceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RadialControllerIndependentInputSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRadialControllerIndependentInputSource>();
}
unsafe impl windows_core::Interface for RadialControllerIndependentInputSource {
    type Vtable = IRadialControllerIndependentInputSource_Vtbl;
    const IID: windows_core::GUID = <IRadialControllerIndependentInputSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RadialControllerIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Input.Core.RadialControllerIndependentInputSource";
}
unsafe impl Send for RadialControllerIndependentInputSource {}
unsafe impl Sync for RadialControllerIndependentInputSource {}
