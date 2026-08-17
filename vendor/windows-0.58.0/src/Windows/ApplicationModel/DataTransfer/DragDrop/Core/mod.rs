windows_core::imp::define_interface!(ICoreDragDropManager, ICoreDragDropManager_Vtbl, 0x7d56d344_8464_4faf_aa49_37ea6e2d7bd1);
impl windows_core::RuntimeType for ICoreDragDropManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreDragDropManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TargetRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTargetRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub AreConcurrentOperationsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAreConcurrentOperationsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreDragDropManagerStatics, ICoreDragDropManagerStatics_Vtbl, 0x9542fdca_da12_4c1c_8d06_041db29733c3);
impl windows_core::RuntimeType for ICoreDragDropManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreDragDropManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreDragInfo, ICoreDragInfo_Vtbl, 0x48353a8b_cb50_464e_9575_cd4e3a7ab028);
impl windows_core::RuntimeType for ICoreDragInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreDragInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Modifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DragDropModifiers) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreDragInfo2, ICoreDragInfo2_Vtbl, 0xc54691e5_e6fb_4d74_b4b1_8a3c17f25e9e);
impl windows_core::RuntimeType for ICoreDragInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreDragInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowedOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::DataPackageOperation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreDragOperation, ICoreDragOperation_Vtbl, 0xcc06de4f_6db0_4e62_ab1b_a74a02dc6d85);
impl windows_core::RuntimeType for ICoreDragOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreDragOperation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPointerId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetDragUIContentFromSoftwareBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetDragUIContentFromSoftwareBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetDragUIContentFromSoftwareBitmapWithAnchorPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::super::Foundation::Point) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetDragUIContentFromSoftwareBitmapWithAnchorPoint: usize,
    pub DragUIContentMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreDragUIContentMode) -> windows_core::HRESULT,
    pub SetDragUIContentMode: unsafe extern "system" fn(*mut core::ffi::c_void, CoreDragUIContentMode) -> windows_core::HRESULT,
    pub StartAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreDragOperation2, ICoreDragOperation2_Vtbl, 0x824b1e2c_d99a_4fc3_8507_6c182f33b46a);
impl windows_core::RuntimeType for ICoreDragOperation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreDragOperation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowedOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::DataPackageOperation) -> windows_core::HRESULT,
    pub SetAllowedOperations: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::DataPackageOperation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreDragUIOverride, ICoreDragUIOverride_Vtbl, 0x89a85064_3389_4f4f_8897_7e8a3ffb3c93);
impl windows_core::RuntimeType for ICoreDragUIOverride {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreDragUIOverride_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetContentFromSoftwareBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetContentFromSoftwareBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetContentFromSoftwareBitmapWithAnchorPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::super::Foundation::Point) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetContentFromSoftwareBitmapWithAnchorPoint: usize,
    pub IsContentVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsContentVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Caption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetCaption: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub IsCaptionVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCaptionVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsGlyphVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsGlyphVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreDropOperationTarget, ICoreDropOperationTarget_Vtbl, 0xd9126196_4c5b_417d_bb37_76381def8db4);
