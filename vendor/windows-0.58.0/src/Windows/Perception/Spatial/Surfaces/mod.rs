windows_core::imp::define_interface!(ISpatialSurfaceInfo, ISpatialSurfaceInfo_Vtbl, 0xf8e9ebe7_39b7_3962_bb03_57f56e1fb0a1);
impl windows_core::RuntimeType for ISpatialSurfaceInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialSurfaceInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub UpdateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetBounds: usize,
    pub TryComputeLatestMeshAsync: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryComputeLatestMeshWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialSurfaceMesh, ISpatialSurfaceMesh_Vtbl, 0x108f57d9_df0d_3950_a0fd_f972c77c27b4);
impl windows_core::RuntimeType for ISpatialSurfaceMesh {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialSurfaceMesh_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SurfaceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CoordinateSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TriangleIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VertexPositions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub VertexPositionScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Numerics::Vector3) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    VertexPositionScale: usize,
    pub VertexNormals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialSurfaceMeshBuffer, ISpatialSurfaceMeshBuffer_Vtbl, 0x93cf59e0_871f_33f8_98b2_03d101458f6f);
impl windows_core::RuntimeType for ISpatialSurfaceMeshBuffer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialSurfaceMeshBuffer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX")]
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    Format: usize,
    pub Stride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ElementCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
windows_core::imp::define_interface!(ISpatialSurfaceMeshOptions, ISpatialSurfaceMeshOptions_Vtbl, 0xd2759f89_3572_3d2d_a10d_5fee9394aa37);
impl windows_core::RuntimeType for ISpatialSurfaceMeshOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialSurfaceMeshOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX")]
    pub VertexPositionFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    VertexPositionFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetVertexPositionFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Graphics::DirectX::DirectXPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetVertexPositionFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub TriangleIndexFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    TriangleIndexFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetTriangleIndexFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Graphics::DirectX::DirectXPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetTriangleIndexFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub VertexNormalFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    VertexNormalFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetVertexNormalFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Graphics::DirectX::DirectXPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetVertexNormalFormat: usize,
    pub IncludeVertexNormals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIncludeVertexNormals: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialSurfaceMeshOptionsStatics, ISpatialSurfaceMeshOptionsStatics_Vtbl, 0x9b340abf_9781_4505_8935_013575caae5e);
impl windows_core::RuntimeType for ISpatialSurfaceMeshOptionsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialSurfaceMeshOptionsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedVertexPositionFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedVertexPositionFormats: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedTriangleIndexFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedTriangleIndexFormats: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedVertexNormalFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedVertexNormalFormats: usize,
}
windows_core::imp::define_interface!(ISpatialSurfaceObserver, ISpatialSurfaceObserver_Vtbl, 0x10b69819_ddca_3483_ac3a_748fe8c86df5);
impl windows_core::RuntimeType for ISpatialSurfaceObserver {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialSurfaceObserver_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetObservedSurfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetObservedSurfaces: usize,
    pub SetBoundingVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetBoundingVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetBoundingVolumes: usize,
    pub ObservedSurfacesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveObservedSurfacesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialSurfaceObserverStatics, ISpatialSurfaceObserverStatics_Vtbl, 0x165951ed_2108_4168_9175_87e027bc9285);
