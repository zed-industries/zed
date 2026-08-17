windows_core::imp::define_interface!(IPalmRejectionDelayZonePreview, IPalmRejectionDelayZonePreview_Vtbl, 0x62b496cb_539d_5343_a65f_41f5300ec70c);
impl windows_core::RuntimeType for IPalmRejectionDelayZonePreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPalmRejectionDelayZonePreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPalmRejectionDelayZonePreviewStatics, IPalmRejectionDelayZonePreviewStatics_Vtbl, 0xcdef5ee0_93d0_53a9_8f0e_9a379f8f7530);
impl windows_core::RuntimeType for IPalmRejectionDelayZonePreviewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPalmRejectionDelayZonePreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateForVisual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::super::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateForVisual: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateForVisualWithViewportClip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::super::Foundation::Rect, *mut core::ffi::c_void, super::super::super::super::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateForVisualWithViewportClip: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct PalmRejectionDelayZonePreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PalmRejectionDelayZonePreview, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PalmRejectionDelayZonePreview, super::super::super::super::Foundation::IClosable);
impl PalmRejectionDelayZonePreview {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn CreateForVisual<P0>(inputpanelvisual: P0, inputpanelrect: super::super::super::super::Foundation::Rect) -> windows_core::Result<PalmRejectionDelayZonePreview>
    where
        P0: windows_core::Param<super::super::super::Composition::Visual>,
    {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForVisual)(windows_core::Interface::as_raw(this), inputpanelvisual.param().abi(), inputpanelrect, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn CreateForVisualWithViewportClip<P0, P1>(inputpanelvisual: P0, inputpanelrect: super::super::super::super::Foundation::Rect, viewportvisual: P1, viewportrect: super::super::super::super::Foundation::Rect) -> windows_core::Result<PalmRejectionDelayZonePreview>
    where
        P0: windows_core::Param<super::super::super::Composition::Visual>,
        P1: windows_core::Param<super::super::super::Composition::Visual>,
    {
        Self::IPalmRejectionDelayZonePreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForVisualWithViewportClip)(windows_core::Interface::as_raw(this), inputpanelvisual.param().abi(), inputpanelrect, viewportvisual.param().abi(), viewportrect, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPalmRejectionDelayZonePreviewStatics<R, F: FnOnce(&IPalmRejectionDelayZonePreviewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PalmRejectionDelayZonePreview, IPalmRejectionDelayZonePreviewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PalmRejectionDelayZonePreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPalmRejectionDelayZonePreview>();
}
unsafe impl windows_core::Interface for PalmRejectionDelayZonePreview {
    type Vtable = IPalmRejectionDelayZonePreview_Vtbl;
    const IID: windows_core::GUID = <IPalmRejectionDelayZonePreview as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PalmRejectionDelayZonePreview {
    const NAME: &'static str = "Windows.UI.Input.Inking.Preview.PalmRejectionDelayZonePreview";
}
unsafe impl Send for PalmRejectionDelayZonePreview {}
unsafe impl Sync for PalmRejectionDelayZonePreview {}
