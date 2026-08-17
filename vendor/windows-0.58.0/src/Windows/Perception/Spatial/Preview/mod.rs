windows_core::imp::define_interface!(ISpatialGraphInteropFrameOfReferencePreview, ISpatialGraphInteropFrameOfReferencePreview_Vtbl, 0xa8271b23_735f_5729_a98e_e64ed189abc5);
impl windows_core::RuntimeType for ISpatialGraphInteropFrameOfReferencePreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialGraphInteropFrameOfReferencePreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CoordinateSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NodeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CoordinateSystemToNodeTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Matrix4x4) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CoordinateSystemToNodeTransform: usize,
}
windows_core::imp::define_interface!(ISpatialGraphInteropPreviewStatics, ISpatialGraphInteropPreviewStatics_Vtbl, 0xc042644c_20d8_4ed0_aef7_6805b8e53f55);
impl windows_core::RuntimeType for ISpatialGraphInteropPreviewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialGraphInteropPreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateCoordinateSystemForNode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateCoordinateSystemForNodeWithPosition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, super::super::super::Foundation::Numerics::Vector3, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateCoordinateSystemForNodeWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateCoordinateSystemForNodeWithPositionAndOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, super::super::super::Foundation::Numerics::Vector3, super::super::super::Foundation::Numerics::Quaternion, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateCoordinateSystemForNodeWithPositionAndOrientation: usize,
    pub CreateLocatorForNode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialGraphInteropPreviewStatics2, ISpatialGraphInteropPreviewStatics2_Vtbl, 0x2490b15f_6cbd_4b1e_b765_31e462a32df2);
impl windows_core::RuntimeType for ISpatialGraphInteropPreviewStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialGraphInteropPreviewStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryCreateFrameOfReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateFrameOfReferenceWithPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateFrameOfReferenceWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateFrameOfReferenceWithPositionAndOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::Numerics::Vector3, super::super::super::Foundation::Numerics::Quaternion, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateFrameOfReferenceWithPositionAndOrientation: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SpatialGraphInteropFrameOfReferencePreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialGraphInteropFrameOfReferencePreview, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialGraphInteropFrameOfReferencePreview {
    pub fn CoordinateSystem(&self) -> windows_core::Result<super::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CoordinateSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NodeId(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CoordinateSystemToNodeTransform(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Matrix4x4> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CoordinateSystemToNodeTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SpatialGraphInteropFrameOfReferencePreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialGraphInteropFrameOfReferencePreview>();
}
unsafe impl windows_core::Interface for SpatialGraphInteropFrameOfReferencePreview {
    type Vtable = ISpatialGraphInteropFrameOfReferencePreview_Vtbl;
    const IID: windows_core::GUID = <ISpatialGraphInteropFrameOfReferencePreview as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialGraphInteropFrameOfReferencePreview {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.SpatialGraphInteropFrameOfReferencePreview";
}
unsafe impl Send for SpatialGraphInteropFrameOfReferencePreview {}
unsafe impl Sync for SpatialGraphInteropFrameOfReferencePreview {}
pub struct SpatialGraphInteropPreview;
impl SpatialGraphInteropPreview {
    pub fn CreateCoordinateSystemForNode(nodeid: windows_core::GUID) -> windows_core::Result<super::SpatialCoordinateSystem> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCoordinateSystemForNode)(windows_core::Interface::as_raw(this), nodeid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateCoordinateSystemForNodeWithPosition(nodeid: windows_core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3) -> windows_core::Result<super::SpatialCoordinateSystem> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCoordinateSystemForNodeWithPosition)(windows_core::Interface::as_raw(this), nodeid, relativeposition, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateCoordinateSystemForNodeWithPositionAndOrientation(nodeid: windows_core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion) -> windows_core::Result<super::SpatialCoordinateSystem> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCoordinateSystemForNodeWithPositionAndOrientation)(windows_core::Interface::as_raw(this), nodeid, relativeposition, relativeorientation, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateLocatorForNode(nodeid: windows_core::GUID) -> windows_core::Result<super::SpatialLocator> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateLocatorForNode)(windows_core::Interface::as_raw(this), nodeid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryCreateFrameOfReference<P0>(coordinatesystem: P0) -> windows_core::Result<SpatialGraphInteropFrameOfReferencePreview>
    where
        P0: windows_core::Param<super::SpatialCoordinateSystem>,
    {
        Self::ISpatialGraphInteropPreviewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateFrameOfReference)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateFrameOfReferenceWithPosition<P0>(coordinatesystem: P0, relativeposition: super::super::super::Foundation::Numerics::Vector3) -> windows_core::Result<SpatialGraphInteropFrameOfReferencePreview>
    where
        P0: windows_core::Param<super::SpatialCoordinateSystem>,
    {
        Self::ISpatialGraphInteropPreviewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateFrameOfReferenceWithPosition)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), relativeposition, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateFrameOfReferenceWithPositionAndOrientation<P0>(coordinatesystem: P0, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion) -> windows_core::Result<SpatialGraphInteropFrameOfReferencePreview>
    where
        P0: windows_core::Param<super::SpatialCoordinateSystem>,
    {
        Self::ISpatialGraphInteropPreviewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateFrameOfReferenceWithPositionAndOrientation)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), relativeposition, relativeorientation, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISpatialGraphInteropPreviewStatics<R, F: FnOnce(&ISpatialGraphInteropPreviewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialGraphInteropPreview, ISpatialGraphInteropPreviewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISpatialGraphInteropPreviewStatics2<R, F: FnOnce(&ISpatialGraphInteropPreviewStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialGraphInteropPreview, ISpatialGraphInteropPreviewStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for SpatialGraphInteropPreview {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.SpatialGraphInteropPreview";
}
