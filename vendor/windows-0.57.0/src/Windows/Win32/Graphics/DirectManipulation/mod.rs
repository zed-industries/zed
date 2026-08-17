windows_core::imp::define_interface!(IDirectManipulationAutoScrollBehavior, IDirectManipulationAutoScrollBehavior_Vtbl, 0x6d5954d4_2003_4356_9b31_d051c9ff0af7);
impl core::ops::Deref for IDirectManipulationAutoScrollBehavior {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationAutoScrollBehavior, windows_core::IUnknown);
impl IDirectManipulationAutoScrollBehavior {
    pub unsafe fn SetConfiguration(&self, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetConfiguration)(windows_core::Interface::as_raw(self), motiontypes, scrollmotion).ok()
    }
}
#[repr(C)]
pub struct IDirectManipulationAutoScrollBehavior_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_MOTION_TYPES, DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationCompositor, IDirectManipulationCompositor_Vtbl, 0x537a0825_0387_4efa_b62f_71eb1f085a7e);
impl core::ops::Deref for IDirectManipulationCompositor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationCompositor, windows_core::IUnknown);
impl IDirectManipulationCompositor {
    pub unsafe fn AddContent<P0, P1, P2, P3>(&self, content: P0, device: P1, parentvisual: P2, childvisual: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectManipulationContent>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).AddContent)(windows_core::Interface::as_raw(self), content.param().abi(), device.param().abi(), parentvisual.param().abi(), childvisual.param().abi()).ok()
    }
    pub unsafe fn RemoveContent<P0>(&self, content: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectManipulationContent>,
    {
        (windows_core::Interface::vtable(self).RemoveContent)(windows_core::Interface::as_raw(self), content.param().abi()).ok()
    }
    pub unsafe fn SetUpdateManager<P0>(&self, updatemanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectManipulationUpdateManager>,
    {
        (windows_core::Interface::vtable(self).SetUpdateManager)(windows_core::Interface::as_raw(self), updatemanager.param().abi()).ok()
    }
    pub unsafe fn Flush(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDirectManipulationCompositor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUpdateManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationCompositor2, IDirectManipulationCompositor2_Vtbl, 0xd38c7822_f1cb_43cb_b4b9_ac0c767a412e);
impl core::ops::Deref for IDirectManipulationCompositor2 {
    type Target = IDirectManipulationCompositor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationCompositor2, windows_core::IUnknown, IDirectManipulationCompositor);
impl IDirectManipulationCompositor2 {
    pub unsafe fn AddContentWithCrossProcessChaining<P0, P1, P2, P3>(&self, content: P0, device: P1, parentvisual: P2, childvisual: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectManipulationPrimaryContent>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).AddContentWithCrossProcessChaining)(windows_core::Interface::as_raw(self), content.param().abi(), device.param().abi(), parentvisual.param().abi(), childvisual.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDirectManipulationCompositor2_Vtbl {
    pub base__: IDirectManipulationCompositor_Vtbl,
    pub AddContentWithCrossProcessChaining: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationContent, IDirectManipulationContent_Vtbl, 0xb89962cb_3d89_442b_bb58_5098fa0f9f16);
impl core::ops::Deref for IDirectManipulationContent {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationContent, windows_core::IUnknown);
impl IDirectManipulationContent {
    pub unsafe fn GetContentRect(&self) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContentRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetContentRect(&self, contentsize: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetContentRect)(windows_core::Interface::as_raw(self), contentsize).ok()
    }
    pub unsafe fn GetViewport<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetViewport)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTag<T>(&self, id: Option<*mut u32>, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).GetTag)(windows_core::Interface::as_raw(self), &T::IID, result__ as *mut _ as *mut _, core::mem::transmute(id.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetTag)(windows_core::Interface::as_raw(self), object.param().abi(), id).ok()
    }
    pub unsafe fn GetOutputTransform(&self, matrix: &mut [f32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOutputTransform)(windows_core::Interface::as_raw(self), core::mem::transmute(matrix.as_ptr()), matrix.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetContentTransform(&self, matrix: &mut [f32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetContentTransform)(windows_core::Interface::as_raw(self), core::mem::transmute(matrix.as_ptr()), matrix.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SyncContentTransform(&self, matrix: &[f32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SyncContentTransform)(windows_core::Interface::as_raw(self), core::mem::transmute(matrix.as_ptr()), matrix.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct IDirectManipulationContent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetContentRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub SetContentRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub GetViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetOutputTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32) -> windows_core::HRESULT,
    pub GetContentTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32) -> windows_core::HRESULT,
    pub SyncContentTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationDeferContactService, IDirectManipulationDeferContactService_Vtbl, 0x652d5c71_fe60_4a98_be70_e5f21291e7f1);
impl core::ops::Deref for IDirectManipulationDeferContactService {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationDeferContactService, windows_core::IUnknown);
impl IDirectManipulationDeferContactService {
    pub unsafe fn DeferContact(&self, pointerid: u32, timeout: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeferContact)(windows_core::Interface::as_raw(self), pointerid, timeout).ok()
    }
    pub unsafe fn CancelContact(&self, pointerid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelContact)(windows_core::Interface::as_raw(self), pointerid).ok()
    }
    pub unsafe fn CancelDeferral(&self, pointerid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelDeferral)(windows_core::Interface::as_raw(self), pointerid).ok()
    }
}
#[repr(C)]
pub struct IDirectManipulationDeferContactService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DeferContact: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub CancelContact: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CancelDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationDragDropBehavior, IDirectManipulationDragDropBehavior_Vtbl, 0x814b5af5_c2c8_4270_a9b7_a198ce8d02fa);
impl core::ops::Deref for IDirectManipulationDragDropBehavior {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationDragDropBehavior, windows_core::IUnknown);
impl IDirectManipulationDragDropBehavior {
    pub unsafe fn SetConfiguration(&self, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetConfiguration)(windows_core::Interface::as_raw(self), configuration).ok()
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<DIRECTMANIPULATION_DRAG_DROP_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDirectManipulationDragDropBehavior_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationDragDropEventHandler, IDirectManipulationDragDropEventHandler_Vtbl, 0x1fa11b10_701b_41ae_b5f2_49e36bd595aa);
impl core::ops::Deref for IDirectManipulationDragDropEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationDragDropEventHandler, windows_core::IUnknown);
impl IDirectManipulationDragDropEventHandler {
    pub unsafe fn OnDragDropStatusChange<P0>(&self, viewport: P0, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectManipulationViewport2>,
    {
        (windows_core::Interface::vtable(self).OnDragDropStatusChange)(windows_core::Interface::as_raw(self), viewport.param().abi(), current, previous).ok()
    }
}
#[repr(C)]
pub struct IDirectManipulationDragDropEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnDragDropStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DIRECTMANIPULATION_DRAG_DROP_STATUS, DIRECTMANIPULATION_DRAG_DROP_STATUS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationFrameInfoProvider, IDirectManipulationFrameInfoProvider_Vtbl, 0xfb759dba_6f4c_4c01_874e_19c8a05907f9);
impl core::ops::Deref for IDirectManipulationFrameInfoProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationFrameInfoProvider, windows_core::IUnknown);
impl IDirectManipulationFrameInfoProvider {
    pub unsafe fn GetNextFrameInfo(&self, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNextFrameInfo)(windows_core::Interface::as_raw(self), time, processtime, compositiontime).ok()
    }
}
#[repr(C)]
pub struct IDirectManipulationFrameInfoProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNextFrameInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationInteractionEventHandler, IDirectManipulationInteractionEventHandler_Vtbl, 0xe43f45b8_42b4_403e_b1f2_273b8f510830);
impl core::ops::Deref for IDirectManipulationInteractionEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationInteractionEventHandler, windows_core::IUnknown);
impl IDirectManipulationInteractionEventHandler {
    pub unsafe fn OnInteraction<P0>(&self, viewport: P0, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectManipulationViewport2>,
    {
        (windows_core::Interface::vtable(self).OnInteraction)(windows_core::Interface::as_raw(self), viewport.param().abi(), interaction).ok()
    }
}
#[repr(C)]
pub struct IDirectManipulationInteractionEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnInteraction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DIRECTMANIPULATION_INTERACTION_TYPE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationManager, IDirectManipulationManager_Vtbl, 0xfbf5d3b4_70c7_4163_9322_5a6f660d6fbc);
impl core::ops::Deref for IDirectManipulationManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationManager, windows_core::IUnknown);
impl IDirectManipulationManager {
    pub unsafe fn Activate<P0>(&self, window: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), window.param().abi()).ok()
    }
    pub unsafe fn Deactivate<P0>(&self, window: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self), window.param().abi()).ok()
    }
    pub unsafe fn RegisterHitTestTarget<P0, P1>(&self, window: P0, hittestwindow: P1, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).RegisterHitTestTarget)(windows_core::Interface::as_raw(self), window.param().abi(), hittestwindow.param().abi(), r#type).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProcessInput)(windows_core::Interface::as_raw(self), message, &mut result__).map(|| result__)
    }
    pub unsafe fn GetUpdateManager<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetUpdateManager)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateViewport<P0, P1, T>(&self, frameinfo: P0, window: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<IDirectManipulationFrameInfoProvider>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateViewport)(windows_core::Interface::as_raw(self), frameinfo.param().abi(), window.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateContent<P0, T>(&self, frameinfo: P0, clsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        P0: windows_core::Param<IDirectManipulationFrameInfoProvider>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateContent)(windows_core::Interface::as_raw(self), frameinfo.param().abi(), clsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDirectManipulationManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub RegisterHitTestTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, super::super::Foundation::HWND, DIRECTMANIPULATION_HITTEST_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub ProcessInput: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::WindowsAndMessaging::MSG, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    ProcessInput: usize,
    pub GetUpdateManager: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationManager2, IDirectManipulationManager2_Vtbl, 0xfa1005e9_3d16_484c_bfc9_62b61e56ec4e);
impl core::ops::Deref for IDirectManipulationManager2 {
    type Target = IDirectManipulationManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationManager2, windows_core::IUnknown, IDirectManipulationManager);
impl IDirectManipulationManager2 {
    pub unsafe fn CreateBehavior<T>(&self, clsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateBehavior)(windows_core::Interface::as_raw(self), clsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDirectManipulationManager2_Vtbl {
    pub base__: IDirectManipulationManager_Vtbl,
    pub CreateBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationManager3, IDirectManipulationManager3_Vtbl, 0x2cb6b33d_ffe8_488c_b750_fbdfe88dca8c);
impl core::ops::Deref for IDirectManipulationManager3 {
    type Target = IDirectManipulationManager2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationManager3, windows_core::IUnknown, IDirectManipulationManager, IDirectManipulationManager2);
impl IDirectManipulationManager3 {
    pub unsafe fn GetService<T>(&self, clsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), clsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDirectManipulationManager3_Vtbl {
    pub base__: IDirectManipulationManager2_Vtbl,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationPrimaryContent, IDirectManipulationPrimaryContent_Vtbl, 0xc12851e4_1698_4625_b9b1_7ca3ec18630b);
impl core::ops::Deref for IDirectManipulationPrimaryContent {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationPrimaryContent, windows_core::IUnknown);
impl IDirectManipulationPrimaryContent {
    pub unsafe fn SetSnapInterval(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSnapInterval)(windows_core::Interface::as_raw(self), motion, interval, offset).ok()
    }
    pub unsafe fn SetSnapPoints(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, points: Option<&[f32]>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSnapPoints)(windows_core::Interface::as_raw(self), motion, core::mem::transmute(points.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), points.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetSnapType(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSnapType)(windows_core::Interface::as_raw(self), motion, r#type).ok()
    }
    pub unsafe fn SetSnapCoordinate(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSnapCoordinate)(windows_core::Interface::as_raw(self), motion, coordinate, origin).ok()
    }
    pub unsafe fn SetZoomBoundaries(&self, zoomminimum: f32, zoommaximum: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetZoomBoundaries)(windows_core::Interface::as_raw(self), zoomminimum, zoommaximum).ok()
    }
    pub unsafe fn SetHorizontalAlignment(&self, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHorizontalAlignment)(windows_core::Interface::as_raw(self), alignment).ok()
    }
    pub unsafe fn SetVerticalAlignment(&self, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVerticalAlignment)(windows_core::Interface::as_raw(self), alignment).ok()
    }
    pub unsafe fn GetInertiaEndTransform(&self, matrix: &mut [f32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInertiaEndTransform)(windows_core::Interface::as_raw(self), core::mem::transmute(matrix.as_ptr()), matrix.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetCenterPoint(&self, centerx: *mut f32, centery: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCenterPoint)(windows_core::Interface::as_raw(self), centerx, centery).ok()
    }
}
#[repr(C)]
pub struct IDirectManipulationPrimaryContent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSnapInterval: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_MOTION_TYPES, f32, f32) -> windows_core::HRESULT,
    pub SetSnapPoints: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_MOTION_TYPES, *const f32, u32) -> windows_core::HRESULT,
    pub SetSnapType: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_MOTION_TYPES, DIRECTMANIPULATION_SNAPPOINT_TYPE) -> windows_core::HRESULT,
    pub SetSnapCoordinate: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_MOTION_TYPES, DIRECTMANIPULATION_SNAPPOINT_COORDINATE, f32) -> windows_core::HRESULT,
    pub SetZoomBoundaries: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32) -> windows_core::HRESULT,
    pub SetHorizontalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> windows_core::HRESULT,
    pub SetVerticalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_VERTICALALIGNMENT) -> windows_core::HRESULT,
    pub GetInertiaEndTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32) -> windows_core::HRESULT,
    pub GetCenterPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationUpdateHandler, IDirectManipulationUpdateHandler_Vtbl, 0x790b6337_64f8_4ff5_a269_b32bc2af27a7);