impl windows_core::RuntimeType for ISpatialSurfaceObserverStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialSurfaceObserverStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialSurfaceObserverStatics2, ISpatialSurfaceObserverStatics2_Vtbl, 0x0f534261_c55d_4e6b_a895_a19de69a42e3);
impl windows_core::RuntimeType for ISpatialSurfaceObserverStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISpatialSurfaceObserverStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SpatialSurfaceInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialSurfaceInfo, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialSurfaceInfo {
    pub fn Id(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UpdateTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UpdateTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryGetBounds<P0>(&self, coordinatesystem: P0) -> windows_core::Result<super::super::super::Foundation::IReference<super::SpatialBoundingOrientedBox>>
    where
        P0: windows_core::Param<super::SpatialCoordinateSystem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetBounds)(windows_core::Interface::as_raw(this), coordinatesystem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryComputeLatestMeshAsync(&self, maxtrianglespercubicmeter: f64) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryComputeLatestMeshAsync)(windows_core::Interface::as_raw(this), maxtrianglespercubicmeter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryComputeLatestMeshWithOptionsAsync<P0>(&self, maxtrianglespercubicmeter: f64, options: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>>
    where
        P0: windows_core::Param<SpatialSurfaceMeshOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryComputeLatestMeshWithOptionsAsync)(windows_core::Interface::as_raw(this), maxtrianglespercubicmeter, options.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialSurfaceInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialSurfaceInfo>();
}
unsafe impl windows_core::Interface for SpatialSurfaceInfo {
    type Vtable = ISpatialSurfaceInfo_Vtbl;
    const IID: windows_core::GUID = <ISpatialSurfaceInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialSurfaceInfo {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo";
}
unsafe impl Send for SpatialSurfaceInfo {}
unsafe impl Sync for SpatialSurfaceInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SpatialSurfaceMesh(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialSurfaceMesh, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialSurfaceMesh {
    pub fn SurfaceInfo(&self) -> windows_core::Result<SpatialSurfaceInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SurfaceInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CoordinateSystem(&self) -> windows_core::Result<super::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CoordinateSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TriangleIndices(&self) -> windows_core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriangleIndices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VertexPositions(&self) -> windows_core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VertexPositions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn VertexPositionScale(&self) -> windows_core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VertexPositionScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn VertexNormals(&self) -> windows_core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VertexNormals)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialSurfaceMesh {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialSurfaceMesh>();
}
unsafe impl windows_core::Interface for SpatialSurfaceMesh {
    type Vtable = ISpatialSurfaceMesh_Vtbl;
    const IID: windows_core::GUID = <ISpatialSurfaceMesh as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialSurfaceMesh {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh";
}
unsafe impl Send for SpatialSurfaceMesh {}
unsafe impl Sync for SpatialSurfaceMesh {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SpatialSurfaceMeshBuffer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialSurfaceMeshBuffer, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialSurfaceMeshBuffer {
    #[cfg(feature = "Graphics_DirectX")]
    pub fn Format(&self) -> windows_core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Format)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Stride(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stride)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ElementCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ElementCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SpatialSurfaceMeshBuffer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialSurfaceMeshBuffer>();
}
unsafe impl windows_core::Interface for SpatialSurfaceMeshBuffer {
    type Vtable = ISpatialSurfaceMeshBuffer_Vtbl;
    const IID: windows_core::GUID = <ISpatialSurfaceMeshBuffer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialSurfaceMeshBuffer {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer";
}
unsafe impl Send for SpatialSurfaceMeshBuffer {}
unsafe impl Sync for SpatialSurfaceMeshBuffer {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SpatialSurfaceMeshOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialSurfaceMeshOptions, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialSurfaceMeshOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialSurfaceMeshOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn VertexPositionFormat(&self) -> windows_core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VertexPositionFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetVertexPositionFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVertexPositionFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn TriangleIndexFormat(&self) -> windows_core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriangleIndexFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetTriangleIndexFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTriangleIndexFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn VertexNormalFormat(&self) -> windows_core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VertexNormalFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetVertexNormalFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVertexNormalFormat)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeVertexNormals(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IncludeVertexNormals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIncludeVertexNormals(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIncludeVertexNormals)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedVertexPositionFormats() -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedVertexPositionFormats)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedTriangleIndexFormats() -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedTriangleIndexFormats)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedVertexNormalFormats() -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedVertexNormalFormats)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ISpatialSurfaceMeshOptionsStatics<R, F: FnOnce(&ISpatialSurfaceMeshOptionsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialSurfaceMeshOptions, ISpatialSurfaceMeshOptionsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpatialSurfaceMeshOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialSurfaceMeshOptions>();
}
unsafe impl windows_core::Interface for SpatialSurfaceMeshOptions {
    type Vtable = ISpatialSurfaceMeshOptions_Vtbl;
    const IID: windows_core::GUID = <ISpatialSurfaceMeshOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialSurfaceMeshOptions {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions";
}
unsafe impl Send for SpatialSurfaceMeshOptions {}
unsafe impl Sync for SpatialSurfaceMeshOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SpatialSurfaceObserver(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SpatialSurfaceObserver, windows_core::IUnknown, windows_core::IInspectable);
impl SpatialSurfaceObserver {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialSurfaceObserver, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetObservedSurfaces(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IMapView<windows_core::GUID, SpatialSurfaceInfo>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetObservedSurfaces)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBoundingVolume<P0>(&self, bounds: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::SpatialBoundingVolume>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBoundingVolume)(windows_core::Interface::as_raw(this), bounds.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetBoundingVolumes<P0>(&self, bounds: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Collections::IIterable<super::SpatialBoundingVolume>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBoundingVolumes)(windows_core::Interface::as_raw(this), bounds.param().abi()).ok() }
    }
    pub fn ObservedSurfacesChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<SpatialSurfaceObserver, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ObservedSurfacesChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveObservedSurfacesChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveObservedSurfacesChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RequestAccessAsync() -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>> {
        Self::ISpatialSurfaceObserverStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::ISpatialSurfaceObserverStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialSurfaceObserverStatics<R, F: FnOnce(&ISpatialSurfaceObserverStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialSurfaceObserver, ISpatialSurfaceObserverStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISpatialSurfaceObserverStatics2<R, F: FnOnce(&ISpatialSurfaceObserverStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SpatialSurfaceObserver, ISpatialSurfaceObserverStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SpatialSurfaceObserver {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISpatialSurfaceObserver>();
}
unsafe impl windows_core::Interface for SpatialSurfaceObserver {
    type Vtable = ISpatialSurfaceObserver_Vtbl;
    const IID: windows_core::GUID = <ISpatialSurfaceObserver as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SpatialSurfaceObserver {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver";
}
unsafe impl Send for SpatialSurfaceObserver {}
unsafe impl Sync for SpatialSurfaceObserver {}
