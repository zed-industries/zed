#[inline]
pub unsafe fn CreatePresentationFactory<P0, T>(d3ddevice: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("dcomp.dll" "system" fn CreatePresentationFactory(d3ddevice : * mut core::ffi::c_void, riid : *const windows_core::GUID, presentationfactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CreatePresentationFactory(d3ddevice.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompositionFrameInstanceKind(pub i32);
pub const CompositionFrameInstanceKind_ComposedOnScreen: CompositionFrameInstanceKind = CompositionFrameInstanceKind(0i32);
pub const CompositionFrameInstanceKind_ComposedToIntermediate: CompositionFrameInstanceKind = CompositionFrameInstanceKind(2i32);
pub const CompositionFrameInstanceKind_ScanoutOnScreen: CompositionFrameInstanceKind = CompositionFrameInstanceKind(1i32);
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
        unsafe { (windows_core::Interface::vtable(self).GetContentTag)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCompositionFrameId(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetCompositionFrameId)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayInstanceArray(&self, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance) {
        unsafe { (windows_core::Interface::vtable(self).GetDisplayInstanceArray)(windows_core::Interface::as_raw(self), displayinstancearraycount as _, displayinstancearray as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionFramePresentStatistics_Vtbl {
    pub base__: IPresentStatistics_Vtbl,
    pub GetContentTag: unsafe extern "system" fn(*mut core::ffi::c_void) -> usize,
    pub GetCompositionFrameId: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDisplayInstanceArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut CompositionFrameDisplayInstance),
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDisplayInstanceArray: usize,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ICompositionFramePresentStatistics_Impl: IPresentStatistics_Impl {
    fn GetContentTag(&self) -> usize;
    fn GetCompositionFrameId(&self) -> u64;
    fn GetDisplayInstanceArray(&self, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ICompositionFramePresentStatistics_Vtbl {
    pub const fn new<Identity: ICompositionFramePresentStatistics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContentTag<Identity: ICompositionFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionFramePresentStatistics_Impl::GetContentTag(this)
            }
        }
        unsafe extern "system" fn GetCompositionFrameId<Identity: ICompositionFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionFramePresentStatistics_Impl::GetCompositionFrameId(this)
            }
        }
        unsafe extern "system" fn GetDisplayInstanceArray<Identity: ICompositionFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionFramePresentStatistics_Impl::GetDisplayInstanceArray(this, core::mem::transmute_copy(&displayinstancearraycount), core::mem::transmute_copy(&displayinstancearray))
            }
        }
        Self {
            base__: IPresentStatistics_Vtbl::new::<Identity, OFFSET>(),
            GetContentTag: GetContentTag::<Identity, OFFSET>,
            GetCompositionFrameId: GetCompositionFrameId::<Identity, OFFSET>,
            GetDisplayInstanceArray: GetDisplayInstanceArray::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionFramePresentStatistics as windows_core::Interface>::IID || iid == &<IPresentStatistics as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ICompositionFramePresentStatistics {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputAdapterLUID)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn GetOutputVidPnSourceId(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetOutputVidPnSourceId)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetContentTag(&self) -> usize {
        unsafe { (windows_core::Interface::vtable(self).GetContentTag)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDisplayedTime(&self) -> SystemInterruptTime {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayedTime)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn GetPresentDuration(&self) -> SystemInterruptTime {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPresentDuration)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndependentFlipFramePresentStatistics_Vtbl {
    pub base__: IPresentStatistics_Vtbl,
    pub GetOutputAdapterLUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::LUID),
    pub GetOutputVidPnSourceId: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetContentTag: unsafe extern "system" fn(*mut core::ffi::c_void) -> usize,
    pub GetDisplayedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SystemInterruptTime),
    pub GetPresentDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SystemInterruptTime),
}
pub trait IIndependentFlipFramePresentStatistics_Impl: IPresentStatistics_Impl {
    fn GetOutputAdapterLUID(&self) -> super::super::Foundation::LUID;
    fn GetOutputVidPnSourceId(&self) -> u32;
    fn GetContentTag(&self) -> usize;
    fn GetDisplayedTime(&self) -> SystemInterruptTime;
    fn GetPresentDuration(&self) -> SystemInterruptTime;
}
impl IIndependentFlipFramePresentStatistics_Vtbl {
    pub const fn new<Identity: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOutputAdapterLUID<Identity: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = IIndependentFlipFramePresentStatistics_Impl::GetOutputAdapterLUID(this)
            }
        }
        unsafe extern "system" fn GetOutputVidPnSourceId<Identity: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIndependentFlipFramePresentStatistics_Impl::GetOutputVidPnSourceId(this)
            }
        }
        unsafe extern "system" fn GetContentTag<Identity: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIndependentFlipFramePresentStatistics_Impl::GetContentTag(this)
            }
        }
        unsafe extern "system" fn GetDisplayedTime<Identity: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SystemInterruptTime) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = IIndependentFlipFramePresentStatistics_Impl::GetDisplayedTime(this)
            }
        }
        unsafe extern "system" fn GetPresentDuration<Identity: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SystemInterruptTime) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = IIndependentFlipFramePresentStatistics_Impl::GetPresentDuration(this)
            }
        }
        Self {
            base__: IPresentStatistics_Vtbl::new::<Identity, OFFSET>(),
            GetOutputAdapterLUID: GetOutputAdapterLUID::<Identity, OFFSET>,
            GetOutputVidPnSourceId: GetOutputVidPnSourceId::<Identity, OFFSET>,
            GetContentTag: GetContentTag::<Identity, OFFSET>,
            GetDisplayedTime: GetDisplayedTime::<Identity, OFFSET>,
            GetPresentDuration: GetPresentDuration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIndependentFlipFramePresentStatistics as windows_core::Interface>::IID || iid == &<IPresentStatistics as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IIndependentFlipFramePresentStatistics {}
windows_core::imp::define_interface!(IPresentStatistics, IPresentStatistics_Vtbl, 0xb44b8bda_7282_495d_9dd7_ceadd8b4bb86);
windows_core::imp::interface_hierarchy!(IPresentStatistics, windows_core::IUnknown);
impl IPresentStatistics {
    pub unsafe fn GetPresentId(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetPresentId)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        unsafe { (windows_core::Interface::vtable(self).GetKind)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentStatistics_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPresentId: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetKind: unsafe extern "system" fn(*mut core::ffi::c_void) -> PresentStatisticsKind,
}
pub trait IPresentStatistics_Impl: windows_core::IUnknownImpl {
    fn GetPresentId(&self) -> u64;
    fn GetKind(&self) -> PresentStatisticsKind;
}
impl IPresentStatistics_Vtbl {
    pub const fn new<Identity: IPresentStatistics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPresentId<Identity: IPresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentStatistics_Impl::GetPresentId(this)
            }
        }
        unsafe extern "system" fn GetKind<Identity: IPresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> PresentStatisticsKind {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentStatistics_Impl::GetKind(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPresentId: GetPresentId::<Identity, OFFSET>,
            GetKind: GetKind::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPresentStatistics as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPresentStatistics {}
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
        unsafe { (windows_core::Interface::vtable(self).GetCompositionFrameId)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetPresentStatus(&self) -> PresentStatus {
        unsafe { (windows_core::Interface::vtable(self).GetPresentStatus)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentStatusPresentStatistics_Vtbl {
    pub base__: IPresentStatistics_Vtbl,
    pub GetCompositionFrameId: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub GetPresentStatus: unsafe extern "system" fn(*mut core::ffi::c_void) -> PresentStatus,
}
pub trait IPresentStatusPresentStatistics_Impl: IPresentStatistics_Impl {
    fn GetCompositionFrameId(&self) -> u64;
    fn GetPresentStatus(&self) -> PresentStatus;
}
impl IPresentStatusPresentStatistics_Vtbl {
    pub const fn new<Identity: IPresentStatusPresentStatistics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCompositionFrameId<Identity: IPresentStatusPresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentStatusPresentStatistics_Impl::GetCompositionFrameId(this)
            }
        }
        unsafe extern "system" fn GetPresentStatus<Identity: IPresentStatusPresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> PresentStatus {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentStatusPresentStatistics_Impl::GetPresentStatus(this)
            }
        }
        Self {
            base__: IPresentStatistics_Vtbl::new::<Identity, OFFSET>(),
            GetCompositionFrameId: GetCompositionFrameId::<Identity, OFFSET>,
            GetPresentStatus: GetPresentStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPresentStatusPresentStatistics as windows_core::Interface>::IID || iid == &<IPresentStatistics as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPresentStatusPresentStatistics {}
windows_core::imp::define_interface!(IPresentationBuffer, IPresentationBuffer_Vtbl, 0x2e217d3a_5abb_4138_9a13_a775593c89ca);
windows_core::imp::interface_hierarchy!(IPresentationBuffer, windows_core::IUnknown);
impl IPresentationBuffer {
    pub unsafe fn GetAvailableEvent(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAvailableEvent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsAvailable(&self) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAvailableEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub IsAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
pub trait IPresentationBuffer_Impl: windows_core::IUnknownImpl {
    fn GetAvailableEvent(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn IsAvailable(&self) -> windows_core::Result<u8>;
}
impl IPresentationBuffer_Vtbl {
    pub const fn new<Identity: IPresentationBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAvailableEvent<Identity: IPresentationBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, availableeventhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPresentationBuffer_Impl::GetAvailableEvent(this) {
                    Ok(ok__) => {
                        availableeventhandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsAvailable<Identity: IPresentationBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isavailable: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPresentationBuffer_Impl::IsAvailable(this) {
                    Ok(ok__) => {
                        isavailable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAvailableEvent: GetAvailableEvent::<Identity, OFFSET>,
            IsAvailable: IsAvailable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPresentationBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPresentationBuffer {}
windows_core::imp::define_interface!(IPresentationContent, IPresentationContent_Vtbl, 0x5668bb79_3d8e_415c_b215_f38020f2d252);
windows_core::imp::interface_hierarchy!(IPresentationContent, windows_core::IUnknown);
impl IPresentationContent {
    pub unsafe fn SetTag(&self, tag: usize) {
        unsafe { (windows_core::Interface::vtable(self).SetTag)(windows_core::Interface::as_raw(self), tag) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationContent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, usize),
}
pub trait IPresentationContent_Impl: windows_core::IUnknownImpl {
    fn SetTag(&self, tag: usize);
}
impl IPresentationContent_Vtbl {
    pub const fn new<Identity: IPresentationContent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTag<Identity: IPresentationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tag: usize) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationContent_Impl::SetTag(this, core::mem::transmute_copy(&tag))
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetTag: SetTag::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPresentationContent as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPresentationContent {}
windows_core::imp::define_interface!(IPresentationFactory, IPresentationFactory_Vtbl, 0x8fb37b58_1d74_4f64_a49c_1f97a80a2ec0);
windows_core::imp::interface_hierarchy!(IPresentationFactory, windows_core::IUnknown);
impl IPresentationFactory {
    pub unsafe fn IsPresentationSupported(&self) -> u8 {
        unsafe { (windows_core::Interface::vtable(self).IsPresentationSupported)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsPresentationSupportedWithIndependentFlip(&self) -> u8 {
        unsafe { (windows_core::Interface::vtable(self).IsPresentationSupportedWithIndependentFlip)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CreatePresentationManager(&self) -> windows_core::Result<IPresentationManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePresentationManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsPresentationSupported: unsafe extern "system" fn(*mut core::ffi::c_void) -> u8,
    pub IsPresentationSupportedWithIndependentFlip: unsafe extern "system" fn(*mut core::ffi::c_void) -> u8,
    pub CreatePresentationManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPresentationFactory_Impl: windows_core::IUnknownImpl {
    fn IsPresentationSupported(&self) -> u8;
    fn IsPresentationSupportedWithIndependentFlip(&self) -> u8;
    fn CreatePresentationManager(&self) -> windows_core::Result<IPresentationManager>;
}
impl IPresentationFactory_Vtbl {
    pub const fn new<Identity: IPresentationFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsPresentationSupported<Identity: IPresentationFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u8 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationFactory_Impl::IsPresentationSupported(this)
            }
        }
        unsafe extern "system" fn IsPresentationSupportedWithIndependentFlip<Identity: IPresentationFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u8 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationFactory_Impl::IsPresentationSupportedWithIndependentFlip(this)
            }
        }
        unsafe extern "system" fn CreatePresentationManager<Identity: IPresentationFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppresentationmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPresentationFactory_Impl::CreatePresentationManager(this) {
                    Ok(ok__) => {
                        pppresentationmanager.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsPresentationSupported: IsPresentationSupported::<Identity, OFFSET>,
            IsPresentationSupportedWithIndependentFlip: IsPresentationSupportedWithIndependentFlip::<Identity, OFFSET>,
            CreatePresentationManager: CreatePresentationManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPresentationFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPresentationFactory {}
windows_core::imp::define_interface!(IPresentationManager, IPresentationManager_Vtbl, 0xfb562f82_6292_470a_88b1_843661e7f20c);
windows_core::imp::interface_hierarchy!(IPresentationManager, windows_core::IUnknown);
impl IPresentationManager {
    pub unsafe fn AddBufferFromResource<P0>(&self, resource: P0) -> windows_core::Result<IPresentationBuffer>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddBufferFromResource)(windows_core::Interface::as_raw(self), resource.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePresentationSurface(&self, compositionsurfacehandle: super::super::Foundation::HANDLE) -> windows_core::Result<IPresentationSurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePresentationSurface)(windows_core::Interface::as_raw(self), compositionsurfacehandle, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNextPresentId(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNextPresentId)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetTargetTime(&self, targettime: SystemInterruptTime) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTargetTime)(windows_core::Interface::as_raw(self), core::mem::transmute(targettime)).ok() }
    }
    pub unsafe fn SetPreferredPresentDuration(&self, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPreferredPresentDuration)(windows_core::Interface::as_raw(self), core::mem::transmute(preferredduration), core::mem::transmute(deviationtolerance)).ok() }
    }
    pub unsafe fn ForceVSyncInterrupt(&self, forcevsyncinterrupt: u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ForceVSyncInterrupt)(windows_core::Interface::as_raw(self), forcevsyncinterrupt).ok() }
    }
    pub unsafe fn Present(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetPresentRetiringFence<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetPresentRetiringFence)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn CancelPresentsFrom(&self, presentidtocancelfrom: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelPresentsFrom)(windows_core::Interface::as_raw(self), presentidtocancelfrom).ok() }
    }
    pub unsafe fn GetLostEvent(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLostEvent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPresentStatisticsAvailableEvent(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPresentStatisticsAvailableEvent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnablePresentStatisticsKind(&self, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnablePresentStatisticsKind)(windows_core::Interface::as_raw(self), presentstatisticskind, enabled).ok() }
    }
    pub unsafe fn GetNextPresentStatistics(&self) -> windows_core::Result<IPresentStatistics> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextPresentStatistics)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IPresentationManager_Impl: windows_core::IUnknownImpl {
    fn AddBufferFromResource(&self, resource: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<IPresentationBuffer>;
    fn CreatePresentationSurface(&self, compositionsurfacehandle: super::super::Foundation::HANDLE) -> windows_core::Result<IPresentationSurface>;
    fn GetNextPresentId(&self) -> u64;
    fn SetTargetTime(&self, targettime: &SystemInterruptTime) -> windows_core::Result<()>;
    fn SetPreferredPresentDuration(&self, preferredduration: &SystemInterruptTime, deviationtolerance: &SystemInterruptTime) -> windows_core::Result<()>;
    fn ForceVSyncInterrupt(&self, forcevsyncinterrupt: u8) -> windows_core::Result<()>;
    fn Present(&self) -> windows_core::Result<()>;
    fn GetPresentRetiringFence(&self, riid: *const windows_core::GUID, fence: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CancelPresentsFrom(&self, presentidtocancelfrom: u64) -> windows_core::Result<()>;
    fn GetLostEvent(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn GetPresentStatisticsAvailableEvent(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn EnablePresentStatisticsKind(&self, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> windows_core::Result<()>;
    fn GetNextPresentStatistics(&self) -> windows_core::Result<IPresentStatistics>;
}
impl IPresentationManager_Vtbl {
    pub const fn new<Identity: IPresentationManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddBufferFromResource<Identity: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, presentationbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPresentationManager_Impl::AddBufferFromResource(this, core::mem::transmute_copy(&resource)) {
                    Ok(ok__) => {
                        presentationbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePresentationSurface<Identity: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, compositionsurfacehandle: super::super::Foundation::HANDLE, presentationsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPresentationManager_Impl::CreatePresentationSurface(this, core::mem::transmute_copy(&compositionsurfacehandle)) {
                    Ok(ok__) => {
                        presentationsurface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNextPresentId<Identity: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationManager_Impl::GetNextPresentId(this)
            }
        }
        unsafe extern "system" fn SetTargetTime<Identity: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targettime: SystemInterruptTime) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationManager_Impl::SetTargetTime(this, core::mem::transmute(&targettime)).into()
            }
        }
        unsafe extern "system" fn SetPreferredPresentDuration<Identity: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationManager_Impl::SetPreferredPresentDuration(this, core::mem::transmute(&preferredduration), core::mem::transmute(&deviationtolerance)).into()
            }
        }
        unsafe extern "system" fn ForceVSyncInterrupt<Identity: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, forcevsyncinterrupt: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationManager_Impl::ForceVSyncInterrupt(this, core::mem::transmute_copy(&forcevsyncinterrupt)).into()
            }
        }
        unsafe extern "system" fn Present<Identity: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationManager_Impl::Present(this).into()
            }
        }
        unsafe extern "system" fn GetPresentRetiringFence<Identity: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, fence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationManager_Impl::GetPresentRetiringFence(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&fence)).into()
            }
        }
        unsafe extern "system" fn CancelPresentsFrom<Identity: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presentidtocancelfrom: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationManager_Impl::CancelPresentsFrom(this, core::mem::transmute_copy(&presentidtocancelfrom)).into()
            }
        }
        unsafe extern "system" fn GetLostEvent<Identity: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, losteventhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPresentationManager_Impl::GetLostEvent(this) {
                    Ok(ok__) => {
                        losteventhandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPresentStatisticsAvailableEvent<Identity: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presentstatisticsavailableeventhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPresentationManager_Impl::GetPresentStatisticsAvailableEvent(this) {
                    Ok(ok__) => {
                        presentstatisticsavailableeventhandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnablePresentStatisticsKind<Identity: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationManager_Impl::EnablePresentStatisticsKind(this, core::mem::transmute_copy(&presentstatisticskind), core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn GetNextPresentStatistics<Identity: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nextpresentstatistics: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPresentationManager_Impl::GetNextPresentStatistics(this) {
                    Ok(ok__) => {
                        nextpresentstatistics.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddBufferFromResource: AddBufferFromResource::<Identity, OFFSET>,
            CreatePresentationSurface: CreatePresentationSurface::<Identity, OFFSET>,
            GetNextPresentId: GetNextPresentId::<Identity, OFFSET>,
            SetTargetTime: SetTargetTime::<Identity, OFFSET>,
            SetPreferredPresentDuration: SetPreferredPresentDuration::<Identity, OFFSET>,
            ForceVSyncInterrupt: ForceVSyncInterrupt::<Identity, OFFSET>,
            Present: Present::<Identity, OFFSET>,
            GetPresentRetiringFence: GetPresentRetiringFence::<Identity, OFFSET>,
            CancelPresentsFrom: CancelPresentsFrom::<Identity, OFFSET>,
            GetLostEvent: GetLostEvent::<Identity, OFFSET>,
            GetPresentStatisticsAvailableEvent: GetPresentStatisticsAvailableEvent::<Identity, OFFSET>,
            EnablePresentStatisticsKind: EnablePresentStatisticsKind::<Identity, OFFSET>,
            GetNextPresentStatistics: GetNextPresentStatistics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPresentationManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPresentationManager {}
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
        unsafe { (windows_core::Interface::vtable(self).SetBuffer)(windows_core::Interface::as_raw(self), presentationbuffer.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetColorSpace)(windows_core::Interface::as_raw(self), colorspace).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetAlphaMode(&self, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAlphaMode)(windows_core::Interface::as_raw(self), alphamode).ok() }
    }
    pub unsafe fn SetSourceRect(&self, sourcerect: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSourceRect)(windows_core::Interface::as_raw(self), sourcerect).ok() }
    }
    pub unsafe fn SetTransform(&self, transform: *const PresentationTransform) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform).ok() }
    }
    pub unsafe fn RestrictToOutput<P0>(&self, output: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RestrictToOutput)(windows_core::Interface::as_raw(self), output.param().abi()).ok() }
    }
    pub unsafe fn SetDisableReadback(&self, value: u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisableReadback)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub unsafe fn SetLetterboxingMargins(&self, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLetterboxingMargins)(windows_core::Interface::as_raw(self), leftletterboxsize, topletterboxsize, rightletterboxsize, bottomletterboxsize).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IPresentationSurface_Impl: IPresentationContent_Impl {
    fn SetBuffer(&self, presentationbuffer: windows_core::Ref<IPresentationBuffer>) -> windows_core::Result<()>;
    fn SetColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<()>;
    fn SetAlphaMode(&self, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<()>;
    fn SetSourceRect(&self, sourcerect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn SetTransform(&self, transform: *const PresentationTransform) -> windows_core::Result<()>;
    fn RestrictToOutput(&self, output: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetDisableReadback(&self, value: u8) -> windows_core::Result<()>;
    fn SetLetterboxingMargins(&self, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IPresentationSurface_Vtbl {
    pub const fn new<Identity: IPresentationSurface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetBuffer<Identity: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presentationbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationSurface_Impl::SetBuffer(this, core::mem::transmute_copy(&presentationbuffer)).into()
            }
        }
        unsafe extern "system" fn SetColorSpace<Identity: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationSurface_Impl::SetColorSpace(this, core::mem::transmute_copy(&colorspace)).into()
            }
        }
        unsafe extern "system" fn SetAlphaMode<Identity: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationSurface_Impl::SetAlphaMode(this, core::mem::transmute_copy(&alphamode)).into()
            }
        }
        unsafe extern "system" fn SetSourceRect<Identity: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcerect: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationSurface_Impl::SetSourceRect(this, core::mem::transmute_copy(&sourcerect)).into()
            }
        }
        unsafe extern "system" fn SetTransform<Identity: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *const PresentationTransform) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationSurface_Impl::SetTransform(this, core::mem::transmute_copy(&transform)).into()
            }
        }
        unsafe extern "system" fn RestrictToOutput<Identity: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, output: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationSurface_Impl::RestrictToOutput(this, core::mem::transmute_copy(&output)).into()
            }
        }
        unsafe extern "system" fn SetDisableReadback<Identity: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationSurface_Impl::SetDisableReadback(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetLetterboxingMargins<Identity: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPresentationSurface_Impl::SetLetterboxingMargins(this, core::mem::transmute_copy(&leftletterboxsize), core::mem::transmute_copy(&topletterboxsize), core::mem::transmute_copy(&rightletterboxsize), core::mem::transmute_copy(&bottomletterboxsize)).into()
            }
        }
        Self {
            base__: IPresentationContent_Vtbl::new::<Identity, OFFSET>(),
            SetBuffer: SetBuffer::<Identity, OFFSET>,
            SetColorSpace: SetColorSpace::<Identity, OFFSET>,
            SetAlphaMode: SetAlphaMode::<Identity, OFFSET>,
            SetSourceRect: SetSourceRect::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            RestrictToOutput: RestrictToOutput::<Identity, OFFSET>,
            SetDisableReadback: SetDisableReadback::<Identity, OFFSET>,
            SetLetterboxingMargins: SetLetterboxingMargins::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPresentationSurface as windows_core::Interface>::IID || iid == &<IPresentationContent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IPresentationSurface {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PresentStatisticsKind(pub i32);
pub const PresentStatisticsKind_CompositionFrame: PresentStatisticsKind = PresentStatisticsKind(2i32);
pub const PresentStatisticsKind_IndependentFlipFrame: PresentStatisticsKind = PresentStatisticsKind(3i32);
pub const PresentStatisticsKind_PresentStatus: PresentStatisticsKind = PresentStatisticsKind(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PresentStatus(pub i32);
pub const PresentStatus_Canceled: PresentStatus = PresentStatus(2i32);
pub const PresentStatus_Queued: PresentStatus = PresentStatus(0i32);
pub const PresentStatus_Skipped: PresentStatus = PresentStatus(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PresentationTransform {
    pub M11: f32,
    pub M12: f32,
    pub M21: f32,
    pub M22: f32,
    pub M31: f32,
    pub M32: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SystemInterruptTime {
    pub value: u64,
}