impl core::ops::Deref for IDirectManipulationUpdateHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationUpdateHandler, windows_core::IUnknown);
impl IDirectManipulationUpdateHandler {
    pub unsafe fn Update(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDirectManipulationUpdateHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationUpdateManager, IDirectManipulationUpdateManager_Vtbl, 0xb0ae62fd_be34_46e7_9caa_d361facbb9cc);
impl core::ops::Deref for IDirectManipulationUpdateManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationUpdateManager, windows_core::IUnknown);
impl IDirectManipulationUpdateManager {
    pub unsafe fn RegisterWaitHandleCallback<P0, P1>(&self, handle: P0, eventhandler: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
        P1: windows_core::Param<IDirectManipulationUpdateHandler>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterWaitHandleCallback)(windows_core::Interface::as_raw(self), handle.param().abi(), eventhandler.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn UnregisterWaitHandleCallback(&self, cookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterWaitHandleCallback)(windows_core::Interface::as_raw(self), cookie).ok()
    }
    pub unsafe fn Update<P0>(&self, frameinfo: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectManipulationFrameInfoProvider>,
    {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), frameinfo.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDirectManipulationUpdateManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterWaitHandleCallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnregisterWaitHandleCallback: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationViewport, IDirectManipulationViewport_Vtbl, 0x28b85a3d_60a0_48bd_9ba1_5ce8d9ea3a6d);
impl core::ops::Deref for IDirectManipulationViewport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationViewport, windows_core::IUnknown);
impl IDirectManipulationViewport {
    pub unsafe fn Enable(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disable(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Disable)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetContact(&self, pointerid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetContact)(windows_core::Interface::as_raw(self), pointerid).ok()
    }
    pub unsafe fn ReleaseContact(&self, pointerid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseContact)(windows_core::Interface::as_raw(self), pointerid).ok()
    }
    pub unsafe fn ReleaseAllContacts(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseAllContacts)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<DIRECTMANIPULATION_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTag<T>(&self, id: Option<*mut u32>, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        (windows_core::Interface::vtable(self).GetTag)(windows_core::Interface::as_raw(self), &T::IID, result__ as *mut _ as *mut _, core::mem::transmute(id.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetTag)(windows_core::Interface::as_raw(self), object.param().abi(), id).ok()
    }
    pub unsafe fn GetViewportRect(&self) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetViewportRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetViewportRect(&self, viewport: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetViewportRect)(windows_core::Interface::as_raw(self), viewport).ok()
    }
    pub unsafe fn ZoomToRect<P0>(&self, left: f32, top: f32, right: f32, bottom: f32, animate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ZoomToRect)(windows_core::Interface::as_raw(self), left, top, right, bottom, animate.param().abi()).ok()
    }
    pub unsafe fn SetViewportTransform(&self, matrix: &[f32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetViewportTransform)(windows_core::Interface::as_raw(self), core::mem::transmute(matrix.as_ptr()), matrix.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SyncDisplayTransform(&self, matrix: &[f32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SyncDisplayTransform)(windows_core::Interface::as_raw(self), core::mem::transmute(matrix.as_ptr()), matrix.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetPrimaryContent<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetPrimaryContent)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddContent<P0>(&self, content: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectManipulationContent>,
    {
        (windows_core::Interface::vtable(self).AddContent)(windows_core::Interface::as_raw(self), content.param().abi()).ok()
    }
    pub unsafe fn RemoveContent<P0>(&self, content: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectManipulationContent>,
    {
        (windows_core::Interface::vtable(self).RemoveContent)(windows_core::Interface::as_raw(self), content.param().abi()).ok()
    }
    pub unsafe fn SetViewportOptions(&self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetViewportOptions)(windows_core::Interface::as_raw(self), options).ok()
    }
    pub unsafe fn AddConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddConfiguration)(windows_core::Interface::as_raw(self), configuration).ok()
    }
    pub unsafe fn RemoveConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveConfiguration)(windows_core::Interface::as_raw(self), configuration).ok()
    }
    pub unsafe fn ActivateConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ActivateConfiguration)(windows_core::Interface::as_raw(self), configuration).ok()
    }
    pub unsafe fn SetManualGesture(&self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetManualGesture)(windows_core::Interface::as_raw(self), configuration).ok()
    }
    pub unsafe fn SetChaining(&self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetChaining)(windows_core::Interface::as_raw(self), enabledtypes).ok()
    }
    pub unsafe fn AddEventHandler<P0, P1>(&self, window: P0, eventhandler: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<IDirectManipulationViewportEventHandler>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddEventHandler)(windows_core::Interface::as_raw(self), window.param().abi(), eventhandler.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn RemoveEventHandler(&self, cookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveEventHandler)(windows_core::Interface::as_raw(self), cookie).ok()
    }
    pub unsafe fn SetInputMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInputMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn SetUpdateMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUpdateMode)(windows_core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Abandon(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abandon)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDirectManipulationViewport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContact: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReleaseContact: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReleaseAllContacts: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIRECTMANIPULATION_STATUS) -> windows_core::HRESULT,
    pub GetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetViewportRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub SetViewportRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub ZoomToRect: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetViewportTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32) -> windows_core::HRESULT,
    pub SyncDisplayTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32) -> windows_core::HRESULT,
    pub GetPrimaryContent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetViewportOptions: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> windows_core::HRESULT,
    pub AddConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_CONFIGURATION) -> windows_core::HRESULT,
    pub RemoveConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_CONFIGURATION) -> windows_core::HRESULT,
    pub ActivateConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_CONFIGURATION) -> windows_core::HRESULT,
    pub SetManualGesture: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> windows_core::HRESULT,
    pub SetChaining: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_MOTION_TYPES) -> windows_core::HRESULT,
    pub AddEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetInputMode: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_INPUT_MODE) -> windows_core::HRESULT,
    pub SetUpdateMode: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_INPUT_MODE) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abandon: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationViewport2, IDirectManipulationViewport2_Vtbl, 0x923ccaac_61e1_4385_b726_017af189882a);
