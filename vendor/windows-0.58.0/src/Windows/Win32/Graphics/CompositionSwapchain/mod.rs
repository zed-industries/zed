#[inline]
pub unsafe fn CreatePresentationFactory<P0>(d3ddevice: P0, riid: *const windows_core::GUID, presentationfactory: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("dcomp.dll" "system" fn CreatePresentationFactory(d3ddevice : * mut core::ffi::c_void, riid : *const windows_core::GUID, presentationfactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CreatePresentationFactory(d3ddevice.param().abi(), riid, presentationfactory).ok()
}
windows_core::imp::define_interface!(ICompositionFramePresentStatistics, ICompositionFramePresentStatistics_Vtbl, 0xab41d127_c101_4c0a_911d_f9f2e9d08e64);
impl core::ops::Deref for ICompositionFramePresentStatistics {
    type Target = IPresentStatistics;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICompositionFramePresentStatistics, windows_core::IUnknown, IPresentStatistics);
impl ICompositionFramePresentStatistics {
    pub unsafe fn GetContentTag(&self) -> usize {
        (windows_core::Interface::vtable(self).GetContentTag)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetCompositionFrameId(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetCompositionFrameId)(windows_core::Interface::as_raw(self))
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayInstanceArray(&self, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance) {
        (windows_core::Interface::vtable(self).GetDisplayInstanceArray)(windows_core::Interface::as_raw(self), displayinstancearraycount, displayinstancearray)
    }
}
#[repr(C)]
pub struct ICompositionFramePresentStatistics_Vtbl {
    pub base__: IPresentStatistics_Vtbl,
    pub GetContentTag: unsafe extern "system" fn(*mut core::ffi::c_void) -> usize,
    pub GetCompositionFrameId: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDisplayInstanceArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut CompositionFrameDisplayInstance),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDisplayInstanceArray: usize,
}
windows_core::imp::define_interface!(IIndependentFlipFramePresentStatistics, IIndependentFlipFramePresentStatistics_Vtbl, 0x8c93be27_ad94_4da0_8fd4_2413132d124e);
impl core::ops::Deref for IIndependentFlipFramePresentStatistics {
    type Target = IPresentStatistics;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IIndependentFlipFramePresentStatistics, windows_core::IUnknown, IPresentStatistics);
impl IIndependentFlipFramePresentStatistics {
    pub unsafe fn GetOutputAdapterLUID(&self) -> super::super::Foundation::LUID {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOutputAdapterLUID)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetOutputVidPnSourceId(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetOutputVidPnSourceId)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetContentTag(&self) -> usize {
        (windows_core::Interface::vtable(self).GetContentTag)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDisplayedTime(&self) -> SystemInterruptTime {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDisplayedTime)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetPresentDuration(&self) -> SystemInterruptTime {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPresentDuration)(windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
#[repr(C)]
pub struct IIndependentFlipFramePresentStatistics_Vtbl {
    pub base__: IPresentStatistics_Vtbl,
    pub GetOutputAdapterLUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::LUID),
    pub GetOutputVidPnSourceId: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetContentTag: unsafe extern "system" fn(*mut core::ffi::c_void) -> usize,
    pub GetDisplayedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SystemInterruptTime),
    pub GetPresentDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SystemInterruptTime),
}
windows_core::imp::define_interface!(IPresentStatistics, IPresentStatistics_Vtbl, 0xb44b8bda_7282_495d_9dd7_ceadd8b4bb86);
impl core::ops::Deref for IPresentStatistics {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPresentStatistics, windows_core::IUnknown);
impl IPresentStatistics {
    pub unsafe fn GetPresentId(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetPresentId)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        (windows_core::Interface::vtable(self).GetKind)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IPresentStatistics_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPresentId: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetKind: unsafe extern "system" fn(*mut core::ffi::c_void) -> PresentStatisticsKind,
}
windows_core::imp::define_interface!(IPresentStatusPresentStatistics, IPresentStatusPresentStatistics_Vtbl, 0xc9ed2a41_79cb_435e_964e_c8553055420c);
impl core::ops::Deref for IPresentStatusPresentStatistics {
    type Target = IPresentStatistics;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPresentStatusPresentStatistics, windows_core::IUnknown, IPresentStatistics);
impl IPresentStatusPresentStatistics {
    pub unsafe fn GetCompositionFrameId(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetCompositionFrameId)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetPresentStatus(&self) -> PresentStatus {
        (windows_core::Interface::vtable(self).GetPresentStatus)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IPresentStatusPresentStatistics_Vtbl {
    pub base__: IPresentStatistics_Vtbl,
    pub GetCompositionFrameId: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetPresentStatus: unsafe extern "system" fn(*mut core::ffi::c_void) -> PresentStatus,
}
windows_core::imp::define_interface!(IPresentationBuffer, IPresentationBuffer_Vtbl, 0x2e217d3a_5abb_4138_9a13_a775593c89ca);
impl core::ops::Deref for IPresentationBuffer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPresentationBuffer, windows_core::IUnknown);
impl IPresentationBuffer {
    pub unsafe fn GetAvailableEvent(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAvailableEvent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsAvailable(&self) -> windows_core::Result<u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IPresentationBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAvailableEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub IsAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPresentationContent, IPresentationContent_Vtbl, 0x5668bb79_3d8e_415c_b215_f38020f2d252);
impl core::ops::Deref for IPresentationContent {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPresentationContent, windows_core::IUnknown);
impl IPresentationContent {
    pub unsafe fn SetTag(&self, tag: usize) {
        (windows_core::Interface::vtable(self).SetTag)(windows_core::Interface::as_raw(self), tag)
    }
}
#[repr(C)]
pub struct IPresentationContent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, usize),
}
windows_core::imp::define_interface!(IPresentationFactory, IPresentationFactory_Vtbl, 0x8fb37b58_1d74_4f64_a49c_1f97a80a2ec0);
impl core::ops::Deref for IPresentationFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPresentationFactory, windows_core::IUnknown);
impl IPresentationFactory {
    pub unsafe fn IsPresentationSupported(&self) -> u8 {
        (windows_core::Interface::vtable(self).IsPresentationSupported)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsPresentationSupportedWithIndependentFlip(&self) -> u8 {
        (windows_core::Interface::vtable(self).IsPresentationSupportedWithIndependentFlip)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn CreatePresentationManager(&self) -> windows_core::Result<IPresentationManager> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePresentationManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPresentationFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsPresentationSupported: unsafe extern "system" fn(*mut core::ffi::c_void) -> u8,
    pub IsPresentationSupportedWithIndependentFlip: unsafe extern "system" fn(*mut core::ffi::c_void) -> u8,
    pub CreatePresentationManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPresentationManager, IPresentationManager_Vtbl, 0xfb562f82_6292_470a_88b1_843661e7f20c);
impl core::ops::Deref for IPresentationManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPresentationManager, windows_core::IUnknown);
impl IPresentationManager {
    pub unsafe fn AddBufferFromResource<P0>(&self, resource: P0) -> windows_core::Result<IPresentationBuffer>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddBufferFromResource)(windows_core::Interface::as_raw(self), resource.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreatePresentationSurface<P0>(&self, compositionsurfacehandle: P0) -> windows_core::Result<IPresentationSurface>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePresentationSurface)(windows_core::Interface::as_raw(self), compositionsurfacehandle.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNextPresentId(&self) -> u64 {
        (windows_core::Interface::vtable(self).GetNextPresentId)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetTargetTime(&self, targettime: SystemInterruptTime) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTargetTime)(windows_core::Interface::as_raw(self), core::mem::transmute(targettime)).ok()
    }
    pub unsafe fn SetPreferredPresentDuration(&self, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPreferredPresentDuration)(windows_core::Interface::as_raw(self), core::mem::transmute(preferredduration), core::mem::transmute(deviationtolerance)).ok()
    }
    pub unsafe fn ForceVSyncInterrupt(&self, forcevsyncinterrupt: u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ForceVSyncInterrupt)(windows_core::Interface::as_raw(self), forcevsyncinterrupt).ok()
    }
    pub unsafe fn Present(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetPresentRetiringFence(&self, riid: *const windows_core::GUID) -> windows_core::Result<*mut core::ffi::c_void> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPresentRetiringFence)(windows_core::Interface::as_raw(self), riid, &mut result__).map(|| result__)
    }
    pub unsafe fn CancelPresentsFrom(&self, presentidtocancelfrom: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelPresentsFrom)(windows_core::Interface::as_raw(self), presentidtocancelfrom).ok()
    }
    pub unsafe fn GetLostEvent(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLostEvent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPresentStatisticsAvailableEvent(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPresentStatisticsAvailableEvent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnablePresentStatisticsKind(&self, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnablePresentStatisticsKind)(windows_core::Interface::as_raw(self), presentstatisticskind, enabled).ok()
    }
    pub unsafe fn GetNextPresentStatistics(&self) -> windows_core::Result<IPresentStatistics> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNextPresentStatistics)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPresentationManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddBufferFromResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePresentationSurface: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextPresentId: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub SetTargetTime: unsafe extern "system" fn(*mut core::ffi::c_void, SystemInterruptTime) -> windows_core::HRESULT,
    pub SetPreferredPresentDuration: unsafe extern "system" fn(*mut core::ffi::c_void, SystemInterruptTime, SystemInterruptTime) -> windows_core::HRESULT,
    pub ForceVSyncInterrupt: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub Present: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPresentRetiringFence: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelPresentsFrom: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub GetLostEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub GetPresentStatisticsAvailableEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub EnablePresentStatisticsKind: unsafe extern "system" fn(*mut core::ffi::c_void, PresentStatisticsKind, u8) -> windows_core::HRESULT,
    pub GetNextPresentStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPresentationSurface, IPresentationSurface_Vtbl, 0x956710fb_ea40_4eba_a3eb_4375a0eb4edc);