impl core::ops::Deref for ICoreDropOperationTarget {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreDropOperationTarget, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreDropOperationTarget {
    pub fn EnterAsync<P0, P1>(&self, draginfo: P0, draguioverride: P1) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>
    where
        P0: windows_core::Param<CoreDragInfo>,
        P1: windows_core::Param<CoreDragUIOverride>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EnterAsync)(windows_core::Interface::as_raw(this), draginfo.param().abi(), draguioverride.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn OverAsync<P0, P1>(&self, draginfo: P0, draguioverride: P1) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>
    where
        P0: windows_core::Param<CoreDragInfo>,
        P1: windows_core::Param<CoreDragUIOverride>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OverAsync)(windows_core::Interface::as_raw(this), draginfo.param().abi(), draguioverride.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LeaveAsync<P0>(&self, draginfo: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<CoreDragInfo>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LeaveAsync)(windows_core::Interface::as_raw(this), draginfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DropAsync<P0>(&self, draginfo: P0) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>
    where
        P0: windows_core::Param<CoreDragInfo>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DropAsync)(windows_core::Interface::as_raw(this), draginfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ICoreDropOperationTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreDropOperationTarget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EnterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OverAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LeaveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DropAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreDropOperationTargetRequestedEventArgs, ICoreDropOperationTargetRequestedEventArgs_Vtbl, 0x2aca929a_5e28_4ea6_829e_29134e665d6d);
impl windows_core::RuntimeType for ICoreDropOperationTargetRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreDropOperationTargetRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreDragDropManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreDragDropManager, windows_core::IUnknown, windows_core::IInspectable);
impl CoreDragDropManager {
    pub fn TargetRequested<P0>(&self, value: P0) -> windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::super::super::Foundation::TypedEventHandler<CoreDragDropManager, CoreDropOperationTargetRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetRequested)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTargetRequested(&self, value: super::super::super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTargetRequested)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AreConcurrentOperationsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AreConcurrentOperationsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAreConcurrentOperationsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAreConcurrentOperationsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<CoreDragDropManager> {
        Self::ICoreDragDropManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICoreDragDropManagerStatics<R, F: FnOnce(&ICoreDragDropManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreDragDropManager, ICoreDragDropManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreDragDropManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreDragDropManager>();
}
unsafe impl windows_core::Interface for CoreDragDropManager {
    type Vtable = ICoreDragDropManager_Vtbl;
    const IID: windows_core::GUID = <ICoreDragDropManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreDragDropManager {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragDropManager";
}
unsafe impl Send for CoreDragDropManager {}
unsafe impl Sync for CoreDragDropManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreDragInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreDragInfo, windows_core::IUnknown, windows_core::IInspectable);
impl CoreDragInfo {
    pub fn Data(&self) -> windows_core::Result<super::super::DataPackageView> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Modifiers(&self) -> windows_core::Result<super::DragDropModifiers> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Modifiers)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AllowedOperations(&self) -> windows_core::Result<super::super::DataPackageOperation> {
        let this = &windows_core::Interface::cast::<ICoreDragInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowedOperations)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreDragInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreDragInfo>();
}
unsafe impl windows_core::Interface for CoreDragInfo {
    type Vtable = ICoreDragInfo_Vtbl;
    const IID: windows_core::GUID = <ICoreDragInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreDragInfo {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo";
}
unsafe impl Send for CoreDragInfo {}
unsafe impl Sync for CoreDragInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreDragOperation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreDragOperation, windows_core::IUnknown, windows_core::IInspectable);
impl CoreDragOperation {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreDragOperation, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Data(&self) -> windows_core::Result<super::super::DataPackage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPointerId(&self, pointerid: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPointerId)(windows_core::Interface::as_raw(this), pointerid).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetDragUIContentFromSoftwareBitmap<P0>(&self, softwarebitmap: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDragUIContentFromSoftwareBitmap)(windows_core::Interface::as_raw(this), softwarebitmap.param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetDragUIContentFromSoftwareBitmapWithAnchorPoint<P0>(&self, softwarebitmap: P0, anchorpoint: super::super::super::super::Foundation::Point) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDragUIContentFromSoftwareBitmapWithAnchorPoint)(windows_core::Interface::as_raw(this), softwarebitmap.param().abi(), anchorpoint).ok() }
    }
    pub fn DragUIContentMode(&self) -> windows_core::Result<CoreDragUIContentMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DragUIContentMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDragUIContentMode(&self, value: CoreDragUIContentMode) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDragUIContentMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StartAsync(&self) -> windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AllowedOperations(&self) -> windows_core::Result<super::super::DataPackageOperation> {
        let this = &windows_core::Interface::cast::<ICoreDragOperation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowedOperations)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowedOperations(&self, value: super::super::DataPackageOperation) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreDragOperation2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowedOperations)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for CoreDragOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreDragOperation>();
}
unsafe impl windows_core::Interface for CoreDragOperation {
    type Vtable = ICoreDragOperation_Vtbl;
    const IID: windows_core::GUID = <ICoreDragOperation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreDragOperation {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragOperation";
}
unsafe impl Send for CoreDragOperation {}
unsafe impl Sync for CoreDragOperation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreDragUIOverride(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreDragUIOverride, windows_core::IUnknown, windows_core::IInspectable);
impl CoreDragUIOverride {
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetContentFromSoftwareBitmap<P0>(&self, softwarebitmap: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentFromSoftwareBitmap)(windows_core::Interface::as_raw(this), softwarebitmap.param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetContentFromSoftwareBitmapWithAnchorPoint<P0>(&self, softwarebitmap: P0, anchorpoint: super::super::super::super::Foundation::Point) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetContentFromSoftwareBitmapWithAnchorPoint)(windows_core::Interface::as_raw(this), softwarebitmap.param().abi(), anchorpoint).ok() }
    }
    pub fn IsContentVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsContentVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsContentVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsContentVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Caption(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Caption)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCaption(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCaption)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsCaptionVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsCaptionVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCaptionVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsCaptionVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsGlyphVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGlyphVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsGlyphVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIsGlyphVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for CoreDragUIOverride {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreDragUIOverride>();
}
unsafe impl windows_core::Interface for CoreDragUIOverride {
    type Vtable = ICoreDragUIOverride_Vtbl;
    const IID: windows_core::GUID = <ICoreDragUIOverride as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreDragUIOverride {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride";
}
unsafe impl Send for CoreDragUIOverride {}
unsafe impl Sync for CoreDragUIOverride {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CoreDropOperationTargetRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreDropOperationTargetRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreDropOperationTargetRequestedEventArgs {
    pub fn SetTarget<P0>(&self, target: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICoreDropOperationTarget>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTarget)(windows_core::Interface::as_raw(this), target.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for CoreDropOperationTargetRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreDropOperationTargetRequestedEventArgs>();
}
unsafe impl windows_core::Interface for CoreDropOperationTargetRequestedEventArgs {
    type Vtable = ICoreDropOperationTargetRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <ICoreDropOperationTargetRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreDropOperationTargetRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDropOperationTargetRequestedEventArgs";
}
unsafe impl Send for CoreDropOperationTargetRequestedEventArgs {}
unsafe impl Sync for CoreDropOperationTargetRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoreDragUIContentMode(pub u32);
impl CoreDragUIContentMode {
    pub const Auto: Self = Self(0u32);
    pub const Deferred: Self = Self(1u32);
}
impl windows_core::TypeKind for CoreDragUIContentMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoreDragUIContentMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoreDragUIContentMode").field(&self.0).finish()
    }
}
impl CoreDragUIContentMode {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CoreDragUIContentMode {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CoreDragUIContentMode {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CoreDragUIContentMode {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CoreDragUIContentMode {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CoreDragUIContentMode {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for CoreDragUIContentMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIContentMode;u4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
