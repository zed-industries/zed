#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ICompositionFramePresentStatistics_Impl: Sized + IPresentStatistics_Impl {
    fn GetContentTag(&self) -> usize;
    fn GetCompositionFrameId(&self) -> u64;
    fn GetDisplayInstanceArray(&self, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ICompositionFramePresentStatistics {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ICompositionFramePresentStatistics_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionFramePresentStatistics_Impl, const OFFSET: isize>() -> ICompositionFramePresentStatistics_Vtbl {
        unsafe extern "system" fn GetContentTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICompositionFramePresentStatistics_Impl::GetContentTag(this)
        }
        unsafe extern "system" fn GetCompositionFrameId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICompositionFramePresentStatistics_Impl::GetCompositionFrameId(this)
        }
        unsafe extern "system" fn GetDisplayInstanceArray<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICompositionFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICompositionFramePresentStatistics_Impl::GetDisplayInstanceArray(this, core::mem::transmute_copy(&displayinstancearraycount), core::mem::transmute_copy(&displayinstancearray))
        }
        Self {
            base__: IPresentStatistics_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetContentTag: GetContentTag::<Identity, Impl, OFFSET>,
            GetCompositionFrameId: GetCompositionFrameId::<Identity, Impl, OFFSET>,
            GetDisplayInstanceArray: GetDisplayInstanceArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionFramePresentStatistics as windows_core::Interface>::IID || iid == &<IPresentStatistics as windows_core::Interface>::IID
    }
}
pub trait IIndependentFlipFramePresentStatistics_Impl: Sized + IPresentStatistics_Impl {
    fn GetOutputAdapterLUID(&self) -> super::super::Foundation::LUID;
    fn GetOutputVidPnSourceId(&self) -> u32;
    fn GetContentTag(&self) -> usize;
    fn GetDisplayedTime(&self) -> SystemInterruptTime;
    fn GetPresentDuration(&self) -> SystemInterruptTime;
}
impl windows_core::RuntimeName for IIndependentFlipFramePresentStatistics {}
impl IIndependentFlipFramePresentStatistics_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>() -> IIndependentFlipFramePresentStatistics_Vtbl {
        unsafe extern "system" fn GetOutputAdapterLUID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = IIndependentFlipFramePresentStatistics_Impl::GetOutputAdapterLUID(this)
        }
        unsafe extern "system" fn GetOutputVidPnSourceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IIndependentFlipFramePresentStatistics_Impl::GetOutputVidPnSourceId(this)
        }
        unsafe extern "system" fn GetContentTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IIndependentFlipFramePresentStatistics_Impl::GetContentTag(this)
        }
        unsafe extern "system" fn GetDisplayedTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SystemInterruptTime) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = IIndependentFlipFramePresentStatistics_Impl::GetDisplayedTime(this)
        }
        unsafe extern "system" fn GetPresentDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SystemInterruptTime) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = IIndependentFlipFramePresentStatistics_Impl::GetPresentDuration(this)
        }
        Self {
            base__: IPresentStatistics_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOutputAdapterLUID: GetOutputAdapterLUID::<Identity, Impl, OFFSET>,
            GetOutputVidPnSourceId: GetOutputVidPnSourceId::<Identity, Impl, OFFSET>,
            GetContentTag: GetContentTag::<Identity, Impl, OFFSET>,
            GetDisplayedTime: GetDisplayedTime::<Identity, Impl, OFFSET>,
            GetPresentDuration: GetPresentDuration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIndependentFlipFramePresentStatistics as windows_core::Interface>::IID || iid == &<IPresentStatistics as windows_core::Interface>::IID
    }
}
pub trait IPresentStatistics_Impl: Sized {
    fn GetPresentId(&self) -> u64;
    fn GetKind(&self) -> PresentStatisticsKind;
}
impl windows_core::RuntimeName for IPresentStatistics {}
impl IPresentStatistics_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentStatistics_Impl, const OFFSET: isize>() -> IPresentStatistics_Vtbl {
        unsafe extern "system" fn GetPresentId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentStatistics_Impl::GetPresentId(this)
        }
        unsafe extern "system" fn GetKind<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> PresentStatisticsKind {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentStatistics_Impl::GetKind(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPresentId: GetPresentId::<Identity, Impl, OFFSET>,
            GetKind: GetKind::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPresentStatistics as windows_core::Interface>::IID
    }
}
pub trait IPresentStatusPresentStatistics_Impl: Sized + IPresentStatistics_Impl {
    fn GetCompositionFrameId(&self) -> u64;
    fn GetPresentStatus(&self) -> PresentStatus;
}
impl windows_core::RuntimeName for IPresentStatusPresentStatistics {}
impl IPresentStatusPresentStatistics_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentStatusPresentStatistics_Impl, const OFFSET: isize>() -> IPresentStatusPresentStatistics_Vtbl {
        unsafe extern "system" fn GetCompositionFrameId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentStatusPresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentStatusPresentStatistics_Impl::GetCompositionFrameId(this)
        }
        unsafe extern "system" fn GetPresentStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentStatusPresentStatistics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> PresentStatus {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentStatusPresentStatistics_Impl::GetPresentStatus(this)
        }
        Self {
            base__: IPresentStatistics_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCompositionFrameId: GetCompositionFrameId::<Identity, Impl, OFFSET>,
            GetPresentStatus: GetPresentStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPresentStatusPresentStatistics as windows_core::Interface>::IID || iid == &<IPresentStatistics as windows_core::Interface>::IID
    }
}
pub trait IPresentationBuffer_Impl: Sized {
    fn GetAvailableEvent(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn IsAvailable(&self) -> windows_core::Result<u8>;
}
impl windows_core::RuntimeName for IPresentationBuffer {}
impl IPresentationBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationBuffer_Impl, const OFFSET: isize>() -> IPresentationBuffer_Vtbl {
        unsafe extern "system" fn GetAvailableEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, availableeventhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPresentationBuffer_Impl::GetAvailableEvent(this) {
                Ok(ok__) => {
                    core::ptr::write(availableeventhandle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isavailable: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPresentationBuffer_Impl::IsAvailable(this) {
                Ok(ok__) => {
                    core::ptr::write(isavailable, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAvailableEvent: GetAvailableEvent::<Identity, Impl, OFFSET>,
            IsAvailable: IsAvailable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPresentationBuffer as windows_core::Interface>::IID
    }
}
pub trait IPresentationContent_Impl: Sized {
    fn SetTag(&self, tag: usize);
}
impl windows_core::RuntimeName for IPresentationContent {}
impl IPresentationContent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationContent_Impl, const OFFSET: isize>() -> IPresentationContent_Vtbl {
        unsafe extern "system" fn SetTag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tag: usize) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationContent_Impl::SetTag(this, core::mem::transmute_copy(&tag))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetTag: SetTag::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPresentationContent as windows_core::Interface>::IID
    }
}
pub trait IPresentationFactory_Impl: Sized {
    fn IsPresentationSupported(&self) -> u8;
    fn IsPresentationSupportedWithIndependentFlip(&self) -> u8;
    fn CreatePresentationManager(&self) -> windows_core::Result<IPresentationManager>;
}
impl windows_core::RuntimeName for IPresentationFactory {}
impl IPresentationFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationFactory_Impl, const OFFSET: isize>() -> IPresentationFactory_Vtbl {
        unsafe extern "system" fn IsPresentationSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u8 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationFactory_Impl::IsPresentationSupported(this)
        }
        unsafe extern "system" fn IsPresentationSupportedWithIndependentFlip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u8 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationFactory_Impl::IsPresentationSupportedWithIndependentFlip(this)
        }
        unsafe extern "system" fn CreatePresentationManager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppresentationmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPresentationFactory_Impl::CreatePresentationManager(this) {
                Ok(ok__) => {
                    core::ptr::write(pppresentationmanager, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsPresentationSupported: IsPresentationSupported::<Identity, Impl, OFFSET>,
            IsPresentationSupportedWithIndependentFlip: IsPresentationSupportedWithIndependentFlip::<Identity, Impl, OFFSET>,
            CreatePresentationManager: CreatePresentationManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPresentationFactory as windows_core::Interface>::IID
    }
}
pub trait IPresentationManager_Impl: Sized {
    fn AddBufferFromResource(&self, resource: Option<&windows_core::IUnknown>) -> windows_core::Result<IPresentationBuffer>;
    fn CreatePresentationSurface(&self, compositionsurfacehandle: super::super::Foundation::HANDLE) -> windows_core::Result<IPresentationSurface>;
    fn GetNextPresentId(&self) -> u64;
    fn SetTargetTime(&self, targettime: &SystemInterruptTime) -> windows_core::Result<()>;
    fn SetPreferredPresentDuration(&self, preferredduration: &SystemInterruptTime, deviationtolerance: &SystemInterruptTime) -> windows_core::Result<()>;
    fn ForceVSyncInterrupt(&self, forcevsyncinterrupt: u8) -> windows_core::Result<()>;
    fn Present(&self) -> windows_core::Result<()>;
    fn GetPresentRetiringFence(&self, riid: *const windows_core::GUID) -> windows_core::Result<*mut core::ffi::c_void>;
    fn CancelPresentsFrom(&self, presentidtocancelfrom: u64) -> windows_core::Result<()>;
    fn GetLostEvent(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn GetPresentStatisticsAvailableEvent(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn EnablePresentStatisticsKind(&self, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> windows_core::Result<()>;
    fn GetNextPresentStatistics(&self) -> windows_core::Result<IPresentStatistics>;
}
impl windows_core::RuntimeName for IPresentationManager {}
impl IPresentationManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>() -> IPresentationManager_Vtbl {
        unsafe extern "system" fn AddBufferFromResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, presentationbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPresentationManager_Impl::AddBufferFromResource(this, windows_core::from_raw_borrowed(&resource)) {
                Ok(ok__) => {
                    core::ptr::write(presentationbuffer, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePresentationSurface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, compositionsurfacehandle: super::super::Foundation::HANDLE, presentationsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPresentationManager_Impl::CreatePresentationSurface(this, core::mem::transmute_copy(&compositionsurfacehandle)) {
                Ok(ok__) => {
                    core::ptr::write(presentationsurface, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextPresentId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationManager_Impl::GetNextPresentId(this)
        }
        unsafe extern "system" fn SetTargetTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targettime: SystemInterruptTime) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationManager_Impl::SetTargetTime(this, core::mem::transmute(&targettime)).into()
        }
        unsafe extern "system" fn SetPreferredPresentDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationManager_Impl::SetPreferredPresentDuration(this, core::mem::transmute(&preferredduration), core::mem::transmute(&deviationtolerance)).into()
        }
        unsafe extern "system" fn ForceVSyncInterrupt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, forcevsyncinterrupt: u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationManager_Impl::ForceVSyncInterrupt(this, core::mem::transmute_copy(&forcevsyncinterrupt)).into()
        }
        unsafe extern "system" fn Present<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationManager_Impl::Present(this).into()
        }
        unsafe extern "system" fn GetPresentRetiringFence<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, fence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPresentationManager_Impl::GetPresentRetiringFence(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    core::ptr::write(fence, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelPresentsFrom<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presentidtocancelfrom: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationManager_Impl::CancelPresentsFrom(this, core::mem::transmute_copy(&presentidtocancelfrom)).into()
        }
        unsafe extern "system" fn GetLostEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, losteventhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPresentationManager_Impl::GetLostEvent(this) {
                Ok(ok__) => {
                    core::ptr::write(losteventhandle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresentStatisticsAvailableEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presentstatisticsavailableeventhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPresentationManager_Impl::GetPresentStatisticsAvailableEvent(this) {
                Ok(ok__) => {
                    core::ptr::write(presentstatisticsavailableeventhandle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnablePresentStatisticsKind<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationManager_Impl::EnablePresentStatisticsKind(this, core::mem::transmute_copy(&presentstatisticskind), core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn GetNextPresentStatistics<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nextpresentstatistics: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPresentationManager_Impl::GetNextPresentStatistics(this) {
                Ok(ok__) => {
                    core::ptr::write(nextpresentstatistics, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddBufferFromResource: AddBufferFromResource::<Identity, Impl, OFFSET>,
            CreatePresentationSurface: CreatePresentationSurface::<Identity, Impl, OFFSET>,
            GetNextPresentId: GetNextPresentId::<Identity, Impl, OFFSET>,
            SetTargetTime: SetTargetTime::<Identity, Impl, OFFSET>,
            SetPreferredPresentDuration: SetPreferredPresentDuration::<Identity, Impl, OFFSET>,
            ForceVSyncInterrupt: ForceVSyncInterrupt::<Identity, Impl, OFFSET>,
            Present: Present::<Identity, Impl, OFFSET>,
            GetPresentRetiringFence: GetPresentRetiringFence::<Identity, Impl, OFFSET>,
            CancelPresentsFrom: CancelPresentsFrom::<Identity, Impl, OFFSET>,
            GetLostEvent: GetLostEvent::<Identity, Impl, OFFSET>,
            GetPresentStatisticsAvailableEvent: GetPresentStatisticsAvailableEvent::<Identity, Impl, OFFSET>,
            EnablePresentStatisticsKind: EnablePresentStatisticsKind::<Identity, Impl, OFFSET>,
            GetNextPresentStatistics: GetNextPresentStatistics::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPresentationManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IPresentationSurface_Impl: Sized + IPresentationContent_Impl {
    fn SetBuffer(&self, presentationbuffer: Option<&IPresentationBuffer>) -> windows_core::Result<()>;
    fn SetColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<()>;
    fn SetAlphaMode(&self, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::Result<()>;
    fn SetSourceRect(&self, sourcerect: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn SetTransform(&self, transform: *const PresentationTransform) -> windows_core::Result<()>;
    fn RestrictToOutput(&self, output: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetDisableReadback(&self, value: u8) -> windows_core::Result<()>;
    fn SetLetterboxingMargins(&self, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IPresentationSurface {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IPresentationSurface_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>() -> IPresentationSurface_Vtbl {
        unsafe extern "system" fn SetBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presentationbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationSurface_Impl::SetBuffer(this, windows_core::from_raw_borrowed(&presentationbuffer)).into()
        }
        unsafe extern "system" fn SetColorSpace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationSurface_Impl::SetColorSpace(this, core::mem::transmute_copy(&colorspace)).into()
        }
        unsafe extern "system" fn SetAlphaMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationSurface_Impl::SetAlphaMode(this, core::mem::transmute_copy(&alphamode)).into()
        }
        unsafe extern "system" fn SetSourceRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcerect: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationSurface_Impl::SetSourceRect(this, core::mem::transmute_copy(&sourcerect)).into()
        }
        unsafe extern "system" fn SetTransform<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *const PresentationTransform) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationSurface_Impl::SetTransform(this, core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn RestrictToOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, output: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationSurface_Impl::RestrictToOutput(this, windows_core::from_raw_borrowed(&output)).into()
        }
        unsafe extern "system" fn SetDisableReadback<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationSurface_Impl::SetDisableReadback(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetLetterboxingMargins<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPresentationSurface_Impl::SetLetterboxingMargins(this, core::mem::transmute_copy(&leftletterboxsize), core::mem::transmute_copy(&topletterboxsize), core::mem::transmute_copy(&rightletterboxsize), core::mem::transmute_copy(&bottomletterboxsize)).into()
        }
        Self {
            base__: IPresentationContent_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetBuffer: SetBuffer::<Identity, Impl, OFFSET>,
            SetColorSpace: SetColorSpace::<Identity, Impl, OFFSET>,
            SetAlphaMode: SetAlphaMode::<Identity, Impl, OFFSET>,
            SetSourceRect: SetSourceRect::<Identity, Impl, OFFSET>,
            SetTransform: SetTransform::<Identity, Impl, OFFSET>,
            RestrictToOutput: RestrictToOutput::<Identity, Impl, OFFSET>,
            SetDisableReadback: SetDisableReadback::<Identity, Impl, OFFSET>,
            SetLetterboxingMargins: SetLetterboxingMargins::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPresentationSurface as windows_core::Interface>::IID || iid == &<IPresentationContent as windows_core::Interface>::IID
    }
}
