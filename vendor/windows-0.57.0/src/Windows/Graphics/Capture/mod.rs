windows_core::imp::define_interface!(IDirect3D11CaptureFrame, IDirect3D11CaptureFrame_Vtbl, 0xfa50c623_38da_4b32_acf3_fa9734ad800e);
impl windows_core::RuntimeType for IDirect3D11CaptureFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDirect3D11CaptureFrame_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Surface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Surface: usize,
    pub SystemRelativeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub ContentSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SizeInt32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirect3D11CaptureFramePool, IDirect3D11CaptureFramePool_Vtbl, 0x24eb6d22_1975_422e_82e7_780dbd8ddf24);
impl windows_core::RuntimeType for IDirect3D11CaptureFramePool {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDirect3D11CaptureFramePool_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Recreate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DirectX::DirectXPixelFormat, i32, super::SizeInt32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Recreate: usize,
    pub TryGetNextFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FrameArrived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveFrameArrived: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub CreateCaptureSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub DispatcherQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    DispatcherQueue: usize,
}
windows_core::imp::define_interface!(IDirect3D11CaptureFramePoolStatics, IDirect3D11CaptureFramePoolStatics_Vtbl, 0x7784056a_67aa_4d53_ae54_1088d5a8ca21);
impl windows_core::RuntimeType for IDirect3D11CaptureFramePoolStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDirect3D11CaptureFramePoolStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DirectX::DirectXPixelFormat, i32, super::SizeInt32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Create: usize,
}
windows_core::imp::define_interface!(IDirect3D11CaptureFramePoolStatics2, IDirect3D11CaptureFramePoolStatics2_Vtbl, 0x589b103f_6bbc_5df5_a991_02e28b3b66d5);
impl windows_core::RuntimeType for IDirect3D11CaptureFramePoolStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDirect3D11CaptureFramePoolStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFreeThreaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DirectX::DirectXPixelFormat, i32, super::SizeInt32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFreeThreaded: usize,
}
windows_core::imp::define_interface!(IGraphicsCaptureAccessStatics, IGraphicsCaptureAccessStatics_Vtbl, 0x743ed370_06ec_5040_a58a_901f0f757095);
impl windows_core::RuntimeType for IGraphicsCaptureAccessStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGraphicsCaptureAccessStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Authorization_AppCapabilityAccess")]
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, GraphicsCaptureAccessKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authorization_AppCapabilityAccess"))]
    RequestAccessAsync: usize,
}
windows_core::imp::define_interface!(IGraphicsCaptureItem, IGraphicsCaptureItem_Vtbl, 0x79c3f95b_31f7_4ec2_a464_632ef5d30760);
impl windows_core::RuntimeType for IGraphicsCaptureItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGraphicsCaptureItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SizeInt32) -> windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGraphicsCaptureItemStatics, IGraphicsCaptureItemStatics_Vtbl, 0xa87ebea5_457c_5788_ab47_0cf1d3637e74);
impl windows_core::RuntimeType for IGraphicsCaptureItemStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGraphicsCaptureItemStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateFromVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateFromVisual: usize,
}
windows_core::imp::define_interface!(IGraphicsCaptureItemStatics2, IGraphicsCaptureItemStatics2_Vtbl, 0x3b92acc9_e584_5862_bf5c_9c316c6d2dbb);
impl windows_core::RuntimeType for IGraphicsCaptureItemStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGraphicsCaptureItemStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub TryCreateFromWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::WindowId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    TryCreateFromWindowId: usize,
    pub TryCreateFromDisplayId: unsafe extern "system" fn(*mut core::ffi::c_void, super::DisplayId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGraphicsCapturePicker, IGraphicsCapturePicker_Vtbl, 0x5a1711b3_ad79_4b4a_9336_1318fdde3539);
