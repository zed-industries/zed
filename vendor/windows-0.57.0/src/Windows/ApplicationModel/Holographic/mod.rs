windows_core::imp::define_interface!(IHolographicKeyboard, IHolographicKeyboard_Vtbl, 0x07dd0893_aa21_5e6f_a91b_11b2b3fd7be3);
impl windows_core::RuntimeType for IHolographicKeyboard {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicKeyboard_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetPlacementOverride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3, super::super::Foundation::Numerics::Quaternion) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetPlacementOverride: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetPlacementOverrideWithMaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::Numerics::Vector3, super::super::Foundation::Numerics::Quaternion, super::super::Foundation::Numerics::Vector2) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetPlacementOverrideWithMaxSize: usize,
    pub ResetPlacementOverride: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHolographicKeyboardStatics, IHolographicKeyboardStatics_Vtbl, 0xb676c624_63d7_58cf_b06b_08baa032a23f);
impl windows_core::RuntimeType for IHolographicKeyboardStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IHolographicKeyboardStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct HolographicKeyboard(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HolographicKeyboard, windows_core::IUnknown, windows_core::IInspectable);
impl HolographicKeyboard {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetPlacementOverride<P0>(&self, coordinatesystem: P0, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPlacementOverride)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), topcenterposition, orientation).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetPlacementOverrideWithMaxSize<P0>(&self, coordinatesystem: P0, topcenterposition: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion, maxsize: super::super::Foundation::Numerics::Vector2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Perception::Spatial::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPlacementOverrideWithMaxSize)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), topcenterposition, orientation, maxsize).ok() }
    }
    pub fn ResetPlacementOverride(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ResetPlacementOverride)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDefault() -> windows_core::Result<HolographicKeyboard> {
        Self::IHolographicKeyboardStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IHolographicKeyboardStatics<R, F: FnOnce(&IHolographicKeyboardStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HolographicKeyboard, IHolographicKeyboardStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for HolographicKeyboard {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHolographicKeyboard>();
}
unsafe impl windows_core::Interface for HolographicKeyboard {
    type Vtable = IHolographicKeyboard_Vtbl;
    const IID: windows_core::GUID = <IHolographicKeyboard as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HolographicKeyboard {
    const NAME: &'static str = "Windows.ApplicationModel.Holographic.HolographicKeyboard";
}
unsafe impl Send for HolographicKeyboard {}
unsafe impl Sync for HolographicKeyboard {}
