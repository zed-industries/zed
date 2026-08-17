windows_core::imp::define_interface!(IHolographicApplicationPreviewStatics, IHolographicApplicationPreviewStatics_Vtbl, 0xfe038691_2a3a_45a9_a208_7bed691919f3);
impl windows_core::RuntimeType for IHolographicApplicationPreviewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicApplicationPreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCurrentViewPresentedOnHolographicDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub IsHolographicActivation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    IsHolographicActivation: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IHolographicKeyboardPlacementOverridePreview, IHolographicKeyboardPlacementOverridePreview_Vtbl, 0xc8a8ce3a_dfde_5a14_8d5f_182c526dd9c4);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IHolographicKeyboardPlacementOverridePreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IHolographicKeyboardPlacementOverridePreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub SetPlacementOverride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3, super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated")))]
    SetPlacementOverride: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub SetPlacementOverrideWithMaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3, super::super::super::Foundation::Numerics::Vector3, super::super::super::Foundation::Numerics::Vector2) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated")))]
    SetPlacementOverrideWithMaxSize: usize,
    #[cfg(feature = "deprecated")]
    pub ResetPlacementOverride: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResetPlacementOverride: usize,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IHolographicKeyboardPlacementOverridePreviewStatics, IHolographicKeyboardPlacementOverridePreviewStatics_Vtbl, 0x202e6039_1ff6_5a06_aac4_a5e24fa3ec4b);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IHolographicKeyboardPlacementOverridePreviewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IHolographicKeyboardPlacementOverridePreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetForCurrentView: usize,
}
pub struct HolographicApplicationPreview;
impl HolographicApplicationPreview {
    pub fn IsCurrentViewPresentedOnHolographicDisplay() -> windows_core::Result<bool> {
        Self::IHolographicApplicationPreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCurrentViewPresentedOnHolographicDisplay)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn IsHolographicActivation<P0>(activatedeventargs: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::Activation::IActivatedEventArgs>,
    {
        Self::IHolographicApplicationPreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHolographicActivation)(windows_core::Interface::as_raw(this), activatedeventargs.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IHolographicApplicationPreviewStatics<R, F: FnOnce(&IHolographicApplicationPreviewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HolographicApplicationPreview, IHolographicApplicationPreviewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for HolographicApplicationPreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Holographic.HolographicApplicationPreview";
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HolographicKeyboardPlacementOverridePreview(windows_core::IUnknown);
#[cfg(feature = "deprecated")]
windows_core::imp::interface_hierarchy!(HolographicKeyboardPlacementOverridePreview, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl HolographicKeyboardPlacementOverridePreview {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub fn SetPlacementOverride<P0>(&self, coordinatesystem: P0, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPlacementOverride)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), topcenterposition, normal).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated"))]
    pub fn SetPlacementOverrideWithMaxSize<P0>(&self, coordinatesystem: P0, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3, maxsize: super::super::super::Foundation::Numerics::Vector2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPlacementOverrideWithMaxSize)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), topcenterposition, normal, maxsize).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ResetPlacementOverride(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ResetPlacementOverride)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn GetForCurrentView() -> windows_core::Result<HolographicKeyboardPlacementOverridePreview> {
        Self::IHolographicKeyboardPlacementOverridePreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IHolographicKeyboardPlacementOverridePreviewStatics<R, F: FnOnce(&IHolographicKeyboardPlacementOverridePreviewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HolographicKeyboardPlacementOverridePreview, IHolographicKeyboardPlacementOverridePreviewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for HolographicKeyboardPlacementOverridePreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicKeyboardPlacementOverridePreview>();
}
#[cfg(feature = "deprecated")]
unsafe impl windows_core::Interface for HolographicKeyboardPlacementOverridePreview {
    type Vtable = IHolographicKeyboardPlacementOverridePreview_Vtbl;
    const IID: windows_core::GUID = <IHolographicKeyboardPlacementOverridePreview as windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for HolographicKeyboardPlacementOverridePreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Holographic.HolographicKeyboardPlacementOverridePreview";
}
#[cfg(feature = "deprecated")]
unsafe impl Send for HolographicKeyboardPlacementOverridePreview {}
#[cfg(feature = "deprecated")]
unsafe impl Sync for HolographicKeyboardPlacementOverridePreview {}
