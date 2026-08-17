windows_core::imp::define_interface!(ICoreIncrementalInkStroke, ICoreIncrementalInkStroke_Vtbl, 0xfda015d3_9d66_4f7d_a57f_cc70b9cfaa76);
impl windows_core::RuntimeType for ICoreIncrementalInkStroke {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreIncrementalInkStroke_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppendInkPoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppendInkPoints: usize,
    pub CreateInkStroke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DrawingAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub PointTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PointTransform: usize,
    pub BoundingRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::Foundation::Rect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreIncrementalInkStrokeFactory, ICoreIncrementalInkStrokeFactory_Vtbl, 0xd7c59f46_8da8_4f70_9751_e53bb6df4596);
impl windows_core::RuntimeType for ICoreIncrementalInkStrokeFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreIncrementalInkStrokeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::super::Foundation::Numerics::Matrix3x2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Create: usize,
}
windows_core::imp::define_interface!(ICoreInkIndependentInputSource, ICoreInkIndependentInputSource_Vtbl, 0x39b38da9_7639_4499_a5b5_191d00e35b16);
impl windows_core::RuntimeType for ICoreInkIndependentInputSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInkIndependentInputSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Core")]
    pub PointerEntering: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerEntering: usize,
    pub RemovePointerEntering: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub PointerHovering: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerHovering: usize,
    pub RemovePointerHovering: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub PointerExiting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerExiting: usize,
    pub RemovePointerExiting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub PointerPressing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerPressing: usize,
    pub RemovePointerPressing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub PointerMoving: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerMoving: usize,
    pub RemovePointerMoving: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub PointerReleasing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerReleasing: usize,
    pub RemovePointerReleasing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub PointerLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerLost: usize,
    pub RemovePointerLost: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub InkPresenter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInkIndependentInputSource2, ICoreInkIndependentInputSource2_Vtbl, 0x2846b012_0b59_5bb9_a3c5_becb7cf03a33);
impl windows_core::RuntimeType for ICoreInkIndependentInputSource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInkIndependentInputSource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Core")]
    pub PointerCursor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    PointerCursor: usize,
    #[cfg(feature = "UI_Core")]
    pub SetPointerCursor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    SetPointerCursor: usize,
}
windows_core::imp::define_interface!(ICoreInkIndependentInputSourceStatics, ICoreInkIndependentInputSourceStatics_Vtbl, 0x73e6011b_80c0_4dfb_9b66_10ba7f3f9c84);
impl windows_core::RuntimeType for ICoreInkIndependentInputSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInkIndependentInputSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInkPresenterHost, ICoreInkPresenterHost_Vtbl, 0x396e89e6_7d55_4617_9e58_68c70c9169b9);
impl windows_core::RuntimeType for ICoreInkPresenterHost {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreInkPresenterHost_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InkPresenter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub RootVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    RootVisual: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetRootVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetRootVisual: usize,
}
windows_core::imp::define_interface!(ICoreWetStrokeUpdateEventArgs, ICoreWetStrokeUpdateEventArgs_Vtbl, 0xfb07d14c_3380_457a_a987_991357896c1b);
impl windows_core::RuntimeType for ICoreWetStrokeUpdateEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWetStrokeUpdateEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub NewInkPoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NewInkPoints: usize,
    pub PointerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Disposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreWetStrokeDisposition) -> windows_core::HRESULT,
    pub SetDisposition: unsafe extern "system" fn(*mut core::ffi::c_void, CoreWetStrokeDisposition) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWetStrokeUpdateSource, ICoreWetStrokeUpdateSource_Vtbl, 0x1f718e22_ee52_4e00_8209_4c3e5b21a3cc);