impl core::ops::Deref for IPresentationSurface {
    type Target = IPresentationContent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPresentationSurface, windows_core::IUnknown, IPresentationContent);
impl IPresentationSurface {
    pub unsafe fn SetBuffer<P0>(&self, presentationbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPresentationBuffer>,
    {
        (windows_core::Interface::vtable(self).SetBuffer)(windows_core::Interface::as_raw(self), presentationbuffer.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColorSpace)(windows_core::Interface::as_raw(self), colorspace).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetAlphaMode(&self, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAlphaMode)(windows_core::Interface::as_raw(self), alphamode).ok()
    }
    pub unsafe fn SetSourceRect(&self, sourcerect: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSourceRect)(windows_core::Interface::as_raw(self), sourcerect).ok()
    }
    pub unsafe fn SetTransform(&self, transform: *const PresentationTransform) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform).ok()
    }
    pub unsafe fn RestrictToOutput<P0>(&self, output: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).RestrictToOutput)(windows_core::Interface::as_raw(self), output.param().abi()).ok()
    }
    pub unsafe fn SetDisableReadback(&self, value: u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDisableReadback)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn SetLetterboxingMargins(&self, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLetterboxingMargins)(windows_core::Interface::as_raw(self), leftletterboxsize, topletterboxsize, rightletterboxsize, bottomletterboxsize).ok()
    }
}
#[repr(C)]
pub struct IPresentationSurface_Vtbl {
    pub base__: IPresentationContent_Vtbl,
    pub SetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetColorSpace: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetAlphaMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetAlphaMode: usize,
    pub SetSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const PresentationTransform) -> windows_core::HRESULT,
    pub RestrictToOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisableReadback: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub SetLetterboxingMargins: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32) -> windows_core::HRESULT,
}
pub const CompositionFrameInstanceKind_ComposedOnScreen: CompositionFrameInstanceKind = CompositionFrameInstanceKind(0i32);
pub const CompositionFrameInstanceKind_ComposedToIntermediate: CompositionFrameInstanceKind = CompositionFrameInstanceKind(2i32);
pub const CompositionFrameInstanceKind_ScanoutOnScreen: CompositionFrameInstanceKind = CompositionFrameInstanceKind(1i32);
pub const PresentStatisticsKind_CompositionFrame: PresentStatisticsKind = PresentStatisticsKind(2i32);
pub const PresentStatisticsKind_IndependentFlipFrame: PresentStatisticsKind = PresentStatisticsKind(3i32);
pub const PresentStatisticsKind_PresentStatus: PresentStatisticsKind = PresentStatisticsKind(1i32);
pub const PresentStatus_Canceled: PresentStatus = PresentStatus(2i32);
pub const PresentStatus_Queued: PresentStatus = PresentStatus(0i32);
pub const PresentStatus_Skipped: PresentStatus = PresentStatus(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CompositionFrameInstanceKind(pub i32);
impl windows_core::TypeKind for CompositionFrameInstanceKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CompositionFrameInstanceKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CompositionFrameInstanceKind").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PresentStatisticsKind(pub i32);
impl windows_core::TypeKind for PresentStatisticsKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PresentStatisticsKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PresentStatisticsKind").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PresentStatus(pub i32);
impl windows_core::TypeKind for PresentStatus {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PresentStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PresentStatus").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CompositionFrameDisplayInstance {
    pub displayAdapterLUID: super::super::Foundation::LUID,
    pub displayVidPnSourceId: u32,
    pub displayUniqueId: u32,
    pub renderAdapterLUID: super::super::Foundation::LUID,
    pub instanceKind: CompositionFrameInstanceKind,
    pub finalTransform: PresentationTransform,
    pub requiredCrossAdapterCopy: u8,
    pub colorSpace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for CompositionFrameDisplayInstance {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for CompositionFrameDisplayInstance {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PresentationTransform {
    pub M11: f32,
    pub M12: f32,
    pub M21: f32,
    pub M22: f32,
    pub M31: f32,
    pub M32: f32,
}
impl windows_core::TypeKind for PresentationTransform {
    type TypeKind = windows_core::CopyType;
}
impl Default for PresentationTransform {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SystemInterruptTime {
    pub value: u64,
}
impl windows_core::TypeKind for SystemInterruptTime {
    type TypeKind = windows_core::CopyType;
}
impl Default for SystemInterruptTime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