impl windows_core::RuntimeType for IGraphicsCapturePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGraphicsCapturePicker_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PickSingleItemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGraphicsCaptureSession, IGraphicsCaptureSession_Vtbl, 0x814e42a9_f70f_4ad7_939b_fddcc6eb880d);
impl windows_core::RuntimeType for IGraphicsCaptureSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGraphicsCaptureSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartCapture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGraphicsCaptureSession2, IGraphicsCaptureSession2_Vtbl, 0x2c39ae40_7d2e_5044_804e_8b6799d4cf9e);
impl windows_core::RuntimeType for IGraphicsCaptureSession2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGraphicsCaptureSession2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsCursorCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCursorCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGraphicsCaptureSession3, IGraphicsCaptureSession3_Vtbl, 0xf2cdd966_22ae_5ea1_9596_3a289344c3be);
impl windows_core::RuntimeType for IGraphicsCaptureSession3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGraphicsCaptureSession3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsBorderRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsBorderRequired: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGraphicsCaptureSessionStatics, IGraphicsCaptureSessionStatics_Vtbl, 0x2224a540_5974_49aa_b232_0882536f4cb5);
impl windows_core::RuntimeType for IGraphicsCaptureSessionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGraphicsCaptureSessionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Direct3D11CaptureFrame(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Direct3D11CaptureFrame, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Direct3D11CaptureFrame, super::super::Foundation::IClosable);
impl Direct3D11CaptureFrame {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Surface(&self) -> windows_core::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Surface)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SystemRelativeTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemRelativeTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ContentSize(&self) -> windows_core::Result<super::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for Direct3D11CaptureFrame {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDirect3D11CaptureFrame>();
}
unsafe impl windows_core::Interface for Direct3D11CaptureFrame {
    type Vtable = IDirect3D11CaptureFrame_Vtbl;
    const IID: windows_core::GUID = <IDirect3D11CaptureFrame as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Direct3D11CaptureFrame {
    const NAME: &'static str = "Windows.Graphics.Capture.Direct3D11CaptureFrame";
}
unsafe impl Send for Direct3D11CaptureFrame {}
unsafe impl Sync for Direct3D11CaptureFrame {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Direct3D11CaptureFramePool(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Direct3D11CaptureFramePool, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Direct3D11CaptureFramePool, super::super::Foundation::IClosable);
impl Direct3D11CaptureFramePool {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Recreate<P0>(&self, device: P0, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DirectX::Direct3D11::IDirect3DDevice>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Recreate)(windows_core::Interface::as_raw(this), device.param().abi(), pixelformat, numberofbuffers, size).ok() }
    }
    pub fn TryGetNextFrame(&self) -> windows_core::Result<Direct3D11CaptureFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetNextFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FrameArrived<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Direct3D11CaptureFramePool, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameArrived)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFrameArrived(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveFrameArrived)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateCaptureSession<P0>(&self, item: P0) -> windows_core::Result<GraphicsCaptureSession>
    where
        P0: windows_core::Param<GraphicsCaptureItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCaptureSession)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::System::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Create<P0>(device: P0, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> windows_core::Result<Direct3D11CaptureFramePool>
    where
        P0: windows_core::Param<super::DirectX::Direct3D11::IDirect3DDevice>,
    {
        Self::IDirect3D11CaptureFramePoolStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), device.param().abi(), pixelformat, numberofbuffers, size, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFreeThreaded<P0>(device: P0, pixelformat: super::DirectX::DirectXPixelFormat, numberofbuffers: i32, size: super::SizeInt32) -> windows_core::Result<Direct3D11CaptureFramePool>
    where
        P0: windows_core::Param<super::DirectX::Direct3D11::IDirect3DDevice>,
    {
        Self::IDirect3D11CaptureFramePoolStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFreeThreaded)(windows_core::Interface::as_raw(this), device.param().abi(), pixelformat, numberofbuffers, size, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDirect3D11CaptureFramePoolStatics<R, F: FnOnce(&IDirect3D11CaptureFramePoolStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Direct3D11CaptureFramePool, IDirect3D11CaptureFramePoolStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDirect3D11CaptureFramePoolStatics2<R, F: FnOnce(&IDirect3D11CaptureFramePoolStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Direct3D11CaptureFramePool, IDirect3D11CaptureFramePoolStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Direct3D11CaptureFramePool {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDirect3D11CaptureFramePool>();
}
unsafe impl windows_core::Interface for Direct3D11CaptureFramePool {
    type Vtable = IDirect3D11CaptureFramePool_Vtbl;
    const IID: windows_core::GUID = <IDirect3D11CaptureFramePool as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Direct3D11CaptureFramePool {
    const NAME: &'static str = "Windows.Graphics.Capture.Direct3D11CaptureFramePool";
}
unsafe impl Send for Direct3D11CaptureFramePool {}
unsafe impl Sync for Direct3D11CaptureFramePool {}
pub struct GraphicsCaptureAccess;
impl GraphicsCaptureAccess {
    #[cfg(feature = "Security_Authorization_AppCapabilityAccess")]
    pub fn RequestAccessAsync(request: GraphicsCaptureAccessKind) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus>> {
        Self::IGraphicsCaptureAccessStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), request, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureAccessStatics<R, F: FnOnce(&IGraphicsCaptureAccessStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GraphicsCaptureAccess, IGraphicsCaptureAccessStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for GraphicsCaptureAccess {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureAccess";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GraphicsCaptureItem(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GraphicsCaptureItem, windows_core::IUnknown, windows_core::IInspectable);
impl GraphicsCaptureItem {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<super::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Closed<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GraphicsCaptureItem, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn CreateFromVisual<P0>(visual: P0) -> windows_core::Result<GraphicsCaptureItem>
    where
        P0: windows_core::Param<super::super::UI::Composition::Visual>,
    {
        Self::IGraphicsCaptureItemStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromVisual)(windows_core::Interface::as_raw(this), visual.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI")]
    pub fn TryCreateFromWindowId(windowid: super::super::UI::WindowId) -> windows_core::Result<GraphicsCaptureItem> {
        Self::IGraphicsCaptureItemStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateFromWindowId)(windows_core::Interface::as_raw(this), windowid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryCreateFromDisplayId(displayid: super::DisplayId) -> windows_core::Result<GraphicsCaptureItem> {
        Self::IGraphicsCaptureItemStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCreateFromDisplayId)(windows_core::Interface::as_raw(this), displayid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureItemStatics<R, F: FnOnce(&IGraphicsCaptureItemStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GraphicsCaptureItem, IGraphicsCaptureItemStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureItemStatics2<R, F: FnOnce(&IGraphicsCaptureItemStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GraphicsCaptureItem, IGraphicsCaptureItemStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GraphicsCaptureItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGraphicsCaptureItem>();
}
unsafe impl windows_core::Interface for GraphicsCaptureItem {
    type Vtable = IGraphicsCaptureItem_Vtbl;
    const IID: windows_core::GUID = <IGraphicsCaptureItem as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GraphicsCaptureItem {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureItem";
}
unsafe impl Send for GraphicsCaptureItem {}
unsafe impl Sync for GraphicsCaptureItem {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GraphicsCapturePicker(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GraphicsCapturePicker, windows_core::IUnknown, windows_core::IInspectable);
impl GraphicsCapturePicker {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GraphicsCapturePicker, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PickSingleItemAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<GraphicsCaptureItem>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PickSingleItemAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GraphicsCapturePicker {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGraphicsCapturePicker>();
}
unsafe impl windows_core::Interface for GraphicsCapturePicker {
    type Vtable = IGraphicsCapturePicker_Vtbl;
    const IID: windows_core::GUID = <IGraphicsCapturePicker as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GraphicsCapturePicker {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCapturePicker";
}
unsafe impl Send for GraphicsCapturePicker {}
unsafe impl Sync for GraphicsCapturePicker {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct GraphicsCaptureSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GraphicsCaptureSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(GraphicsCaptureSession, super::super::Foundation::IClosable);
impl GraphicsCaptureSession {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StartCapture(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartCapture)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsCursorCaptureEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IGraphicsCaptureSession2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCursorCaptureEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCursorCaptureEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGraphicsCaptureSession2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsCursorCaptureEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsBorderRequired(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IGraphicsCaptureSession3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBorderRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsBorderRequired(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IGraphicsCaptureSession3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsBorderRequired)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IGraphicsCaptureSessionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IGraphicsCaptureSessionStatics<R, F: FnOnce(&IGraphicsCaptureSessionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GraphicsCaptureSession, IGraphicsCaptureSessionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GraphicsCaptureSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGraphicsCaptureSession>();
}
unsafe impl windows_core::Interface for GraphicsCaptureSession {
    type Vtable = IGraphicsCaptureSession_Vtbl;
    const IID: windows_core::GUID = <IGraphicsCaptureSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GraphicsCaptureSession {
    const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureSession";
}
unsafe impl Send for GraphicsCaptureSession {}
unsafe impl Sync for GraphicsCaptureSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GraphicsCaptureAccessKind(pub i32);
impl GraphicsCaptureAccessKind {
    pub const Borderless: Self = Self(0i32);
    pub const Programmatic: Self = Self(1i32);
}
impl windows_core::TypeKind for GraphicsCaptureAccessKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GraphicsCaptureAccessKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GraphicsCaptureAccessKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GraphicsCaptureAccessKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Capture.GraphicsCaptureAccessKind;i4)");
}