impl windows_core::RuntimeType for ICoreWetStrokeUpdateSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWetStrokeUpdateSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WetStrokeStarting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveWetStrokeStarting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub WetStrokeContinuing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveWetStrokeContinuing: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub WetStrokeStopping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveWetStrokeStopping: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub WetStrokeCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveWetStrokeCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub WetStrokeCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveWetStrokeCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub InkPresenter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreWetStrokeUpdateSourceStatics, ICoreWetStrokeUpdateSourceStatics_Vtbl, 0x3dad9cba_1d3d_46ae_ab9d_8647486c6f90);
impl windows_core::RuntimeType for ICoreWetStrokeUpdateSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreWetStrokeUpdateSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CoreIncrementalInkStroke(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreIncrementalInkStroke, windows_core::IUnknown, windows_core::IInspectable);
impl CoreIncrementalInkStroke {
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppendInkPoints<P0>(&self, inkpoints: P0) -> windows_core::Result<super::super::super::super::Foundation::Rect>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::Collections::IIterable<super::InkPoint>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendInkPoints)(windows_core::Interface::as_raw(this), inkpoints.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInkStroke(&self) -> windows_core::Result<super::InkStroke> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInkStroke)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DrawingAttributes(&self) -> windows_core::Result<super::InkDrawingAttributes> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DrawingAttributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PointTransform(&self) -> windows_core::Result<super::super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointTransform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BoundingRect(&self) -> windows_core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BoundingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Create<P0>(drawingattributes: P0, pointtransform: super::super::super::super::Foundation::Numerics::Matrix3x2) -> windows_core::Result<CoreIncrementalInkStroke>
    where
        P0: windows_core::Param<super::InkDrawingAttributes>,
    {
        Self::ICoreIncrementalInkStrokeFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), drawingattributes.param().abi(), pointtransform, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICoreIncrementalInkStrokeFactory<R, F: FnOnce(&ICoreIncrementalInkStrokeFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreIncrementalInkStroke, ICoreIncrementalInkStrokeFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreIncrementalInkStroke {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreIncrementalInkStroke>();
}
unsafe impl windows_core::Interface for CoreIncrementalInkStroke {
    type Vtable = ICoreIncrementalInkStroke_Vtbl;
    const IID: windows_core::GUID = <ICoreIncrementalInkStroke as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreIncrementalInkStroke {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreIncrementalInkStroke";
}
unsafe impl Send for CoreIncrementalInkStroke {}
unsafe impl Sync for CoreIncrementalInkStroke {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CoreInkIndependentInputSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInkIndependentInputSource, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInkIndependentInputSource {
    #[cfg(feature = "UI_Core")]
    pub fn PointerEntering<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerEntering)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerEntering(&self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerEntering)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn PointerHovering<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerHovering)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerHovering(&self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerHovering)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn PointerExiting<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerExiting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerExiting(&self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerExiting)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn PointerPressing<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerPressing)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerPressing(&self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerPressing)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn PointerMoving<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerMoving)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerMoving(&self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerMoving)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn PointerReleasing<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerReleasing)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerReleasing(&self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerReleasing)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn PointerLost<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<CoreInkIndependentInputSource, super::super::super::Core::PointerEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerLost)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePointerLost(&self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePointerLost)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn InkPresenter(&self) -> windows_core::Result<super::InkPresenter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InkPresenter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn PointerCursor(&self) -> windows_core::Result<super::super::super::Core::CoreCursor> {
        let this = &windows_core::Interface::cast::<ICoreInkIndependentInputSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn SetPointerCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Core::CoreCursor>,
    {
        let this = &windows_core::Interface::cast::<ICoreInkIndependentInputSource2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPointerCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Create<P0>(inkpresenter: P0) -> windows_core::Result<CoreInkIndependentInputSource>
    where
        P0: windows_core::Param<super::InkPresenter>,
    {
        Self::ICoreInkIndependentInputSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), inkpresenter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICoreInkIndependentInputSourceStatics<R, F: FnOnce(&ICoreInkIndependentInputSourceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreInkIndependentInputSource, ICoreInkIndependentInputSourceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreInkIndependentInputSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInkIndependentInputSource>();
}
unsafe impl windows_core::Interface for CoreInkIndependentInputSource {
    type Vtable = ICoreInkIndependentInputSource_Vtbl;
    const IID: windows_core::GUID = <ICoreInkIndependentInputSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInkIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreInkIndependentInputSource";
}
unsafe impl Send for CoreInkIndependentInputSource {}
unsafe impl Sync for CoreInkIndependentInputSource {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CoreInkPresenterHost(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInkPresenterHost, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInkPresenterHost {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreInkPresenterHost, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn InkPresenter(&self) -> windows_core::Result<super::InkPresenter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InkPresenter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn RootVisual(&self) -> windows_core::Result<super::super::super::Composition::ContainerVisual> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RootVisual)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn SetRootVisual<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Composition::ContainerVisual>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetRootVisual)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for CoreInkPresenterHost {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInkPresenterHost>();
}
unsafe impl windows_core::Interface for CoreInkPresenterHost {
    type Vtable = ICoreInkPresenterHost_Vtbl;
    const IID: windows_core::GUID = <ICoreInkPresenterHost as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInkPresenterHost {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreInkPresenterHost";
}
unsafe impl Send for CoreInkPresenterHost {}
unsafe impl Sync for CoreInkPresenterHost {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CoreWetStrokeUpdateEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreWetStrokeUpdateEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreWetStrokeUpdateEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn NewInkPoints(&self) -> windows_core::Result<super::super::super::super::Foundation::Collections::IVector<super::InkPoint>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NewInkPoints)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PointerId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Disposition(&self) -> windows_core::Result<CoreWetStrokeDisposition> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Disposition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDisposition(&self, value: CoreWetStrokeDisposition) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDisposition)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for CoreWetStrokeUpdateEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreWetStrokeUpdateEventArgs>();
}
unsafe impl windows_core::Interface for CoreWetStrokeUpdateEventArgs {
    type Vtable = ICoreWetStrokeUpdateEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreWetStrokeUpdateEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreWetStrokeUpdateEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateEventArgs";
}
unsafe impl Send for CoreWetStrokeUpdateEventArgs {}
unsafe impl Sync for CoreWetStrokeUpdateEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct CoreWetStrokeUpdateSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreWetStrokeUpdateSource, windows_core::IUnknown, windows_core::IInspectable);
impl CoreWetStrokeUpdateSource {
    pub fn WetStrokeStarting<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WetStrokeStarting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveWetStrokeStarting(&self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveWetStrokeStarting)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn WetStrokeContinuing<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WetStrokeContinuing)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveWetStrokeContinuing(&self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveWetStrokeContinuing)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn WetStrokeStopping<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WetStrokeStopping)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveWetStrokeStopping(&self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveWetStrokeStopping)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn WetStrokeCompleted<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WetStrokeCompleted)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveWetStrokeCompleted(&self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveWetStrokeCompleted)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn WetStrokeCanceled<P0>(&self, handler: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<CoreWetStrokeUpdateSource, CoreWetStrokeUpdateEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WetStrokeCanceled)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveWetStrokeCanceled(&self, cookie: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveWetStrokeCanceled)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn InkPresenter(&self) -> windows_core::Result<super::InkPresenter> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InkPresenter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create<P0>(inkpresenter: P0) -> windows_core::Result<CoreWetStrokeUpdateSource>
    where
        P0: windows_core::Param<super::InkPresenter>,
    {
        Self::ICoreWetStrokeUpdateSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), inkpresenter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICoreWetStrokeUpdateSourceStatics<R, F: FnOnce(&ICoreWetStrokeUpdateSourceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreWetStrokeUpdateSource, ICoreWetStrokeUpdateSourceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreWetStrokeUpdateSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreWetStrokeUpdateSource>();
}
unsafe impl windows_core::Interface for CoreWetStrokeUpdateSource {
    type Vtable = ICoreWetStrokeUpdateSource_Vtbl;
    const IID: windows_core::GUID = <ICoreWetStrokeUpdateSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreWetStrokeUpdateSource {
    const NAME: &'static str = "Windows.UI.Input.Inking.Core.CoreWetStrokeUpdateSource";
}
unsafe impl Send for CoreWetStrokeUpdateSource {}
unsafe impl Sync for CoreWetStrokeUpdateSource {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreWetStrokeDisposition(pub i32);
impl CoreWetStrokeDisposition {
    pub const Inking: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl windows_core::TypeKind for CoreWetStrokeDisposition {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreWetStrokeDisposition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreWetStrokeDisposition").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CoreWetStrokeDisposition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Core.CoreWetStrokeDisposition;i4)");
}