impl core::ops::Deref for IDirectManipulationViewport2 {
    type Target = IDirectManipulationViewport;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationViewport2, windows_core::IUnknown, IDirectManipulationViewport);
impl IDirectManipulationViewport2 {
    pub unsafe fn AddBehavior<P0>(&self, behavior: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddBehavior)(windows_core::Interface::as_raw(self), behavior.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn RemoveBehavior(&self, cookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveBehavior)(windows_core::Interface::as_raw(self), cookie).ok()
    }
    pub unsafe fn RemoveAllBehaviors(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAllBehaviors)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDirectManipulationViewport2_Vtbl {
    pub base__: IDirectManipulationViewport_Vtbl,
    pub AddBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RemoveAllBehaviors: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectManipulationViewportEventHandler, IDirectManipulationViewportEventHandler_Vtbl, 0x952121da_d69f_45f9_b0f9_f23944321a6d);
impl core::ops::Deref for IDirectManipulationViewportEventHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationViewportEventHandler, windows_core::IUnknown);
impl IDirectManipulationViewportEventHandler {
    pub unsafe fn OnViewportStatusChanged<P0>(&self, viewport: P0, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectManipulationViewport>,
    {
        (windows_core::Interface::vtable(self).OnViewportStatusChanged)(windows_core::Interface::as_raw(self), viewport.param().abi(), current, previous).ok()
    }
    pub unsafe fn OnViewportUpdated<P0>(&self, viewport: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectManipulationViewport>,
    {
        (windows_core::Interface::vtable(self).OnViewportUpdated)(windows_core::Interface::as_raw(self), viewport.param().abi()).ok()
    }
    pub unsafe fn OnContentUpdated<P0, P1>(&self, viewport: P0, content: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDirectManipulationViewport>,
        P1: windows_core::Param<IDirectManipulationContent>,
    {
        (windows_core::Interface::vtable(self).OnContentUpdated)(windows_core::Interface::as_raw(self), viewport.param().abi(), content.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDirectManipulationViewportEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnViewportStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DIRECTMANIPULATION_STATUS, DIRECTMANIPULATION_STATUS) -> windows_core::HRESULT,
    pub OnViewportUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnContentUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const CLSID_AutoScrollBehavior: windows_core::GUID = windows_core::GUID::from_u128(0x26126a51_3c70_4c9a_aec2_948849eeb093);
pub const CLSID_DeferContactService: windows_core::GUID = windows_core::GUID::from_u128(0xd7b67cf4_84bb_434e_86ae_6592bbc9abd9);
pub const CLSID_DragDropConfigurationBehavior: windows_core::GUID = windows_core::GUID::from_u128(0x09b01b3e_ba6c_454d_82e8_95e352329f23);
pub const CLSID_HorizontalIndicatorContent: windows_core::GUID = windows_core::GUID::from_u128(0xe7d18cf5_3ec7_44d5_a76b_3770f3cf903d);
pub const CLSID_VerticalIndicatorContent: windows_core::GUID = windows_core::GUID::from_u128(0xa10b5f17_afe0_4aa2_91e9_3e7001d2e6b4);
pub const CLSID_VirtualViewportContent: windows_core::GUID = windows_core::GUID::from_u128(0x3206a19a_86f0_4cb4_a7f3_16e3b7e2d852);
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_FORWARD: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(1i32);
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_REVERSE: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(2i32);
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_STOP: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(0i32);
pub const DIRECTMANIPULATION_BUILDING: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(0i32);
pub const DIRECTMANIPULATION_CONFIGURATION_INTERACTION: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(1i32);
pub const DIRECTMANIPULATION_CONFIGURATION_NONE: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(0i32);
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_X: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(256i32);
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_Y: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(512i32);
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(16i32);
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING_INERTIA: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(128i32);
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_INERTIA: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(32i32);
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_X: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(2i32);
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_Y: DIRECTMANIPULATION_CONFIGURATION = DIRECTMANIPULATION_CONFIGURATION(4i32);
pub const DIRECTMANIPULATION_COORDINATE_BOUNDARY: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = DIRECTMANIPULATION_SNAPPOINT_COORDINATE(0i32);
pub const DIRECTMANIPULATION_COORDINATE_MIRRORED: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = DIRECTMANIPULATION_SNAPPOINT_COORDINATE(16i32);
pub const DIRECTMANIPULATION_COORDINATE_ORIGIN: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = DIRECTMANIPULATION_SNAPPOINT_COORDINATE(1i32);
pub const DIRECTMANIPULATION_DISABLED: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(2i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CANCELLED: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(4i32);
pub const DIRECTMANIPULATION_DRAG_DROP_COMMITTED: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(5i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HOLD_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(64i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HORIZONTAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(2i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(32i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_ONLY: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(16i32);
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_VERTICAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(1i32);
pub const DIRECTMANIPULATION_DRAG_DROP_DRAGGING: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(3i32);
pub const DIRECTMANIPULATION_DRAG_DROP_PRESELECT: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(1i32);
pub const DIRECTMANIPULATION_DRAG_DROP_READY: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(0i32);
pub const DIRECTMANIPULATION_DRAG_DROP_SELECTING: DIRECTMANIPULATION_DRAG_DROP_STATUS = DIRECTMANIPULATION_DRAG_DROP_STATUS(2i32);
pub const DIRECTMANIPULATION_ENABLED: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(1i32);
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_HORIZONTAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(16i32);
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_VERTICAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(8i32);
pub const DIRECTMANIPULATION_GESTURE_DEFAULT: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(0i32);
pub const DIRECTMANIPULATION_GESTURE_NONE: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(0i32);
pub const DIRECTMANIPULATION_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_GESTURE_CONFIGURATION = DIRECTMANIPULATION_GESTURE_CONFIGURATION(32i32);
pub const DIRECTMANIPULATION_HITTEST_TYPE_ASYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = DIRECTMANIPULATION_HITTEST_TYPE(0i32);
pub const DIRECTMANIPULATION_HITTEST_TYPE_AUTO_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = DIRECTMANIPULATION_HITTEST_TYPE(2i32);
pub const DIRECTMANIPULATION_HITTEST_TYPE_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = DIRECTMANIPULATION_HITTEST_TYPE(1i32);
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_CENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(2i32);
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_LEFT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(1i32);
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_NONE: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(0i32);
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_RIGHT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(4i32);
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = DIRECTMANIPULATION_HORIZONTALALIGNMENT(8i32);
pub const DIRECTMANIPULATION_INERTIA: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(4i32);
pub const DIRECTMANIPULATION_INPUT_MODE_AUTOMATIC: DIRECTMANIPULATION_INPUT_MODE = DIRECTMANIPULATION_INPUT_MODE(0i32);
pub const DIRECTMANIPULATION_INPUT_MODE_MANUAL: DIRECTMANIPULATION_INPUT_MODE = DIRECTMANIPULATION_INPUT_MODE(1i32);
pub const DIRECTMANIPULATION_INTERACTION_BEGIN: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(0i32);
pub const DIRECTMANIPULATION_INTERACTION_END: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(100i32);
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_CROSS_SLIDE: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(4i32);
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_HOLD: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(3i32);
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(5i32);
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_TAP: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(2i32);
pub const DIRECTMANIPULATION_INTERACTION_TYPE_MANIPULATION: DIRECTMANIPULATION_INTERACTION_TYPE = DIRECTMANIPULATION_INTERACTION_TYPE(1i32);
pub const DIRECTMANIPULATION_KEYBOARDFOCUS: u32 = 4294967294u32;
pub const DIRECTMANIPULATION_MOTION_ALL: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(55i32);
pub const DIRECTMANIPULATION_MOTION_CENTERX: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(16i32);
pub const DIRECTMANIPULATION_MOTION_CENTERY: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(32i32);
pub const DIRECTMANIPULATION_MOTION_NONE: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(0i32);
pub const DIRECTMANIPULATION_MOTION_TRANSLATEX: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(1i32);
pub const DIRECTMANIPULATION_MOTION_TRANSLATEY: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(2i32);
pub const DIRECTMANIPULATION_MOTION_ZOOM: DIRECTMANIPULATION_MOTION_TYPES = DIRECTMANIPULATION_MOTION_TYPES(4i32);
pub const DIRECTMANIPULATION_MOUSEFOCUS: u32 = 4294967293u32;
pub const DIRECTMANIPULATION_READY: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(5i32);
pub const DIRECTMANIPULATION_RUNNING: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(3i32);
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(0i32);
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(2i32);
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(1i32);
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = DIRECTMANIPULATION_SNAPPOINT_TYPE(3i32);
pub const DIRECTMANIPULATION_SUSPENDED: DIRECTMANIPULATION_STATUS = DIRECTMANIPULATION_STATUS(6i32);
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_BOTTOM: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(4i32);
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_CENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(2i32);
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_NONE: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(0i32);
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_TOP: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(1i32);
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = DIRECTMANIPULATION_VERTICALALIGNMENT(8i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_AUTODISABLE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(1i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DEFAULT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(0i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DISABLEPIXELSNAPPING: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(16i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_EXPLICITHITTEST: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(8i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_INPUT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(4i32);
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_MANUALUPDATE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = DIRECTMANIPULATION_VIEWPORT_OPTIONS(2i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION(pub i32);
impl windows_core::TypeKind for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTMANIPULATION_CONFIGURATION(pub i32);
impl windows_core::TypeKind for DIRECTMANIPULATION_CONFIGURATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTMANIPULATION_CONFIGURATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_CONFIGURATION").field(&self.0).finish()
    }
}
impl DIRECTMANIPULATION_CONFIGURATION {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DIRECTMANIPULATION_CONFIGURATION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DIRECTMANIPULATION_CONFIGURATION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DIRECTMANIPULATION_CONFIGURATION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DIRECTMANIPULATION_CONFIGURATION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DIRECTMANIPULATION_CONFIGURATION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION(pub i32);
impl windows_core::TypeKind for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION").field(&self.0).finish()
    }
}
impl DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTMANIPULATION_DRAG_DROP_STATUS(pub i32);
impl windows_core::TypeKind for DIRECTMANIPULATION_DRAG_DROP_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTMANIPULATION_DRAG_DROP_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_DRAG_DROP_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTMANIPULATION_GESTURE_CONFIGURATION(pub i32);
impl windows_core::TypeKind for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_GESTURE_CONFIGURATION").field(&self.0).finish()
    }
}
impl DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DIRECTMANIPULATION_GESTURE_CONFIGURATION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTMANIPULATION_HITTEST_TYPE(pub i32);
impl windows_core::TypeKind for DIRECTMANIPULATION_HITTEST_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTMANIPULATION_HITTEST_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_HITTEST_TYPE").field(&self.0).finish()
    }
}
impl DIRECTMANIPULATION_HITTEST_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DIRECTMANIPULATION_HITTEST_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DIRECTMANIPULATION_HITTEST_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DIRECTMANIPULATION_HITTEST_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DIRECTMANIPULATION_HITTEST_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DIRECTMANIPULATION_HITTEST_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTMANIPULATION_HORIZONTALALIGNMENT(pub i32);
impl windows_core::TypeKind for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_HORIZONTALALIGNMENT").field(&self.0).finish()
    }
}
impl DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DIRECTMANIPULATION_HORIZONTALALIGNMENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTMANIPULATION_INPUT_MODE(pub i32);
impl windows_core::TypeKind for DIRECTMANIPULATION_INPUT_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTMANIPULATION_INPUT_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_INPUT_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTMANIPULATION_INTERACTION_TYPE(pub i32);
impl windows_core::TypeKind for DIRECTMANIPULATION_INTERACTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTMANIPULATION_INTERACTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_INTERACTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTMANIPULATION_MOTION_TYPES(pub i32);
impl windows_core::TypeKind for DIRECTMANIPULATION_MOTION_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTMANIPULATION_MOTION_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_MOTION_TYPES").field(&self.0).finish()
    }
}
impl DIRECTMANIPULATION_MOTION_TYPES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DIRECTMANIPULATION_MOTION_TYPES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DIRECTMANIPULATION_MOTION_TYPES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DIRECTMANIPULATION_MOTION_TYPES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DIRECTMANIPULATION_MOTION_TYPES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DIRECTMANIPULATION_MOTION_TYPES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTMANIPULATION_SNAPPOINT_COORDINATE(pub i32);
impl windows_core::TypeKind for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_SNAPPOINT_COORDINATE").field(&self.0).finish()
    }
}
impl DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DIRECTMANIPULATION_SNAPPOINT_COORDINATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTMANIPULATION_SNAPPOINT_TYPE(pub i32);
impl windows_core::TypeKind for DIRECTMANIPULATION_SNAPPOINT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTMANIPULATION_SNAPPOINT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_SNAPPOINT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTMANIPULATION_STATUS(pub i32);
impl windows_core::TypeKind for DIRECTMANIPULATION_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTMANIPULATION_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTMANIPULATION_VERTICALALIGNMENT(pub i32);
impl windows_core::TypeKind for DIRECTMANIPULATION_VERTICALALIGNMENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTMANIPULATION_VERTICALALIGNMENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_VERTICALALIGNMENT").field(&self.0).finish()
    }
}
impl DIRECTMANIPULATION_VERTICALALIGNMENT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DIRECTMANIPULATION_VERTICALALIGNMENT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DIRECTMANIPULATION_VERTICALALIGNMENT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DIRECTMANIPULATION_VERTICALALIGNMENT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DIRECTMANIPULATION_VERTICALALIGNMENT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DIRECTMANIPULATION_VERTICALALIGNMENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DIRECTMANIPULATION_VIEWPORT_OPTIONS(pub i32);
impl windows_core::TypeKind for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DIRECTMANIPULATION_VIEWPORT_OPTIONS").field(&self.0).finish()
    }
}
impl DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DIRECTMANIPULATION_VIEWPORT_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DCompManipulationCompositor: windows_core::GUID = windows_core::GUID::from_u128(0x79dea627_a08a_43ac_8ef5_6900b9299126);
pub const DirectManipulationManager: windows_core::GUID = windows_core::GUID::from_u128(0x54e211b6_3650_4f75_8334_fa359598e1c5);
pub const DirectManipulationPrimaryContent: windows_core::GUID = windows_core::GUID::from_u128(0xcaa02661_d59e_41c7_8393_3ba3bacb6b57);
pub const DirectManipulationSharedManager: windows_core::GUID = windows_core::GUID::from_u128(0x99793286_77cc_4b57_96db_3b354f6f9fb5);
pub const DirectManipulationUpdateManager: windows_core::GUID = windows_core::GUID::from_u128(0x9fc1bfd5_1835_441a_b3b1_b6cc74b727d0);
pub const DirectManipulationViewport: windows_core::GUID = windows_core::GUID::from_u128(0x34e211b6_3650_4f75_8334_fa359598e1c5);
#[cfg(feature = "implement")]
core::include!("impl.rs");
