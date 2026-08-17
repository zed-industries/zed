pub trait ID3D12CommandAllocator_Impl: Sized + ID3D12Pageable_Impl {
    fn Reset(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D12CommandAllocator {}
impl ID3D12CommandAllocator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandAllocator_Impl, const OFFSET: isize>() -> ID3D12CommandAllocator_Vtbl {
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12CommandAllocator_Impl::Reset(this).into()
        }
        Self { base__: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(), Reset: Reset::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12CommandAllocator as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Pageable as windows_core::Interface>::IID
    }
}
pub trait ID3D12CommandList_Impl: Sized + ID3D12DeviceChild_Impl {
    fn GetType(&self) -> D3D12_COMMAND_LIST_TYPE;
}
impl windows_core::RuntimeName for ID3D12CommandList {}
impl ID3D12CommandList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandList_Impl, const OFFSET: isize>() -> ID3D12CommandList_Vtbl {
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D12_COMMAND_LIST_TYPE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12CommandList_Impl::GetType(this)
        }
        Self { base__: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetType: GetType::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D12CommandQueue_Impl: Sized + ID3D12Pageable_Impl {
    fn UpdateTileMappings(&self, presource: Option<&ID3D12Resource>, numresourceregions: u32, presourceregionstartcoordinates: *const D3D12_TILED_RESOURCE_COORDINATE, presourceregionsizes: *const D3D12_TILE_REGION_SIZE, pheap: Option<&ID3D12Heap>, numranges: u32, prangeflags: *const D3D12_TILE_RANGE_FLAGS, pheaprangestartoffsets: *const u32, prangetilecounts: *const u32, flags: D3D12_TILE_MAPPING_FLAGS);
    fn CopyTileMappings(&self, pdstresource: Option<&ID3D12Resource>, pdstregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, psrcresource: Option<&ID3D12Resource>, psrcregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, pregionsize: *const D3D12_TILE_REGION_SIZE, flags: D3D12_TILE_MAPPING_FLAGS);
    fn ExecuteCommandLists(&self, numcommandlists: u32, ppcommandlists: *const Option<ID3D12CommandList>);
    fn SetMarker(&self, metadata: u32, pdata: *const core::ffi::c_void, size: u32);
    fn BeginEvent(&self, metadata: u32, pdata: *const core::ffi::c_void, size: u32);
    fn EndEvent(&self);
    fn Signal(&self, pfence: Option<&ID3D12Fence>, value: u64) -> windows_core::Result<()>;
    fn Wait(&self, pfence: Option<&ID3D12Fence>, value: u64) -> windows_core::Result<()>;
    fn GetTimestampFrequency(&self) -> windows_core::Result<u64>;
    fn GetClockCalibration(&self, pgputimestamp: *mut u64, pcputimestamp: *mut u64) -> windows_core::Result<()>;
    fn GetDesc(&self) -> D3D12_COMMAND_QUEUE_DESC;
}
impl windows_core::RuntimeName for ID3D12CommandQueue {}
impl ID3D12CommandQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>() -> ID3D12CommandQueue_Vtbl {
        unsafe extern "system" fn UpdateTileMappings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, numresourceregions: u32, presourceregionstartcoordinates: *const D3D12_TILED_RESOURCE_COORDINATE, presourceregionsizes: *const D3D12_TILE_REGION_SIZE, pheap: *mut core::ffi::c_void, numranges: u32, prangeflags: *const D3D12_TILE_RANGE_FLAGS, pheaprangestartoffsets: *const u32, prangetilecounts: *const u32, flags: D3D12_TILE_MAPPING_FLAGS) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12CommandQueue_Impl::UpdateTileMappings(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&numresourceregions), core::mem::transmute_copy(&presourceregionstartcoordinates), core::mem::transmute_copy(&presourceregionsizes), windows_core::from_raw_borrowed(&pheap), core::mem::transmute_copy(&numranges), core::mem::transmute_copy(&prangeflags), core::mem::transmute_copy(&pheaprangestartoffsets), core::mem::transmute_copy(&prangetilecounts), core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn CopyTileMappings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, pdstregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, psrcresource: *mut core::ffi::c_void, psrcregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, pregionsize: *const D3D12_TILE_REGION_SIZE, flags: D3D12_TILE_MAPPING_FLAGS) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12CommandQueue_Impl::CopyTileMappings(this, windows_core::from_raw_borrowed(&pdstresource), core::mem::transmute_copy(&pdstregionstartcoordinate), windows_core::from_raw_borrowed(&psrcresource), core::mem::transmute_copy(&psrcregionstartcoordinate), core::mem::transmute_copy(&pregionsize), core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn ExecuteCommandLists<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numcommandlists: u32, ppcommandlists: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12CommandQueue_Impl::ExecuteCommandLists(this, core::mem::transmute_copy(&numcommandlists), core::mem::transmute_copy(&ppcommandlists))
        }
        unsafe extern "system" fn SetMarker<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadata: u32, pdata: *const core::ffi::c_void, size: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12CommandQueue_Impl::SetMarker(this, core::mem::transmute_copy(&metadata), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size))
        }
        unsafe extern "system" fn BeginEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadata: u32, pdata: *const core::ffi::c_void, size: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12CommandQueue_Impl::BeginEvent(this, core::mem::transmute_copy(&metadata), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size))
        }
        unsafe extern "system" fn EndEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12CommandQueue_Impl::EndEvent(this)
        }
        unsafe extern "system" fn Signal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfence: *mut core::ffi::c_void, value: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12CommandQueue_Impl::Signal(this, windows_core::from_raw_borrowed(&pfence), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Wait<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfence: *mut core::ffi::c_void, value: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12CommandQueue_Impl::Wait(this, windows_core::from_raw_borrowed(&pfence), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTimestampFrequency<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrequency: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D12CommandQueue_Impl::GetTimestampFrequency(this) {
                Ok(ok__) => {
                    core::ptr::write(pfrequency, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClockCalibration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgputimestamp: *mut u64, pcputimestamp: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12CommandQueue_Impl::GetClockCalibration(this, core::mem::transmute_copy(&pgputimestamp), core::mem::transmute_copy(&pcputimestamp)).into()
        }
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_COMMAND_QUEUE_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12CommandQueue_Impl::GetDesc(this)
        }
        Self {
            base__: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(),
            UpdateTileMappings: UpdateTileMappings::<Identity, Impl, OFFSET>,
            CopyTileMappings: CopyTileMappings::<Identity, Impl, OFFSET>,
            ExecuteCommandLists: ExecuteCommandLists::<Identity, Impl, OFFSET>,
            SetMarker: SetMarker::<Identity, Impl, OFFSET>,
            BeginEvent: BeginEvent::<Identity, Impl, OFFSET>,
            EndEvent: EndEvent::<Identity, Impl, OFFSET>,
            Signal: Signal::<Identity, Impl, OFFSET>,
            Wait: Wait::<Identity, Impl, OFFSET>,
            GetTimestampFrequency: GetTimestampFrequency::<Identity, Impl, OFFSET>,
            GetClockCalibration: GetClockCalibration::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12CommandQueue as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Pageable as windows_core::Interface>::IID
    }
}
pub trait ID3D12CommandSignature_Impl: Sized + ID3D12Pageable_Impl {}
impl windows_core::RuntimeName for ID3D12CommandSignature {}
impl ID3D12CommandSignature_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12CommandSignature_Impl, const OFFSET: isize>() -> ID3D12CommandSignature_Vtbl {
        Self { base__: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12CommandSignature as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Pageable as windows_core::Interface>::IID
    }
}
pub trait ID3D12Debug_Impl: Sized {
    fn EnableDebugLayer(&self);
}
impl windows_core::RuntimeName for ID3D12Debug {}
impl ID3D12Debug_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug_Impl, const OFFSET: isize>() -> ID3D12Debug_Vtbl {
        unsafe extern "system" fn EnableDebugLayer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Debug_Impl::EnableDebugLayer(this)
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnableDebugLayer: EnableDebugLayer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Debug as windows_core::Interface>::IID
    }
}
pub trait ID3D12Debug1_Impl: Sized {
    fn EnableDebugLayer(&self);
    fn SetEnableGPUBasedValidation(&self, enable: super::super::Foundation::BOOL);
    fn SetEnableSynchronizedCommandQueueValidation(&self, enable: super::super::Foundation::BOOL);
}
impl windows_core::RuntimeName for ID3D12Debug1 {}
impl ID3D12Debug1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug1_Impl, const OFFSET: isize>() -> ID3D12Debug1_Vtbl {
        unsafe extern "system" fn EnableDebugLayer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Debug1_Impl::EnableDebugLayer(this)
        }
        unsafe extern "system" fn SetEnableGPUBasedValidation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Debug1_Impl::SetEnableGPUBasedValidation(this, core::mem::transmute_copy(&enable))
        }
        unsafe extern "system" fn SetEnableSynchronizedCommandQueueValidation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Debug1_Impl::SetEnableSynchronizedCommandQueueValidation(this, core::mem::transmute_copy(&enable))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableDebugLayer: EnableDebugLayer::<Identity, Impl, OFFSET>,
            SetEnableGPUBasedValidation: SetEnableGPUBasedValidation::<Identity, Impl, OFFSET>,
            SetEnableSynchronizedCommandQueueValidation: SetEnableSynchronizedCommandQueueValidation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Debug1 as windows_core::Interface>::IID
    }
}
pub trait ID3D12Debug2_Impl: Sized {
    fn SetGPUBasedValidationFlags(&self, flags: D3D12_GPU_BASED_VALIDATION_FLAGS);
}
impl windows_core::RuntimeName for ID3D12Debug2 {}
impl ID3D12Debug2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug2_Impl, const OFFSET: isize>() -> ID3D12Debug2_Vtbl {
        unsafe extern "system" fn SetGPUBasedValidationFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Debug2_Impl::SetGPUBasedValidationFlags(this, core::mem::transmute_copy(&flags))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetGPUBasedValidationFlags: SetGPUBasedValidationFlags::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Debug2 as windows_core::Interface>::IID
    }
}
pub trait ID3D12Debug3_Impl: Sized + ID3D12Debug_Impl {
    fn SetEnableGPUBasedValidation(&self, enable: super::super::Foundation::BOOL);
    fn SetEnableSynchronizedCommandQueueValidation(&self, enable: super::super::Foundation::BOOL);
    fn SetGPUBasedValidationFlags(&self, flags: D3D12_GPU_BASED_VALIDATION_FLAGS);
}
impl windows_core::RuntimeName for ID3D12Debug3 {}
impl ID3D12Debug3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug3_Impl, const OFFSET: isize>() -> ID3D12Debug3_Vtbl {
        unsafe extern "system" fn SetEnableGPUBasedValidation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Debug3_Impl::SetEnableGPUBasedValidation(this, core::mem::transmute_copy(&enable))
        }
        unsafe extern "system" fn SetEnableSynchronizedCommandQueueValidation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Debug3_Impl::SetEnableSynchronizedCommandQueueValidation(this, core::mem::transmute_copy(&enable))
        }
        unsafe extern "system" fn SetGPUBasedValidationFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Debug3_Impl::SetGPUBasedValidationFlags(this, core::mem::transmute_copy(&flags))
        }
        Self {
            base__: ID3D12Debug_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetEnableGPUBasedValidation: SetEnableGPUBasedValidation::<Identity, Impl, OFFSET>,
            SetEnableSynchronizedCommandQueueValidation: SetEnableSynchronizedCommandQueueValidation::<Identity, Impl, OFFSET>,
            SetGPUBasedValidationFlags: SetGPUBasedValidationFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Debug3 as windows_core::Interface>::IID || iid == &<ID3D12Debug as windows_core::Interface>::IID
    }
}
pub trait ID3D12Debug4_Impl: Sized + ID3D12Debug3_Impl {
    fn DisableDebugLayer(&self);
}
impl windows_core::RuntimeName for ID3D12Debug4 {}
impl ID3D12Debug4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug4_Impl, const OFFSET: isize>() -> ID3D12Debug4_Vtbl {
        unsafe extern "system" fn DisableDebugLayer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Debug4_Impl::DisableDebugLayer(this)
        }
        Self { base__: ID3D12Debug3_Vtbl::new::<Identity, Impl, OFFSET>(), DisableDebugLayer: DisableDebugLayer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Debug4 as windows_core::Interface>::IID || iid == &<ID3D12Debug as windows_core::Interface>::IID || iid == &<ID3D12Debug3 as windows_core::Interface>::IID
    }
}
pub trait ID3D12Debug5_Impl: Sized + ID3D12Debug4_Impl {
    fn SetEnableAutoName(&self, enable: super::super::Foundation::BOOL);
}
impl windows_core::RuntimeName for ID3D12Debug5 {}
impl ID3D12Debug5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug5_Impl, const OFFSET: isize>() -> ID3D12Debug5_Vtbl {
        unsafe extern "system" fn SetEnableAutoName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Debug5_Impl::SetEnableAutoName(this, core::mem::transmute_copy(&enable))
        }
        Self { base__: ID3D12Debug4_Vtbl::new::<Identity, Impl, OFFSET>(), SetEnableAutoName: SetEnableAutoName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Debug5 as windows_core::Interface>::IID || iid == &<ID3D12Debug as windows_core::Interface>::IID || iid == &<ID3D12Debug3 as windows_core::Interface>::IID || iid == &<ID3D12Debug4 as windows_core::Interface>::IID
    }
}
pub trait ID3D12Debug6_Impl: Sized + ID3D12Debug5_Impl {
    fn SetForceLegacyBarrierValidation(&self, enable: super::super::Foundation::BOOL);
}
impl windows_core::RuntimeName for ID3D12Debug6 {}
impl ID3D12Debug6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug6_Impl, const OFFSET: isize>() -> ID3D12Debug6_Vtbl {
        unsafe extern "system" fn SetForceLegacyBarrierValidation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Debug6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Debug6_Impl::SetForceLegacyBarrierValidation(this, core::mem::transmute_copy(&enable))
        }
        Self {
            base__: ID3D12Debug5_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetForceLegacyBarrierValidation: SetForceLegacyBarrierValidation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Debug6 as windows_core::Interface>::IID || iid == &<ID3D12Debug as windows_core::Interface>::IID || iid == &<ID3D12Debug3 as windows_core::Interface>::IID || iid == &<ID3D12Debug4 as windows_core::Interface>::IID || iid == &<ID3D12Debug5 as windows_core::Interface>::IID
    }
}
pub trait ID3D12DebugCommandList_Impl: Sized {
    fn AssertResourceState(&self, presource: Option<&ID3D12Resource>, subresource: u32, state: u32) -> super::super::Foundation::BOOL;
    fn SetFeatureMask(&self, mask: D3D12_DEBUG_FEATURE) -> windows_core::Result<()>;
    fn GetFeatureMask(&self) -> D3D12_DEBUG_FEATURE;
}
impl windows_core::RuntimeName for ID3D12DebugCommandList {}
impl ID3D12DebugCommandList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandList_Impl, const OFFSET: isize>() -> ID3D12DebugCommandList_Vtbl {
        unsafe extern "system" fn AssertResourceState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, subresource: u32, state: u32) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugCommandList_Impl::AssertResourceState(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&state))
        }
        unsafe extern "system" fn SetFeatureMask<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mask: D3D12_DEBUG_FEATURE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugCommandList_Impl::SetFeatureMask(this, core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn GetFeatureMask<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D12_DEBUG_FEATURE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugCommandList_Impl::GetFeatureMask(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AssertResourceState: AssertResourceState::<Identity, Impl, OFFSET>,
            SetFeatureMask: SetFeatureMask::<Identity, Impl, OFFSET>,
            GetFeatureMask: GetFeatureMask::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DebugCommandList as windows_core::Interface>::IID
    }
}
pub trait ID3D12DebugCommandList1_Impl: Sized {
    fn AssertResourceState(&self, presource: Option<&ID3D12Resource>, subresource: u32, state: u32) -> super::super::Foundation::BOOL;
    fn SetDebugParameter(&self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const core::ffi::c_void, datasize: u32) -> windows_core::Result<()>;
    fn GetDebugParameter(&self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut core::ffi::c_void, datasize: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D12DebugCommandList1 {}
impl ID3D12DebugCommandList1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandList1_Impl, const OFFSET: isize>() -> ID3D12DebugCommandList1_Vtbl {
        unsafe extern "system" fn AssertResourceState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, subresource: u32, state: u32) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugCommandList1_Impl::AssertResourceState(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&state))
        }
        unsafe extern "system" fn SetDebugParameter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const core::ffi::c_void, datasize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugCommandList1_Impl::SetDebugParameter(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn GetDebugParameter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut core::ffi::c_void, datasize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugCommandList1_Impl::GetDebugParameter(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&datasize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AssertResourceState: AssertResourceState::<Identity, Impl, OFFSET>,
            SetDebugParameter: SetDebugParameter::<Identity, Impl, OFFSET>,
            GetDebugParameter: GetDebugParameter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DebugCommandList1 as windows_core::Interface>::IID
    }
}
pub trait ID3D12DebugCommandList2_Impl: Sized + ID3D12DebugCommandList_Impl {
    fn SetDebugParameter(&self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const core::ffi::c_void, datasize: u32) -> windows_core::Result<()>;
    fn GetDebugParameter(&self, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut core::ffi::c_void, datasize: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D12DebugCommandList2 {}
impl ID3D12DebugCommandList2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandList2_Impl, const OFFSET: isize>() -> ID3D12DebugCommandList2_Vtbl {
        unsafe extern "system" fn SetDebugParameter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *const core::ffi::c_void, datasize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugCommandList2_Impl::SetDebugParameter(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn GetDebugParameter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE, pdata: *mut core::ffi::c_void, datasize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugCommandList2_Impl::GetDebugParameter(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&datasize)).into()
        }
        Self {
            base__: ID3D12DebugCommandList_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetDebugParameter: SetDebugParameter::<Identity, Impl, OFFSET>,
            GetDebugParameter: GetDebugParameter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DebugCommandList2 as windows_core::Interface>::IID || iid == &<ID3D12DebugCommandList as windows_core::Interface>::IID
    }
}
pub trait ID3D12DebugCommandList3_Impl: Sized + ID3D12DebugCommandList2_Impl {
    fn AssertResourceAccess(&self, presource: Option<&ID3D12Resource>, subresource: u32, access: D3D12_BARRIER_ACCESS);
    fn AssertTextureLayout(&self, presource: Option<&ID3D12Resource>, subresource: u32, layout: D3D12_BARRIER_LAYOUT);
}
impl windows_core::RuntimeName for ID3D12DebugCommandList3 {}
impl ID3D12DebugCommandList3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandList3_Impl, const OFFSET: isize>() -> ID3D12DebugCommandList3_Vtbl {
        unsafe extern "system" fn AssertResourceAccess<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandList3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, subresource: u32, access: D3D12_BARRIER_ACCESS) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugCommandList3_Impl::AssertResourceAccess(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&access))
        }
        unsafe extern "system" fn AssertTextureLayout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandList3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, subresource: u32, layout: D3D12_BARRIER_LAYOUT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugCommandList3_Impl::AssertTextureLayout(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&layout))
        }
        Self {
            base__: ID3D12DebugCommandList2_Vtbl::new::<Identity, Impl, OFFSET>(),
            AssertResourceAccess: AssertResourceAccess::<Identity, Impl, OFFSET>,
            AssertTextureLayout: AssertTextureLayout::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DebugCommandList3 as windows_core::Interface>::IID || iid == &<ID3D12DebugCommandList as windows_core::Interface>::IID || iid == &<ID3D12DebugCommandList2 as windows_core::Interface>::IID
    }
}
pub trait ID3D12DebugCommandQueue_Impl: Sized {
    fn AssertResourceState(&self, presource: Option<&ID3D12Resource>, subresource: u32, state: u32) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for ID3D12DebugCommandQueue {}
impl ID3D12DebugCommandQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandQueue_Impl, const OFFSET: isize>() -> ID3D12DebugCommandQueue_Vtbl {
        unsafe extern "system" fn AssertResourceState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, subresource: u32, state: u32) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugCommandQueue_Impl::AssertResourceState(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&state))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AssertResourceState: AssertResourceState::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DebugCommandQueue as windows_core::Interface>::IID
    }
}
pub trait ID3D12DebugCommandQueue1_Impl: Sized + ID3D12DebugCommandQueue_Impl {
    fn AssertResourceAccess(&self, presource: Option<&ID3D12Resource>, subresource: u32, access: D3D12_BARRIER_ACCESS);
    fn AssertTextureLayout(&self, presource: Option<&ID3D12Resource>, subresource: u32, layout: D3D12_BARRIER_LAYOUT);
}
impl windows_core::RuntimeName for ID3D12DebugCommandQueue1 {}
impl ID3D12DebugCommandQueue1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandQueue1_Impl, const OFFSET: isize>() -> ID3D12DebugCommandQueue1_Vtbl {
        unsafe extern "system" fn AssertResourceAccess<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandQueue1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, subresource: u32, access: D3D12_BARRIER_ACCESS) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugCommandQueue1_Impl::AssertResourceAccess(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&access))
        }
        unsafe extern "system" fn AssertTextureLayout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugCommandQueue1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, subresource: u32, layout: D3D12_BARRIER_LAYOUT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugCommandQueue1_Impl::AssertTextureLayout(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&layout))
        }
        Self {
            base__: ID3D12DebugCommandQueue_Vtbl::new::<Identity, Impl, OFFSET>(),
            AssertResourceAccess: AssertResourceAccess::<Identity, Impl, OFFSET>,
            AssertTextureLayout: AssertTextureLayout::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DebugCommandQueue1 as windows_core::Interface>::IID || iid == &<ID3D12DebugCommandQueue as windows_core::Interface>::IID
    }
}
pub trait ID3D12DebugDevice_Impl: Sized {
    fn SetFeatureMask(&self, mask: D3D12_DEBUG_FEATURE) -> windows_core::Result<()>;
    fn GetFeatureMask(&self) -> D3D12_DEBUG_FEATURE;
    fn ReportLiveDeviceObjects(&self, flags: D3D12_RLDO_FLAGS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D12DebugDevice {}
impl ID3D12DebugDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugDevice_Impl, const OFFSET: isize>() -> ID3D12DebugDevice_Vtbl {
        unsafe extern "system" fn SetFeatureMask<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mask: D3D12_DEBUG_FEATURE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugDevice_Impl::SetFeatureMask(this, core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn GetFeatureMask<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D12_DEBUG_FEATURE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugDevice_Impl::GetFeatureMask(this)
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: D3D12_RLDO_FLAGS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugDevice_Impl::ReportLiveDeviceObjects(this, core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFeatureMask: SetFeatureMask::<Identity, Impl, OFFSET>,
            GetFeatureMask: GetFeatureMask::<Identity, Impl, OFFSET>,
            ReportLiveDeviceObjects: ReportLiveDeviceObjects::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DebugDevice as windows_core::Interface>::IID
    }
}
pub trait ID3D12DebugDevice1_Impl: Sized {
    fn SetDebugParameter(&self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const core::ffi::c_void, datasize: u32) -> windows_core::Result<()>;
    fn GetDebugParameter(&self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut core::ffi::c_void, datasize: u32) -> windows_core::Result<()>;
    fn ReportLiveDeviceObjects(&self, flags: D3D12_RLDO_FLAGS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D12DebugDevice1 {}
impl ID3D12DebugDevice1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugDevice1_Impl, const OFFSET: isize>() -> ID3D12DebugDevice1_Vtbl {
        unsafe extern "system" fn SetDebugParameter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const core::ffi::c_void, datasize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugDevice1_Impl::SetDebugParameter(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn GetDebugParameter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut core::ffi::c_void, datasize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugDevice1_Impl::GetDebugParameter(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: D3D12_RLDO_FLAGS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugDevice1_Impl::ReportLiveDeviceObjects(this, core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDebugParameter: SetDebugParameter::<Identity, Impl, OFFSET>,
            GetDebugParameter: GetDebugParameter::<Identity, Impl, OFFSET>,
            ReportLiveDeviceObjects: ReportLiveDeviceObjects::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DebugDevice1 as windows_core::Interface>::IID
    }
}
pub trait ID3D12DebugDevice2_Impl: Sized + ID3D12DebugDevice_Impl {
    fn SetDebugParameter(&self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const core::ffi::c_void, datasize: u32) -> windows_core::Result<()>;
    fn GetDebugParameter(&self, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut core::ffi::c_void, datasize: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D12DebugDevice2 {}
impl ID3D12DebugDevice2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugDevice2_Impl, const OFFSET: isize>() -> ID3D12DebugDevice2_Vtbl {
        unsafe extern "system" fn SetDebugParameter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *const core::ffi::c_void, datasize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugDevice2_Impl::SetDebugParameter(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&datasize)).into()
        }
        unsafe extern "system" fn GetDebugParameter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DebugDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, pdata: *mut core::ffi::c_void, datasize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DebugDevice2_Impl::GetDebugParameter(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&datasize)).into()
        }
        Self {
            base__: ID3D12DebugDevice_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetDebugParameter: SetDebugParameter::<Identity, Impl, OFFSET>,
            GetDebugParameter: GetDebugParameter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DebugDevice2 as windows_core::Interface>::IID || iid == &<ID3D12DebugDevice as windows_core::Interface>::IID
    }
}
pub trait ID3D12DescriptorHeap_Impl: Sized + ID3D12Pageable_Impl {
    fn GetDesc(&self) -> D3D12_DESCRIPTOR_HEAP_DESC;
    fn GetCPUDescriptorHandleForHeapStart(&self) -> D3D12_CPU_DESCRIPTOR_HANDLE;
    fn GetGPUDescriptorHandleForHeapStart(&self) -> D3D12_GPU_DESCRIPTOR_HANDLE;
}
impl windows_core::RuntimeName for ID3D12DescriptorHeap {}
impl ID3D12DescriptorHeap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DescriptorHeap_Impl, const OFFSET: isize>() -> ID3D12DescriptorHeap_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DescriptorHeap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_DESCRIPTOR_HEAP_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12DescriptorHeap_Impl::GetDesc(this)
        }
        unsafe extern "system" fn GetCPUDescriptorHandleForHeapStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DescriptorHeap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12DescriptorHeap_Impl::GetCPUDescriptorHandleForHeapStart(this)
        }
        unsafe extern "system" fn GetGPUDescriptorHandleForHeapStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DescriptorHeap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_GPU_DESCRIPTOR_HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12DescriptorHeap_Impl::GetGPUDescriptorHandleForHeapStart(this)
        }
        Self {
            base__: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetCPUDescriptorHandleForHeapStart: GetCPUDescriptorHandleForHeapStart::<Identity, Impl, OFFSET>,
            GetGPUDescriptorHandleForHeapStart: GetGPUDescriptorHandleForHeapStart::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DescriptorHeap as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Pageable as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device_Impl: Sized + ID3D12Object_Impl {
    fn GetNodeCount(&self) -> u32;
    fn CreateCommandQueue(&self, pdesc: *const D3D12_COMMAND_QUEUE_DESC, riid: *const windows_core::GUID, ppcommandqueue: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateCommandAllocator(&self, r#type: D3D12_COMMAND_LIST_TYPE, riid: *const windows_core::GUID, ppcommandallocator: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateGraphicsPipelineState(&self, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const windows_core::GUID, pppipelinestate: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateComputePipelineState(&self, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const windows_core::GUID, pppipelinestate: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateCommandList(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: Option<&ID3D12CommandAllocator>, pinitialstate: Option<&ID3D12PipelineState>, riid: *const windows_core::GUID, ppcommandlist: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CheckFeatureSupport(&self, feature: D3D12_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()>;
    fn CreateDescriptorHeap(&self, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC, riid: *const windows_core::GUID, ppvheap: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDescriptorHandleIncrementSize(&self, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32;
    fn CreateRootSignature(&self, nodemask: u32, pblobwithrootsignature: *const core::ffi::c_void, bloblengthinbytes: usize, riid: *const windows_core::GUID, ppvrootsignature: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateConstantBufferView(&self, pdesc: *const D3D12_CONSTANT_BUFFER_VIEW_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CreateShaderResourceView(&self, presource: Option<&ID3D12Resource>, pdesc: *const D3D12_SHADER_RESOURCE_VIEW_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CreateUnorderedAccessView(&self, presource: Option<&ID3D12Resource>, pcounterresource: Option<&ID3D12Resource>, pdesc: *const D3D12_UNORDERED_ACCESS_VIEW_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CreateRenderTargetView(&self, presource: Option<&ID3D12Resource>, pdesc: *const D3D12_RENDER_TARGET_VIEW_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CreateDepthStencilView(&self, presource: Option<&ID3D12Resource>, pdesc: *const D3D12_DEPTH_STENCIL_VIEW_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CreateSampler(&self, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn CopyDescriptors(&self, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: *const u32, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: *const u32, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE);
    fn CopyDescriptorsSimple(&self, numdescriptors: u32, destdescriptorrangestart: &D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: &D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE);
    fn GetResourceAllocationInfo(&self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC) -> D3D12_RESOURCE_ALLOCATION_INFO;
    fn GetCustomHeapProperties(&self, nodemask: u32, heaptype: D3D12_HEAP_TYPE) -> D3D12_HEAP_PROPERTIES;
    fn CreateCommittedResource(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riidresource: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateHeap(&self, pdesc: *const D3D12_HEAP_DESC, riid: *const windows_core::GUID, ppvheap: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreatePlacedResource(&self, pheap: Option<&ID3D12Heap>, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateReservedResource(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateSharedHandle(&self, pobject: Option<&ID3D12DeviceChild>, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn OpenSharedHandle(&self, nthandle: super::super::Foundation::HANDLE, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OpenSharedHandleByName(&self, name: &windows_core::PCWSTR, access: u32) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn MakeResident(&self, numobjects: u32, ppobjects: *const Option<ID3D12Pageable>) -> windows_core::Result<()>;
    fn Evict(&self, numobjects: u32, ppobjects: *const Option<ID3D12Pageable>) -> windows_core::Result<()>;
    fn CreateFence(&self, initialvalue: u64, flags: D3D12_FENCE_FLAGS, riid: *const windows_core::GUID, ppfence: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDeviceRemovedReason(&self) -> windows_core::Result<()>;
    fn GetCopyableFootprints(&self, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64);
    fn CreateQueryHeap(&self, pdesc: *const D3D12_QUERY_HEAP_DESC, riid: *const windows_core::GUID, ppvheap: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetStablePowerState(&self, enable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn CreateCommandSignature(&self, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: Option<&ID3D12RootSignature>, riid: *const windows_core::GUID, ppvcommandsignature: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetResourceTiling(&self, ptiledresource: Option<&ID3D12Resource>, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D12_PACKED_MIP_INFO, pstandardtileshapefornonpackedmips: *mut D3D12_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING);
    fn GetAdapterLuid(&self) -> super::super::Foundation::LUID;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl windows_core::RuntimeName for ID3D12Device {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>() -> ID3D12Device_Vtbl {
        unsafe extern "system" fn GetNodeCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::GetNodeCount(this)
        }
        unsafe extern "system" fn CreateCommandQueue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_COMMAND_QUEUE_DESC, riid: *const windows_core::GUID, ppcommandqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateCommandQueue(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppcommandqueue)).into()
        }
        unsafe extern "system" fn CreateCommandAllocator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: D3D12_COMMAND_LIST_TYPE, riid: *const windows_core::GUID, ppcommandallocator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateCommandAllocator(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppcommandallocator)).into()
        }
        unsafe extern "system" fn CreateGraphicsPipelineState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const windows_core::GUID, pppipelinestate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateGraphicsPipelineState(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pppipelinestate)).into()
        }
        unsafe extern "system" fn CreateComputePipelineState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const windows_core::GUID, pppipelinestate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateComputePipelineState(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pppipelinestate)).into()
        }
        unsafe extern "system" fn CreateCommandList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: *mut core::ffi::c_void, pinitialstate: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppcommandlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateCommandList(this, core::mem::transmute_copy(&nodemask), core::mem::transmute_copy(&r#type), windows_core::from_raw_borrowed(&pcommandallocator), windows_core::from_raw_borrowed(&pinitialstate), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppcommandlist)).into()
        }
        unsafe extern "system" fn CheckFeatureSupport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: D3D12_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CheckFeatureSupport(this, core::mem::transmute_copy(&feature), core::mem::transmute_copy(&pfeaturesupportdata), core::mem::transmute_copy(&featuresupportdatasize)).into()
        }
        unsafe extern "system" fn CreateDescriptorHeap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC, riid: *const windows_core::GUID, ppvheap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateDescriptorHeap(this, core::mem::transmute_copy(&pdescriptorheapdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn GetDescriptorHandleIncrementSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::GetDescriptorHandleIncrementSize(this, core::mem::transmute_copy(&descriptorheaptype))
        }
        unsafe extern "system" fn CreateRootSignature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodemask: u32, pblobwithrootsignature: *const core::ffi::c_void, bloblengthinbytes: usize, riid: *const windows_core::GUID, ppvrootsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateRootSignature(this, core::mem::transmute_copy(&nodemask), core::mem::transmute_copy(&pblobwithrootsignature), core::mem::transmute_copy(&bloblengthinbytes), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvrootsignature)).into()
        }
        unsafe extern "system" fn CreateConstantBufferView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_CONSTANT_BUFFER_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateConstantBufferView(this, core::mem::transmute_copy(&pdesc), core::mem::transmute(&destdescriptor))
        }
        unsafe extern "system" fn CreateShaderResourceView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D12_SHADER_RESOURCE_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateShaderResourceView(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute(&destdescriptor))
        }
        unsafe extern "system" fn CreateUnorderedAccessView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pcounterresource: *mut core::ffi::c_void, pdesc: *const D3D12_UNORDERED_ACCESS_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateUnorderedAccessView(this, windows_core::from_raw_borrowed(&presource), windows_core::from_raw_borrowed(&pcounterresource), core::mem::transmute_copy(&pdesc), core::mem::transmute(&destdescriptor))
        }
        unsafe extern "system" fn CreateRenderTargetView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D12_RENDER_TARGET_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateRenderTargetView(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute(&destdescriptor))
        }
        unsafe extern "system" fn CreateDepthStencilView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D12_DEPTH_STENCIL_VIEW_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateDepthStencilView(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute(&destdescriptor))
        }
        unsafe extern "system" fn CreateSampler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateSampler(this, core::mem::transmute_copy(&pdesc), core::mem::transmute(&destdescriptor))
        }
        unsafe extern "system" fn CopyDescriptors<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: *const u32, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: *const u32, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CopyDescriptors(this, core::mem::transmute_copy(&numdestdescriptorranges), core::mem::transmute_copy(&pdestdescriptorrangestarts), core::mem::transmute_copy(&pdestdescriptorrangesizes), core::mem::transmute_copy(&numsrcdescriptorranges), core::mem::transmute_copy(&psrcdescriptorrangestarts), core::mem::transmute_copy(&psrcdescriptorrangesizes), core::mem::transmute_copy(&descriptorheapstype))
        }
        unsafe extern "system" fn CopyDescriptorsSimple<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CopyDescriptorsSimple(this, core::mem::transmute_copy(&numdescriptors), core::mem::transmute(&destdescriptorrangestart), core::mem::transmute(&srcdescriptorrangestart), core::mem::transmute_copy(&descriptorheapstype))
        }
        unsafe extern "system" fn GetResourceAllocationInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12Device_Impl::GetResourceAllocationInfo(this, core::mem::transmute_copy(&visiblemask), core::mem::transmute_copy(&numresourcedescs), core::mem::transmute_copy(&presourcedescs))
        }
        unsafe extern "system" fn GetCustomHeapProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_HEAP_PROPERTIES, nodemask: u32, heaptype: D3D12_HEAP_TYPE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12Device_Impl::GetCustomHeapProperties(this, core::mem::transmute_copy(&nodemask), core::mem::transmute_copy(&heaptype))
        }
        unsafe extern "system" fn CreateCommittedResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riidresource: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateCommittedResource(this, core::mem::transmute_copy(&pheapproperties), core::mem::transmute_copy(&heapflags), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&initialresourcestate), core::mem::transmute_copy(&poptimizedclearvalue), core::mem::transmute_copy(&riidresource), core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateHeap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_HEAP_DESC, riid: *const windows_core::GUID, ppvheap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateHeap(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn CreatePlacedResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheap: *mut core::ffi::c_void, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreatePlacedResource(this, windows_core::from_raw_borrowed(&pheap), core::mem::transmute_copy(&heapoffset), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&initialstate), core::mem::transmute_copy(&poptimizedclearvalue), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateReservedResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateReservedResource(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&initialstate), core::mem::transmute_copy(&poptimizedclearvalue), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateSharedHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: windows_core::PCWSTR, phandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D12Device_Impl::CreateSharedHandle(this, windows_core::from_raw_borrowed(&pobject), core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&access), core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(phandle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenSharedHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nthandle: super::super::Foundation::HANDLE, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::OpenSharedHandle(this, core::mem::transmute_copy(&nthandle), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
        }
        unsafe extern "system" fn OpenSharedHandleByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, access: u32, pnthandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D12Device_Impl::OpenSharedHandleByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&access)) {
                Ok(ok__) => {
                    core::ptr::write(pnthandle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeResident<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numobjects: u32, ppobjects: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::MakeResident(this, core::mem::transmute_copy(&numobjects), core::mem::transmute_copy(&ppobjects)).into()
        }
        unsafe extern "system" fn Evict<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numobjects: u32, ppobjects: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::Evict(this, core::mem::transmute_copy(&numobjects), core::mem::transmute_copy(&ppobjects)).into()
        }
        unsafe extern "system" fn CreateFence<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: u64, flags: D3D12_FENCE_FLAGS, riid: *const windows_core::GUID, ppfence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateFence(this, core::mem::transmute_copy(&initialvalue), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppfence)).into()
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::GetDeviceRemovedReason(this).into()
        }
        unsafe extern "system" fn GetCopyableFootprints<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::GetCopyableFootprints(this, core::mem::transmute_copy(&presourcedesc), core::mem::transmute_copy(&firstsubresource), core::mem::transmute_copy(&numsubresources), core::mem::transmute_copy(&baseoffset), core::mem::transmute_copy(&playouts), core::mem::transmute_copy(&pnumrows), core::mem::transmute_copy(&prowsizeinbytes), core::mem::transmute_copy(&ptotalbytes))
        }
        unsafe extern "system" fn CreateQueryHeap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_QUERY_HEAP_DESC, riid: *const windows_core::GUID, ppvheap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateQueryHeap(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn SetStablePowerState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enable: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::SetStablePowerState(this, core::mem::transmute_copy(&enable)).into()
        }
        unsafe extern "system" fn CreateCommandSignature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvcommandsignature: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::CreateCommandSignature(this, core::mem::transmute_copy(&pdesc), windows_core::from_raw_borrowed(&prootsignature), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvcommandsignature)).into()
        }
        unsafe extern "system" fn GetResourceTiling<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptiledresource: *mut core::ffi::c_void, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D12_PACKED_MIP_INFO, pstandardtileshapefornonpackedmips: *mut D3D12_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device_Impl::GetResourceTiling(this, windows_core::from_raw_borrowed(&ptiledresource), core::mem::transmute_copy(&pnumtilesforentireresource), core::mem::transmute_copy(&ppackedmipdesc), core::mem::transmute_copy(&pstandardtileshapefornonpackedmips), core::mem::transmute_copy(&pnumsubresourcetilings), core::mem::transmute_copy(&firstsubresourcetilingtoget), core::mem::transmute_copy(&psubresourcetilingsfornonpackedmips))
        }
        unsafe extern "system" fn GetAdapterLuid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12Device_Impl::GetAdapterLuid(this)
        }
        Self {
            base__: ID3D12Object_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetNodeCount: GetNodeCount::<Identity, Impl, OFFSET>,
            CreateCommandQueue: CreateCommandQueue::<Identity, Impl, OFFSET>,
            CreateCommandAllocator: CreateCommandAllocator::<Identity, Impl, OFFSET>,
            CreateGraphicsPipelineState: CreateGraphicsPipelineState::<Identity, Impl, OFFSET>,
            CreateComputePipelineState: CreateComputePipelineState::<Identity, Impl, OFFSET>,
            CreateCommandList: CreateCommandList::<Identity, Impl, OFFSET>,
            CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET>,
            CreateDescriptorHeap: CreateDescriptorHeap::<Identity, Impl, OFFSET>,
            GetDescriptorHandleIncrementSize: GetDescriptorHandleIncrementSize::<Identity, Impl, OFFSET>,
            CreateRootSignature: CreateRootSignature::<Identity, Impl, OFFSET>,
            CreateConstantBufferView: CreateConstantBufferView::<Identity, Impl, OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Identity, Impl, OFFSET>,
            CreateUnorderedAccessView: CreateUnorderedAccessView::<Identity, Impl, OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Identity, Impl, OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Identity, Impl, OFFSET>,
            CreateSampler: CreateSampler::<Identity, Impl, OFFSET>,
            CopyDescriptors: CopyDescriptors::<Identity, Impl, OFFSET>,
            CopyDescriptorsSimple: CopyDescriptorsSimple::<Identity, Impl, OFFSET>,
            GetResourceAllocationInfo: GetResourceAllocationInfo::<Identity, Impl, OFFSET>,
            GetCustomHeapProperties: GetCustomHeapProperties::<Identity, Impl, OFFSET>,
            CreateCommittedResource: CreateCommittedResource::<Identity, Impl, OFFSET>,
            CreateHeap: CreateHeap::<Identity, Impl, OFFSET>,
            CreatePlacedResource: CreatePlacedResource::<Identity, Impl, OFFSET>,
            CreateReservedResource: CreateReservedResource::<Identity, Impl, OFFSET>,
            CreateSharedHandle: CreateSharedHandle::<Identity, Impl, OFFSET>,
            OpenSharedHandle: OpenSharedHandle::<Identity, Impl, OFFSET>,
            OpenSharedHandleByName: OpenSharedHandleByName::<Identity, Impl, OFFSET>,
            MakeResident: MakeResident::<Identity, Impl, OFFSET>,
            Evict: Evict::<Identity, Impl, OFFSET>,
            CreateFence: CreateFence::<Identity, Impl, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, Impl, OFFSET>,
            GetCopyableFootprints: GetCopyableFootprints::<Identity, Impl, OFFSET>,
            CreateQueryHeap: CreateQueryHeap::<Identity, Impl, OFFSET>,
            SetStablePowerState: SetStablePowerState::<Identity, Impl, OFFSET>,
            CreateCommandSignature: CreateCommandSignature::<Identity, Impl, OFFSET>,
            GetResourceTiling: GetResourceTiling::<Identity, Impl, OFFSET>,
            GetAdapterLuid: GetAdapterLuid::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Device as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device1_Impl: Sized + ID3D12Device_Impl {
    fn CreatePipelineLibrary(&self, plibraryblob: *const core::ffi::c_void, bloblength: usize, riid: *const windows_core::GUID, pppipelinelibrary: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetEventOnMultipleFenceCompletion(&self, ppfences: *const Option<ID3D12Fence>, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetResidencyPriority(&self, numobjects: u32, ppobjects: *const Option<ID3D12Pageable>, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl windows_core::RuntimeName for ID3D12Device1 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device1_Impl, const OFFSET: isize>() -> ID3D12Device1_Vtbl {
        unsafe extern "system" fn CreatePipelineLibrary<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plibraryblob: *const core::ffi::c_void, bloblength: usize, riid: *const windows_core::GUID, pppipelinelibrary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device1_Impl::CreatePipelineLibrary(this, core::mem::transmute_copy(&plibraryblob), core::mem::transmute_copy(&bloblength), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pppipelinelibrary)).into()
        }
        unsafe extern "system" fn SetEventOnMultipleFenceCompletion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfences: *const *mut core::ffi::c_void, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device1_Impl::SetEventOnMultipleFenceCompletion(this, core::mem::transmute_copy(&ppfences), core::mem::transmute_copy(&pfencevalues), core::mem::transmute_copy(&numfences), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn SetResidencyPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numobjects: u32, ppobjects: *const *mut core::ffi::c_void, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device1_Impl::SetResidencyPriority(this, core::mem::transmute_copy(&numobjects), core::mem::transmute_copy(&ppobjects), core::mem::transmute_copy(&ppriorities)).into()
        }
        Self {
            base__: ID3D12Device_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreatePipelineLibrary: CreatePipelineLibrary::<Identity, Impl, OFFSET>,
            SetEventOnMultipleFenceCompletion: SetEventOnMultipleFenceCompletion::<Identity, Impl, OFFSET>,
            SetResidencyPriority: SetResidencyPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Device1 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12Device as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device10_Impl: Sized + ID3D12Device9_Impl {
    fn CreateCommittedResource3(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initiallayout: D3D12_BARRIER_LAYOUT, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: Option<&ID3D12ProtectedResourceSession>, numcastableformats: u32, pcastableformats: *const super::Dxgi::Common::DXGI_FORMAT, riidresource: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreatePlacedResource2(&self, pheap: Option<&ID3D12Heap>, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initiallayout: D3D12_BARRIER_LAYOUT, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, numcastableformats: u32, pcastableformats: *const super::Dxgi::Common::DXGI_FORMAT, riid: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateReservedResource2(&self, pdesc: *const D3D12_RESOURCE_DESC, initiallayout: D3D12_BARRIER_LAYOUT, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: Option<&ID3D12ProtectedResourceSession>, numcastableformats: u32, pcastableformats: *const super::Dxgi::Common::DXGI_FORMAT, riid: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl windows_core::RuntimeName for ID3D12Device10 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device10_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device10_Impl, const OFFSET: isize>() -> ID3D12Device10_Vtbl {
        unsafe extern "system" fn CreateCommittedResource3<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initiallayout: D3D12_BARRIER_LAYOUT, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: *mut core::ffi::c_void, numcastableformats: u32, pcastableformats: *const super::Dxgi::Common::DXGI_FORMAT, riidresource: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device10_Impl::CreateCommittedResource3(this, core::mem::transmute_copy(&pheapproperties), core::mem::transmute_copy(&heapflags), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&initiallayout), core::mem::transmute_copy(&poptimizedclearvalue), windows_core::from_raw_borrowed(&pprotectedsession), core::mem::transmute_copy(&numcastableformats), core::mem::transmute_copy(&pcastableformats), core::mem::transmute_copy(&riidresource), core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreatePlacedResource2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheap: *mut core::ffi::c_void, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initiallayout: D3D12_BARRIER_LAYOUT, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, numcastableformats: u32, pcastableformats: *const super::Dxgi::Common::DXGI_FORMAT, riid: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device10_Impl::CreatePlacedResource2(this, windows_core::from_raw_borrowed(&pheap), core::mem::transmute_copy(&heapoffset), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&initiallayout), core::mem::transmute_copy(&poptimizedclearvalue), core::mem::transmute_copy(&numcastableformats), core::mem::transmute_copy(&pcastableformats), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateReservedResource2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_RESOURCE_DESC, initiallayout: D3D12_BARRIER_LAYOUT, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: *mut core::ffi::c_void, numcastableformats: u32, pcastableformats: *const super::Dxgi::Common::DXGI_FORMAT, riid: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device10_Impl::CreateReservedResource2(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&initiallayout), core::mem::transmute_copy(&poptimizedclearvalue), windows_core::from_raw_borrowed(&pprotectedsession), core::mem::transmute_copy(&numcastableformats), core::mem::transmute_copy(&pcastableformats), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvresource)).into()
        }
        Self {
            base__: ID3D12Device9_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateCommittedResource3: CreateCommittedResource3::<Identity, Impl, OFFSET>,
            CreatePlacedResource2: CreatePlacedResource2::<Identity, Impl, OFFSET>,
            CreateReservedResource2: CreateReservedResource2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Device10 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12Device as windows_core::Interface>::IID || iid == &<ID3D12Device1 as windows_core::Interface>::IID || iid == &<ID3D12Device2 as windows_core::Interface>::IID || iid == &<ID3D12Device3 as windows_core::Interface>::IID || iid == &<ID3D12Device4 as windows_core::Interface>::IID || iid == &<ID3D12Device5 as windows_core::Interface>::IID || iid == &<ID3D12Device6 as windows_core::Interface>::IID || iid == &<ID3D12Device7 as windows_core::Interface>::IID || iid == &<ID3D12Device8 as windows_core::Interface>::IID || iid == &<ID3D12Device9 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device11_Impl: Sized + ID3D12Device10_Impl {
    fn CreateSampler2(&self, pdesc: *const D3D12_SAMPLER_DESC2, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl windows_core::RuntimeName for ID3D12Device11 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device11_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device11_Impl, const OFFSET: isize>() -> ID3D12Device11_Vtbl {
        unsafe extern "system" fn CreateSampler2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device11_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_SAMPLER_DESC2, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device11_Impl::CreateSampler2(this, core::mem::transmute_copy(&pdesc), core::mem::transmute(&destdescriptor))
        }
        Self { base__: ID3D12Device10_Vtbl::new::<Identity, Impl, OFFSET>(), CreateSampler2: CreateSampler2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Device11 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12Device as windows_core::Interface>::IID || iid == &<ID3D12Device1 as windows_core::Interface>::IID || iid == &<ID3D12Device2 as windows_core::Interface>::IID || iid == &<ID3D12Device3 as windows_core::Interface>::IID || iid == &<ID3D12Device4 as windows_core::Interface>::IID || iid == &<ID3D12Device5 as windows_core::Interface>::IID || iid == &<ID3D12Device6 as windows_core::Interface>::IID || iid == &<ID3D12Device7 as windows_core::Interface>::IID || iid == &<ID3D12Device8 as windows_core::Interface>::IID || iid == &<ID3D12Device9 as windows_core::Interface>::IID || iid == &<ID3D12Device10 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device12_Impl: Sized + ID3D12Device11_Impl {
    fn GetResourceAllocationInfo3(&self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, pnumcastableformats: *const u32, ppcastableformats: *const *const super::Dxgi::Common::DXGI_FORMAT, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) -> D3D12_RESOURCE_ALLOCATION_INFO;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl windows_core::RuntimeName for ID3D12Device12 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device12_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device12_Impl, const OFFSET: isize>() -> ID3D12Device12_Vtbl {
        unsafe extern "system" fn GetResourceAllocationInfo3<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device12_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, pnumcastableformats: *const u32, ppcastableformats: *const *const super::Dxgi::Common::DXGI_FORMAT, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12Device12_Impl::GetResourceAllocationInfo3(this, core::mem::transmute_copy(&visiblemask), core::mem::transmute_copy(&numresourcedescs), core::mem::transmute_copy(&presourcedescs), core::mem::transmute_copy(&pnumcastableformats), core::mem::transmute_copy(&ppcastableformats), core::mem::transmute_copy(&presourceallocationinfo1))
        }
        Self { base__: ID3D12Device11_Vtbl::new::<Identity, Impl, OFFSET>(), GetResourceAllocationInfo3: GetResourceAllocationInfo3::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Device12 as windows_core::Interface>::IID
            || iid == &<ID3D12Object as windows_core::Interface>::IID
            || iid == &<ID3D12Device as windows_core::Interface>::IID
            || iid == &<ID3D12Device1 as windows_core::Interface>::IID
            || iid == &<ID3D12Device2 as windows_core::Interface>::IID
            || iid == &<ID3D12Device3 as windows_core::Interface>::IID
            || iid == &<ID3D12Device4 as windows_core::Interface>::IID
            || iid == &<ID3D12Device5 as windows_core::Interface>::IID
            || iid == &<ID3D12Device6 as windows_core::Interface>::IID
            || iid == &<ID3D12Device7 as windows_core::Interface>::IID
            || iid == &<ID3D12Device8 as windows_core::Interface>::IID
            || iid == &<ID3D12Device9 as windows_core::Interface>::IID
            || iid == &<ID3D12Device10 as windows_core::Interface>::IID
            || iid == &<ID3D12Device11 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device13_Impl: Sized + ID3D12Device12_Impl {
    fn OpenExistingHeapFromAddress1(&self, paddress: *const core::ffi::c_void, size: usize, riid: *const windows_core::GUID, ppvheap: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl windows_core::RuntimeName for ID3D12Device13 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device13_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device13_Impl, const OFFSET: isize>() -> ID3D12Device13_Vtbl {
        unsafe extern "system" fn OpenExistingHeapFromAddress1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device13_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *const core::ffi::c_void, size: usize, riid: *const windows_core::GUID, ppvheap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device13_Impl::OpenExistingHeapFromAddress1(this, core::mem::transmute_copy(&paddress), core::mem::transmute_copy(&size), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvheap)).into()
        }
        Self {
            base__: ID3D12Device12_Vtbl::new::<Identity, Impl, OFFSET>(),
            OpenExistingHeapFromAddress1: OpenExistingHeapFromAddress1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Device13 as windows_core::Interface>::IID
            || iid == &<ID3D12Object as windows_core::Interface>::IID
            || iid == &<ID3D12Device as windows_core::Interface>::IID
            || iid == &<ID3D12Device1 as windows_core::Interface>::IID
            || iid == &<ID3D12Device2 as windows_core::Interface>::IID
            || iid == &<ID3D12Device3 as windows_core::Interface>::IID
            || iid == &<ID3D12Device4 as windows_core::Interface>::IID
            || iid == &<ID3D12Device5 as windows_core::Interface>::IID
            || iid == &<ID3D12Device6 as windows_core::Interface>::IID
            || iid == &<ID3D12Device7 as windows_core::Interface>::IID
            || iid == &<ID3D12Device8 as windows_core::Interface>::IID
            || iid == &<ID3D12Device9 as windows_core::Interface>::IID
            || iid == &<ID3D12Device10 as windows_core::Interface>::IID
            || iid == &<ID3D12Device11 as windows_core::Interface>::IID
            || iid == &<ID3D12Device12 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device2_Impl: Sized + ID3D12Device1_Impl {
    fn CreatePipelineState(&self, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const windows_core::GUID, pppipelinestate: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl windows_core::RuntimeName for ID3D12Device2 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device2_Impl, const OFFSET: isize>() -> ID3D12Device2_Vtbl {
        unsafe extern "system" fn CreatePipelineState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const windows_core::GUID, pppipelinestate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device2_Impl::CreatePipelineState(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pppipelinestate)).into()
        }
        Self { base__: ID3D12Device1_Vtbl::new::<Identity, Impl, OFFSET>(), CreatePipelineState: CreatePipelineState::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Device2 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12Device as windows_core::Interface>::IID || iid == &<ID3D12Device1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device3_Impl: Sized + ID3D12Device2_Impl {
    fn OpenExistingHeapFromAddress(&self, paddress: *const core::ffi::c_void, riid: *const windows_core::GUID, ppvheap: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OpenExistingHeapFromFileMapping(&self, hfilemapping: super::super::Foundation::HANDLE, riid: *const windows_core::GUID, ppvheap: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EnqueueMakeResident(&self, flags: D3D12_RESIDENCY_FLAGS, numobjects: u32, ppobjects: *const Option<ID3D12Pageable>, pfencetosignal: Option<&ID3D12Fence>, fencevaluetosignal: u64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl windows_core::RuntimeName for ID3D12Device3 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device3_Impl, const OFFSET: isize>() -> ID3D12Device3_Vtbl {
        unsafe extern "system" fn OpenExistingHeapFromAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *const core::ffi::c_void, riid: *const windows_core::GUID, ppvheap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device3_Impl::OpenExistingHeapFromAddress(this, core::mem::transmute_copy(&paddress), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn OpenExistingHeapFromFileMapping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfilemapping: super::super::Foundation::HANDLE, riid: *const windows_core::GUID, ppvheap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device3_Impl::OpenExistingHeapFromFileMapping(this, core::mem::transmute_copy(&hfilemapping), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn EnqueueMakeResident<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: D3D12_RESIDENCY_FLAGS, numobjects: u32, ppobjects: *const *mut core::ffi::c_void, pfencetosignal: *mut core::ffi::c_void, fencevaluetosignal: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device3_Impl::EnqueueMakeResident(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&numobjects), core::mem::transmute_copy(&ppobjects), windows_core::from_raw_borrowed(&pfencetosignal), core::mem::transmute_copy(&fencevaluetosignal)).into()
        }
        Self {
            base__: ID3D12Device2_Vtbl::new::<Identity, Impl, OFFSET>(),
            OpenExistingHeapFromAddress: OpenExistingHeapFromAddress::<Identity, Impl, OFFSET>,
            OpenExistingHeapFromFileMapping: OpenExistingHeapFromFileMapping::<Identity, Impl, OFFSET>,
            EnqueueMakeResident: EnqueueMakeResident::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Device3 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12Device as windows_core::Interface>::IID || iid == &<ID3D12Device1 as windows_core::Interface>::IID || iid == &<ID3D12Device2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device4_Impl: Sized + ID3D12Device3_Impl {
    fn CreateCommandList1(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS, riid: *const windows_core::GUID, ppcommandlist: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateProtectedResourceSession(&self, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC, riid: *const windows_core::GUID, ppsession: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateCommittedResource1(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: Option<&ID3D12ProtectedResourceSession>, riidresource: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateHeap1(&self, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: Option<&ID3D12ProtectedResourceSession>, riid: *const windows_core::GUID, ppvheap: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateReservedResource1(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: Option<&ID3D12ProtectedResourceSession>, riid: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetResourceAllocationInfo1(&self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) -> D3D12_RESOURCE_ALLOCATION_INFO;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl windows_core::RuntimeName for ID3D12Device4 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device4_Impl, const OFFSET: isize>() -> ID3D12Device4_Vtbl {
        unsafe extern "system" fn CreateCommandList1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS, riid: *const windows_core::GUID, ppcommandlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device4_Impl::CreateCommandList1(this, core::mem::transmute_copy(&nodemask), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppcommandlist)).into()
        }
        unsafe extern "system" fn CreateProtectedResourceSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC, riid: *const windows_core::GUID, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device4_Impl::CreateProtectedResourceSession(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppsession)).into()
        }
        unsafe extern "system" fn CreateCommittedResource1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: *mut core::ffi::c_void, riidresource: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device4_Impl::CreateCommittedResource1(this, core::mem::transmute_copy(&pheapproperties), core::mem::transmute_copy(&heapflags), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&initialresourcestate), core::mem::transmute_copy(&poptimizedclearvalue), windows_core::from_raw_borrowed(&pprotectedsession), core::mem::transmute_copy(&riidresource), core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateHeap1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvheap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device4_Impl::CreateHeap1(this, core::mem::transmute_copy(&pdesc), windows_core::from_raw_borrowed(&pprotectedsession), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvheap)).into()
        }
        unsafe extern "system" fn CreateReservedResource1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device4_Impl::CreateReservedResource1(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&initialstate), core::mem::transmute_copy(&poptimizedclearvalue), windows_core::from_raw_borrowed(&pprotectedsession), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn GetResourceAllocationInfo1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12Device4_Impl::GetResourceAllocationInfo1(this, core::mem::transmute_copy(&visiblemask), core::mem::transmute_copy(&numresourcedescs), core::mem::transmute_copy(&presourcedescs), core::mem::transmute_copy(&presourceallocationinfo1))
        }
        Self {
            base__: ID3D12Device3_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateCommandList1: CreateCommandList1::<Identity, Impl, OFFSET>,
            CreateProtectedResourceSession: CreateProtectedResourceSession::<Identity, Impl, OFFSET>,
            CreateCommittedResource1: CreateCommittedResource1::<Identity, Impl, OFFSET>,
            CreateHeap1: CreateHeap1::<Identity, Impl, OFFSET>,
            CreateReservedResource1: CreateReservedResource1::<Identity, Impl, OFFSET>,
            GetResourceAllocationInfo1: GetResourceAllocationInfo1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Device4 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12Device as windows_core::Interface>::IID || iid == &<ID3D12Device1 as windows_core::Interface>::IID || iid == &<ID3D12Device2 as windows_core::Interface>::IID || iid == &<ID3D12Device3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device5_Impl: Sized + ID3D12Device4_Impl {
    fn CreateLifetimeTracker(&self, powner: Option<&ID3D12LifetimeOwner>, riid: *const windows_core::GUID, ppvtracker: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RemoveDevice(&self);
    fn EnumerateMetaCommands(&self, pnummetacommands: *mut u32, pdescs: *mut D3D12_META_COMMAND_DESC) -> windows_core::Result<()>;
    fn EnumerateMetaCommandParameters(&self, commandid: *const windows_core::GUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, ptotalstructuresizeinbytes: *mut u32, pparametercount: *mut u32, pparameterdescs: *mut D3D12_META_COMMAND_PARAMETER_DESC) -> windows_core::Result<()>;
    fn CreateMetaCommand(&self, commandid: *const windows_core::GUID, nodemask: u32, pcreationparametersdata: *const core::ffi::c_void, creationparametersdatasizeinbytes: usize, riid: *const windows_core::GUID, ppmetacommand: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateStateObject(&self, pdesc: *const D3D12_STATE_OBJECT_DESC, riid: *const windows_core::GUID, ppstateobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetRaytracingAccelerationStructurePrebuildInfo(&self, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pinfo: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO);
    fn CheckDriverMatchingIdentifier(&self, serializeddatatype: D3D12_SERIALIZED_DATA_TYPE, pidentifiertocheck: *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl windows_core::RuntimeName for ID3D12Device5 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: isize>() -> ID3D12Device5_Vtbl {
        unsafe extern "system" fn CreateLifetimeTracker<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, powner: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvtracker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device5_Impl::CreateLifetimeTracker(this, windows_core::from_raw_borrowed(&powner), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvtracker)).into()
        }
        unsafe extern "system" fn RemoveDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device5_Impl::RemoveDevice(this)
        }
        unsafe extern "system" fn EnumerateMetaCommands<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnummetacommands: *mut u32, pdescs: *mut D3D12_META_COMMAND_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device5_Impl::EnumerateMetaCommands(this, core::mem::transmute_copy(&pnummetacommands), core::mem::transmute_copy(&pdescs)).into()
        }
        unsafe extern "system" fn EnumerateMetaCommandParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: *const windows_core::GUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, ptotalstructuresizeinbytes: *mut u32, pparametercount: *mut u32, pparameterdescs: *mut D3D12_META_COMMAND_PARAMETER_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device5_Impl::EnumerateMetaCommandParameters(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&stage), core::mem::transmute_copy(&ptotalstructuresizeinbytes), core::mem::transmute_copy(&pparametercount), core::mem::transmute_copy(&pparameterdescs)).into()
        }
        unsafe extern "system" fn CreateMetaCommand<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: *const windows_core::GUID, nodemask: u32, pcreationparametersdata: *const core::ffi::c_void, creationparametersdatasizeinbytes: usize, riid: *const windows_core::GUID, ppmetacommand: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device5_Impl::CreateMetaCommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&nodemask), core::mem::transmute_copy(&pcreationparametersdata), core::mem::transmute_copy(&creationparametersdatasizeinbytes), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppmetacommand)).into()
        }
        unsafe extern "system" fn CreateStateObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_STATE_OBJECT_DESC, riid: *const windows_core::GUID, ppstateobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device5_Impl::CreateStateObject(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppstateobject)).into()
        }
        unsafe extern "system" fn GetRaytracingAccelerationStructurePrebuildInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pinfo: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device5_Impl::GetRaytracingAccelerationStructurePrebuildInfo(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinfo))
        }
        unsafe extern "system" fn CheckDriverMatchingIdentifier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serializeddatatype: D3D12_SERIALIZED_DATA_TYPE, pidentifiertocheck: *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device5_Impl::CheckDriverMatchingIdentifier(this, core::mem::transmute_copy(&serializeddatatype), core::mem::transmute_copy(&pidentifiertocheck))
        }
        Self {
            base__: ID3D12Device4_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateLifetimeTracker: CreateLifetimeTracker::<Identity, Impl, OFFSET>,
            RemoveDevice: RemoveDevice::<Identity, Impl, OFFSET>,
            EnumerateMetaCommands: EnumerateMetaCommands::<Identity, Impl, OFFSET>,
            EnumerateMetaCommandParameters: EnumerateMetaCommandParameters::<Identity, Impl, OFFSET>,
            CreateMetaCommand: CreateMetaCommand::<Identity, Impl, OFFSET>,
            CreateStateObject: CreateStateObject::<Identity, Impl, OFFSET>,
            GetRaytracingAccelerationStructurePrebuildInfo: GetRaytracingAccelerationStructurePrebuildInfo::<Identity, Impl, OFFSET>,
            CheckDriverMatchingIdentifier: CheckDriverMatchingIdentifier::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Device5 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12Device as windows_core::Interface>::IID || iid == &<ID3D12Device1 as windows_core::Interface>::IID || iid == &<ID3D12Device2 as windows_core::Interface>::IID || iid == &<ID3D12Device3 as windows_core::Interface>::IID || iid == &<ID3D12Device4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device6_Impl: Sized + ID3D12Device5_Impl {
    fn SetBackgroundProcessingMode(&self, mode: D3D12_BACKGROUND_PROCESSING_MODE, measurementsaction: D3D12_MEASUREMENTS_ACTION, heventtosignaluponcompletion: super::super::Foundation::HANDLE, pbfurthermeasurementsdesired: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl windows_core::RuntimeName for ID3D12Device6 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device6_Impl, const OFFSET: isize>() -> ID3D12Device6_Vtbl {
        unsafe extern "system" fn SetBackgroundProcessingMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: D3D12_BACKGROUND_PROCESSING_MODE, measurementsaction: D3D12_MEASUREMENTS_ACTION, heventtosignaluponcompletion: super::super::Foundation::HANDLE, pbfurthermeasurementsdesired: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device6_Impl::SetBackgroundProcessingMode(this, core::mem::transmute_copy(&mode), core::mem::transmute_copy(&measurementsaction), core::mem::transmute_copy(&heventtosignaluponcompletion), core::mem::transmute_copy(&pbfurthermeasurementsdesired)).into()
        }
        Self { base__: ID3D12Device5_Vtbl::new::<Identity, Impl, OFFSET>(), SetBackgroundProcessingMode: SetBackgroundProcessingMode::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Device6 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12Device as windows_core::Interface>::IID || iid == &<ID3D12Device1 as windows_core::Interface>::IID || iid == &<ID3D12Device2 as windows_core::Interface>::IID || iid == &<ID3D12Device3 as windows_core::Interface>::IID || iid == &<ID3D12Device4 as windows_core::Interface>::IID || iid == &<ID3D12Device5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device7_Impl: Sized + ID3D12Device6_Impl {
    fn AddToStateObject(&self, paddition: *const D3D12_STATE_OBJECT_DESC, pstateobjecttogrowfrom: Option<&ID3D12StateObject>, riid: *const windows_core::GUID, ppnewstateobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateProtectedResourceSession1(&self, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC1, riid: *const windows_core::GUID, ppsession: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl windows_core::RuntimeName for ID3D12Device7 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device7_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device7_Impl, const OFFSET: isize>() -> ID3D12Device7_Vtbl {
        unsafe extern "system" fn AddToStateObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddition: *const D3D12_STATE_OBJECT_DESC, pstateobjecttogrowfrom: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppnewstateobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device7_Impl::AddToStateObject(this, core::mem::transmute_copy(&paddition), windows_core::from_raw_borrowed(&pstateobjecttogrowfrom), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppnewstateobject)).into()
        }
        unsafe extern "system" fn CreateProtectedResourceSession1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC1, riid: *const windows_core::GUID, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device7_Impl::CreateProtectedResourceSession1(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppsession)).into()
        }
        Self {
            base__: ID3D12Device6_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddToStateObject: AddToStateObject::<Identity, Impl, OFFSET>,
            CreateProtectedResourceSession1: CreateProtectedResourceSession1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Device7 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12Device as windows_core::Interface>::IID || iid == &<ID3D12Device1 as windows_core::Interface>::IID || iid == &<ID3D12Device2 as windows_core::Interface>::IID || iid == &<ID3D12Device3 as windows_core::Interface>::IID || iid == &<ID3D12Device4 as windows_core::Interface>::IID || iid == &<ID3D12Device5 as windows_core::Interface>::IID || iid == &<ID3D12Device6 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device8_Impl: Sized + ID3D12Device7_Impl {
    fn GetResourceAllocationInfo2(&self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) -> D3D12_RESOURCE_ALLOCATION_INFO;
    fn CreateCommittedResource2(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: Option<&ID3D12ProtectedResourceSession>, riidresource: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreatePlacedResource1(&self, pheap: Option<&ID3D12Heap>, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateSamplerFeedbackUnorderedAccessView(&self, ptargetedresource: Option<&ID3D12Resource>, pfeedbackresource: Option<&ID3D12Resource>, destdescriptor: &D3D12_CPU_DESCRIPTOR_HANDLE);
    fn GetCopyableFootprints1(&self, presourcedesc: *const D3D12_RESOURCE_DESC1, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64);
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl windows_core::RuntimeName for ID3D12Device8 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device8_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device8_Impl, const OFFSET: isize>() -> ID3D12Device8_Vtbl {
        unsafe extern "system" fn GetResourceAllocationInfo2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_RESOURCE_ALLOCATION_INFO, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, presourceallocationinfo1: *mut D3D12_RESOURCE_ALLOCATION_INFO1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12Device8_Impl::GetResourceAllocationInfo2(this, core::mem::transmute_copy(&visiblemask), core::mem::transmute_copy(&numresourcedescs), core::mem::transmute_copy(&presourcedescs), core::mem::transmute_copy(&presourceallocationinfo1))
        }
        unsafe extern "system" fn CreateCommittedResource2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, pprotectedsession: *mut core::ffi::c_void, riidresource: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device8_Impl::CreateCommittedResource2(this, core::mem::transmute_copy(&pheapproperties), core::mem::transmute_copy(&heapflags), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&initialresourcestate), core::mem::transmute_copy(&poptimizedclearvalue), windows_core::from_raw_borrowed(&pprotectedsession), core::mem::transmute_copy(&riidresource), core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreatePlacedResource1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheap: *mut core::ffi::c_void, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: *const D3D12_CLEAR_VALUE, riid: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device8_Impl::CreatePlacedResource1(this, windows_core::from_raw_borrowed(&pheap), core::mem::transmute_copy(&heapoffset), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&initialstate), core::mem::transmute_copy(&poptimizedclearvalue), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvresource)).into()
        }
        unsafe extern "system" fn CreateSamplerFeedbackUnorderedAccessView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetedresource: *mut core::ffi::c_void, pfeedbackresource: *mut core::ffi::c_void, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device8_Impl::CreateSamplerFeedbackUnorderedAccessView(this, windows_core::from_raw_borrowed(&ptargetedresource), windows_core::from_raw_borrowed(&pfeedbackresource), core::mem::transmute(&destdescriptor))
        }
        unsafe extern "system" fn GetCopyableFootprints1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcedesc: *const D3D12_RESOURCE_DESC1, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, pnumrows: *mut u32, prowsizeinbytes: *mut u64, ptotalbytes: *mut u64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device8_Impl::GetCopyableFootprints1(this, core::mem::transmute_copy(&presourcedesc), core::mem::transmute_copy(&firstsubresource), core::mem::transmute_copy(&numsubresources), core::mem::transmute_copy(&baseoffset), core::mem::transmute_copy(&playouts), core::mem::transmute_copy(&pnumrows), core::mem::transmute_copy(&prowsizeinbytes), core::mem::transmute_copy(&ptotalbytes))
        }
        Self {
            base__: ID3D12Device7_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetResourceAllocationInfo2: GetResourceAllocationInfo2::<Identity, Impl, OFFSET>,
            CreateCommittedResource2: CreateCommittedResource2::<Identity, Impl, OFFSET>,
            CreatePlacedResource1: CreatePlacedResource1::<Identity, Impl, OFFSET>,
            CreateSamplerFeedbackUnorderedAccessView: CreateSamplerFeedbackUnorderedAccessView::<Identity, Impl, OFFSET>,
            GetCopyableFootprints1: GetCopyableFootprints1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Device8 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12Device as windows_core::Interface>::IID || iid == &<ID3D12Device1 as windows_core::Interface>::IID || iid == &<ID3D12Device2 as windows_core::Interface>::IID || iid == &<ID3D12Device3 as windows_core::Interface>::IID || iid == &<ID3D12Device4 as windows_core::Interface>::IID || iid == &<ID3D12Device5 as windows_core::Interface>::IID || iid == &<ID3D12Device6 as windows_core::Interface>::IID || iid == &<ID3D12Device7 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
pub trait ID3D12Device9_Impl: Sized + ID3D12Device8_Impl {
    fn CreateShaderCacheSession(&self, pdesc: *const D3D12_SHADER_CACHE_SESSION_DESC, riid: *const windows_core::GUID, ppvsession: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShaderCacheControl(&self, kinds: D3D12_SHADER_CACHE_KIND_FLAGS, control: D3D12_SHADER_CACHE_CONTROL_FLAGS) -> windows_core::Result<()>;
    fn CreateCommandQueue1(&self, pdesc: *const D3D12_COMMAND_QUEUE_DESC, creatorid: *const windows_core::GUID, riid: *const windows_core::GUID, ppcommandqueue: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl windows_core::RuntimeName for ID3D12Device9 {}
#[cfg(all(feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Security"))]
impl ID3D12Device9_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device9_Impl, const OFFSET: isize>() -> ID3D12Device9_Vtbl {
        unsafe extern "system" fn CreateShaderCacheSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_SHADER_CACHE_SESSION_DESC, riid: *const windows_core::GUID, ppvsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device9_Impl::CreateShaderCacheSession(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvsession)).into()
        }
        unsafe extern "system" fn ShaderCacheControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kinds: D3D12_SHADER_CACHE_KIND_FLAGS, control: D3D12_SHADER_CACHE_CONTROL_FLAGS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device9_Impl::ShaderCacheControl(this, core::mem::transmute_copy(&kinds), core::mem::transmute_copy(&control)).into()
        }
        unsafe extern "system" fn CreateCommandQueue1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Device9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_COMMAND_QUEUE_DESC, creatorid: *const windows_core::GUID, riid: *const windows_core::GUID, ppcommandqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Device9_Impl::CreateCommandQueue1(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&creatorid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppcommandqueue)).into()
        }
        Self {
            base__: ID3D12Device8_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateShaderCacheSession: CreateShaderCacheSession::<Identity, Impl, OFFSET>,
            ShaderCacheControl: ShaderCacheControl::<Identity, Impl, OFFSET>,
            CreateCommandQueue1: CreateCommandQueue1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Device9 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12Device as windows_core::Interface>::IID || iid == &<ID3D12Device1 as windows_core::Interface>::IID || iid == &<ID3D12Device2 as windows_core::Interface>::IID || iid == &<ID3D12Device3 as windows_core::Interface>::IID || iid == &<ID3D12Device4 as windows_core::Interface>::IID || iid == &<ID3D12Device5 as windows_core::Interface>::IID || iid == &<ID3D12Device6 as windows_core::Interface>::IID || iid == &<ID3D12Device7 as windows_core::Interface>::IID || iid == &<ID3D12Device8 as windows_core::Interface>::IID
    }
}
pub trait ID3D12DeviceChild_Impl: Sized + ID3D12Object_Impl {
    fn GetDevice(&self, riid: *const windows_core::GUID, ppvdevice: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D12DeviceChild {}
impl ID3D12DeviceChild_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceChild_Impl, const OFFSET: isize>() -> ID3D12DeviceChild_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceChild_Impl::GetDevice(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvdevice)).into()
        }
        Self { base__: ID3D12Object_Vtbl::new::<Identity, Impl, OFFSET>(), GetDevice: GetDevice::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D12DeviceConfiguration_Impl: Sized {
    fn GetDesc(&self) -> D3D12_DEVICE_CONFIGURATION_DESC;
    fn GetEnabledExperimentalFeatures(&self, pguids: *mut windows_core::GUID, numguids: u32) -> windows_core::Result<()>;
    fn SerializeVersionedRootSignature(&self, pdesc: *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppresult: *mut Option<super::Direct3D::ID3DBlob>, pperror: *mut Option<super::Direct3D::ID3DBlob>) -> windows_core::Result<()>;
    fn CreateVersionedRootSignatureDeserializer(&self, pblob: *const core::ffi::c_void, size: usize, riid: *const windows_core::GUID, ppvdeserializer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::RuntimeName for ID3D12DeviceConfiguration {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12DeviceConfiguration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceConfiguration_Impl, const OFFSET: isize>() -> ID3D12DeviceConfiguration_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_DEVICE_CONFIGURATION_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12DeviceConfiguration_Impl::GetDesc(this)
        }
        unsafe extern "system" fn GetEnabledExperimentalFeatures<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguids: *mut windows_core::GUID, numguids: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceConfiguration_Impl::GetEnabledExperimentalFeatures(this, core::mem::transmute_copy(&pguids), core::mem::transmute_copy(&numguids)).into()
        }
        unsafe extern "system" fn SerializeVersionedRootSignature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppresult: *mut *mut core::ffi::c_void, pperror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceConfiguration_Impl::SerializeVersionedRootSignature(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppresult), core::mem::transmute_copy(&pperror)).into()
        }
        unsafe extern "system" fn CreateVersionedRootSignatureDeserializer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblob: *const core::ffi::c_void, size: usize, riid: *const windows_core::GUID, ppvdeserializer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceConfiguration_Impl::CreateVersionedRootSignatureDeserializer(this, core::mem::transmute_copy(&pblob), core::mem::transmute_copy(&size), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvdeserializer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetEnabledExperimentalFeatures: GetEnabledExperimentalFeatures::<Identity, Impl, OFFSET>,
            SerializeVersionedRootSignature: SerializeVersionedRootSignature::<Identity, Impl, OFFSET>,
            CreateVersionedRootSignatureDeserializer: CreateVersionedRootSignatureDeserializer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DeviceConfiguration as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D12DeviceFactory_Impl: Sized {
    fn InitializeFromGlobalState(&self) -> windows_core::Result<()>;
    fn ApplyToGlobalState(&self) -> windows_core::Result<()>;
    fn SetFlags(&self, flags: D3D12_DEVICE_FACTORY_FLAGS) -> windows_core::Result<()>;
    fn GetFlags(&self) -> D3D12_DEVICE_FACTORY_FLAGS;
    fn GetConfigurationInterface(&self, clsid: *const windows_core::GUID, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EnableExperimentalFeatures(&self, numfeatures: u32, piids: *const windows_core::GUID, pconfigurationstructs: *const core::ffi::c_void, pconfigurationstructsizes: *const u32) -> windows_core::Result<()>;
    fn CreateDevice(&self, adapter: Option<&windows_core::IUnknown>, featurelevel: super::Direct3D::D3D_FEATURE_LEVEL, riid: *const windows_core::GUID, ppvdevice: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::RuntimeName for ID3D12DeviceFactory {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12DeviceFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: isize>() -> ID3D12DeviceFactory_Vtbl {
        unsafe extern "system" fn InitializeFromGlobalState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceFactory_Impl::InitializeFromGlobalState(this).into()
        }
        unsafe extern "system" fn ApplyToGlobalState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceFactory_Impl::ApplyToGlobalState(this).into()
        }
        unsafe extern "system" fn SetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: D3D12_DEVICE_FACTORY_FLAGS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceFactory_Impl::SetFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D12_DEVICE_FACTORY_FLAGS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceFactory_Impl::GetFlags(this)
        }
        unsafe extern "system" fn GetConfigurationInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, iid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceFactory_Impl::GetConfigurationInterface(this, core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn EnableExperimentalFeatures<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numfeatures: u32, piids: *const windows_core::GUID, pconfigurationstructs: *const core::ffi::c_void, pconfigurationstructsizes: *const u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceFactory_Impl::EnableExperimentalFeatures(this, core::mem::transmute_copy(&numfeatures), core::mem::transmute_copy(&piids), core::mem::transmute_copy(&pconfigurationstructs), core::mem::transmute_copy(&pconfigurationstructsizes)).into()
        }
        unsafe extern "system" fn CreateDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapter: *mut core::ffi::c_void, featurelevel: super::Direct3D::D3D_FEATURE_LEVEL, riid: *const windows_core::GUID, ppvdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceFactory_Impl::CreateDevice(this, windows_core::from_raw_borrowed(&adapter), core::mem::transmute_copy(&featurelevel), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvdevice)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromGlobalState: InitializeFromGlobalState::<Identity, Impl, OFFSET>,
            ApplyToGlobalState: ApplyToGlobalState::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetConfigurationInterface: GetConfigurationInterface::<Identity, Impl, OFFSET>,
            EnableExperimentalFeatures: EnableExperimentalFeatures::<Identity, Impl, OFFSET>,
            CreateDevice: CreateDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DeviceFactory as windows_core::Interface>::IID
    }
}
pub trait ID3D12DeviceRemovedExtendedData_Impl: Sized {
    fn GetAutoBreadcrumbsOutput(&self) -> windows_core::Result<D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT>;
    fn GetPageFaultAllocationOutput(&self) -> windows_core::Result<D3D12_DRED_PAGE_FAULT_OUTPUT>;
}
impl windows_core::RuntimeName for ID3D12DeviceRemovedExtendedData {}
impl ID3D12DeviceRemovedExtendedData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData_Impl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedData_Vtbl {
        unsafe extern "system" fn GetAutoBreadcrumbsOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutput: *mut D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D12DeviceRemovedExtendedData_Impl::GetAutoBreadcrumbsOutput(this) {
                Ok(ok__) => {
                    core::ptr::write(poutput, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageFaultAllocationOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D12DeviceRemovedExtendedData_Impl::GetPageFaultAllocationOutput(this) {
                Ok(ok__) => {
                    core::ptr::write(poutput, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAutoBreadcrumbsOutput: GetAutoBreadcrumbsOutput::<Identity, Impl, OFFSET>,
            GetPageFaultAllocationOutput: GetPageFaultAllocationOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DeviceRemovedExtendedData as windows_core::Interface>::IID
    }
}
pub trait ID3D12DeviceRemovedExtendedData1_Impl: Sized + ID3D12DeviceRemovedExtendedData_Impl {
    fn GetAutoBreadcrumbsOutput1(&self) -> windows_core::Result<D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1>;
    fn GetPageFaultAllocationOutput1(&self) -> windows_core::Result<D3D12_DRED_PAGE_FAULT_OUTPUT1>;
}
impl windows_core::RuntimeName for ID3D12DeviceRemovedExtendedData1 {}
impl ID3D12DeviceRemovedExtendedData1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData1_Impl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedData1_Vtbl {
        unsafe extern "system" fn GetAutoBreadcrumbsOutput1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutput: *mut D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D12DeviceRemovedExtendedData1_Impl::GetAutoBreadcrumbsOutput1(this) {
                Ok(ok__) => {
                    core::ptr::write(poutput, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPageFaultAllocationOutput1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT1) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D12DeviceRemovedExtendedData1_Impl::GetPageFaultAllocationOutput1(this) {
                Ok(ok__) => {
                    core::ptr::write(poutput, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID3D12DeviceRemovedExtendedData_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAutoBreadcrumbsOutput1: GetAutoBreadcrumbsOutput1::<Identity, Impl, OFFSET>,
            GetPageFaultAllocationOutput1: GetPageFaultAllocationOutput1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DeviceRemovedExtendedData1 as windows_core::Interface>::IID || iid == &<ID3D12DeviceRemovedExtendedData as windows_core::Interface>::IID
    }
}
pub trait ID3D12DeviceRemovedExtendedData2_Impl: Sized + ID3D12DeviceRemovedExtendedData1_Impl {
    fn GetPageFaultAllocationOutput2(&self, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT2) -> windows_core::Result<()>;
    fn GetDeviceState(&self) -> D3D12_DRED_DEVICE_STATE;
}
impl windows_core::RuntimeName for ID3D12DeviceRemovedExtendedData2 {}
impl ID3D12DeviceRemovedExtendedData2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData2_Impl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedData2_Vtbl {
        unsafe extern "system" fn GetPageFaultAllocationOutput2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutput: *mut D3D12_DRED_PAGE_FAULT_OUTPUT2) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceRemovedExtendedData2_Impl::GetPageFaultAllocationOutput2(this, core::mem::transmute_copy(&poutput)).into()
        }
        unsafe extern "system" fn GetDeviceState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedData2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D12_DRED_DEVICE_STATE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceRemovedExtendedData2_Impl::GetDeviceState(this)
        }
        Self {
            base__: ID3D12DeviceRemovedExtendedData1_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPageFaultAllocationOutput2: GetPageFaultAllocationOutput2::<Identity, Impl, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DeviceRemovedExtendedData2 as windows_core::Interface>::IID || iid == &<ID3D12DeviceRemovedExtendedData as windows_core::Interface>::IID || iid == &<ID3D12DeviceRemovedExtendedData1 as windows_core::Interface>::IID
    }
}
pub trait ID3D12DeviceRemovedExtendedDataSettings_Impl: Sized {
    fn SetAutoBreadcrumbsEnablement(&self, enablement: D3D12_DRED_ENABLEMENT);
    fn SetPageFaultEnablement(&self, enablement: D3D12_DRED_ENABLEMENT);
    fn SetWatsonDumpEnablement(&self, enablement: D3D12_DRED_ENABLEMENT);
}
impl windows_core::RuntimeName for ID3D12DeviceRemovedExtendedDataSettings {}
impl ID3D12DeviceRemovedExtendedDataSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedDataSettings_Vtbl {
        unsafe extern "system" fn SetAutoBreadcrumbsEnablement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceRemovedExtendedDataSettings_Impl::SetAutoBreadcrumbsEnablement(this, core::mem::transmute_copy(&enablement))
        }
        unsafe extern "system" fn SetPageFaultEnablement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceRemovedExtendedDataSettings_Impl::SetPageFaultEnablement(this, core::mem::transmute_copy(&enablement))
        }
        unsafe extern "system" fn SetWatsonDumpEnablement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceRemovedExtendedDataSettings_Impl::SetWatsonDumpEnablement(this, core::mem::transmute_copy(&enablement))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAutoBreadcrumbsEnablement: SetAutoBreadcrumbsEnablement::<Identity, Impl, OFFSET>,
            SetPageFaultEnablement: SetPageFaultEnablement::<Identity, Impl, OFFSET>,
            SetWatsonDumpEnablement: SetWatsonDumpEnablement::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DeviceRemovedExtendedDataSettings as windows_core::Interface>::IID
    }
}
pub trait ID3D12DeviceRemovedExtendedDataSettings1_Impl: Sized + ID3D12DeviceRemovedExtendedDataSettings_Impl {
    fn SetBreadcrumbContextEnablement(&self, enablement: D3D12_DRED_ENABLEMENT);
}
impl windows_core::RuntimeName for ID3D12DeviceRemovedExtendedDataSettings1 {}
impl ID3D12DeviceRemovedExtendedDataSettings1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings1_Impl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedDataSettings1_Vtbl {
        unsafe extern "system" fn SetBreadcrumbContextEnablement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enablement: D3D12_DRED_ENABLEMENT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceRemovedExtendedDataSettings1_Impl::SetBreadcrumbContextEnablement(this, core::mem::transmute_copy(&enablement))
        }
        Self {
            base__: ID3D12DeviceRemovedExtendedDataSettings_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetBreadcrumbContextEnablement: SetBreadcrumbContextEnablement::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DeviceRemovedExtendedDataSettings1 as windows_core::Interface>::IID || iid == &<ID3D12DeviceRemovedExtendedDataSettings as windows_core::Interface>::IID
    }
}
pub trait ID3D12DeviceRemovedExtendedDataSettings2_Impl: Sized + ID3D12DeviceRemovedExtendedDataSettings1_Impl {
    fn UseMarkersOnlyAutoBreadcrumbs(&self, markersonly: super::super::Foundation::BOOL);
}
impl windows_core::RuntimeName for ID3D12DeviceRemovedExtendedDataSettings2 {}
impl ID3D12DeviceRemovedExtendedDataSettings2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings2_Impl, const OFFSET: isize>() -> ID3D12DeviceRemovedExtendedDataSettings2_Vtbl {
        unsafe extern "system" fn UseMarkersOnlyAutoBreadcrumbs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12DeviceRemovedExtendedDataSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, markersonly: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12DeviceRemovedExtendedDataSettings2_Impl::UseMarkersOnlyAutoBreadcrumbs(this, core::mem::transmute_copy(&markersonly))
        }
        Self {
            base__: ID3D12DeviceRemovedExtendedDataSettings1_Vtbl::new::<Identity, Impl, OFFSET>(),
            UseMarkersOnlyAutoBreadcrumbs: UseMarkersOnlyAutoBreadcrumbs::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12DeviceRemovedExtendedDataSettings2 as windows_core::Interface>::IID || iid == &<ID3D12DeviceRemovedExtendedDataSettings as windows_core::Interface>::IID || iid == &<ID3D12DeviceRemovedExtendedDataSettings1 as windows_core::Interface>::IID
    }
}
pub trait ID3D12Fence_Impl: Sized + ID3D12Pageable_Impl {
    fn GetCompletedValue(&self) -> u64;
    fn SetEventOnCompletion(&self, value: u64, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn Signal(&self, value: u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D12Fence {}
impl ID3D12Fence_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Fence_Impl, const OFFSET: isize>() -> ID3D12Fence_Vtbl {
        unsafe extern "system" fn GetCompletedValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Fence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Fence_Impl::GetCompletedValue(this)
        }
        unsafe extern "system" fn SetEventOnCompletion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Fence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64, hevent: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Fence_Impl::SetEventOnCompletion(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn Signal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Fence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Fence_Impl::Signal(this, core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCompletedValue: GetCompletedValue::<Identity, Impl, OFFSET>,
            SetEventOnCompletion: SetEventOnCompletion::<Identity, Impl, OFFSET>,
            Signal: Signal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Fence as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Pageable as windows_core::Interface>::IID
    }
}
pub trait ID3D12Fence1_Impl: Sized + ID3D12Fence_Impl {
    fn GetCreationFlags(&self) -> D3D12_FENCE_FLAGS;
}
impl windows_core::RuntimeName for ID3D12Fence1 {}
impl ID3D12Fence1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Fence1_Impl, const OFFSET: isize>() -> ID3D12Fence1_Vtbl {
        unsafe extern "system" fn GetCreationFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Fence1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D12_FENCE_FLAGS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Fence1_Impl::GetCreationFlags(this)
        }
        Self { base__: ID3D12Fence_Vtbl::new::<Identity, Impl, OFFSET>(), GetCreationFlags: GetCreationFlags::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Fence1 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Pageable as windows_core::Interface>::IID || iid == &<ID3D12Fence as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D12FunctionParameterReflection_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D12_PARAMETER_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12FunctionParameterReflection_Vtbl {
    pub const fn new<Impl: ID3D12FunctionParameterReflection_Impl>() -> ID3D12FunctionParameterReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12FunctionParameterReflection_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D12_PARAMETER_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12FunctionParameterReflection_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        Self { GetDesc: GetDesc::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D12FunctionParameterReflection_ImplVtbl<T: ID3D12FunctionParameterReflection_Impl>(std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D12FunctionParameterReflection_Impl> ID3D12FunctionParameterReflection_ImplVtbl<T> {
    const VTABLE: ID3D12FunctionParameterReflection_Vtbl = ID3D12FunctionParameterReflection_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12FunctionParameterReflection {
    pub fn new<'a, T: ID3D12FunctionParameterReflection_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D12FunctionParameterReflection_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D12FunctionReflection_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D12_FUNCTION_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, bufferindex: u32) -> Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &windows_core::PCSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetFunctionParameter(&self, parameterindex: i32) -> Option<ID3D12FunctionParameterReflection>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12FunctionReflection_Vtbl {
    pub const fn new<Impl: ID3D12FunctionReflection_Impl>() -> ID3D12FunctionReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12FunctionReflection_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D12_FUNCTION_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12FunctionReflection_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D12FunctionReflection_Impl>(this: *mut core::ffi::c_void, bufferindex: u32) -> Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12FunctionReflection_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&bufferindex))
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D12FunctionReflection_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12FunctionReflection_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D12FunctionReflection_Impl>(this: *mut core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12FunctionReflection_Impl::GetResourceBindingDesc(this, core::mem::transmute_copy(&resourceindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D12FunctionReflection_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12FunctionReflection_Impl::GetVariableByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D12FunctionReflection_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12FunctionReflection_Impl::GetResourceBindingDescByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetFunctionParameter<Impl: ID3D12FunctionReflection_Impl>(this: *mut core::ffi::c_void, parameterindex: i32) -> Option<ID3D12FunctionParameterReflection> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12FunctionReflection_Impl::GetFunctionParameter(this, core::mem::transmute_copy(&parameterindex))
        }
        Self {
            GetDesc: GetDesc::<Impl>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Impl>,
            GetConstantBufferByName: GetConstantBufferByName::<Impl>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Impl>,
            GetVariableByName: GetVariableByName::<Impl>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Impl>,
            GetFunctionParameter: GetFunctionParameter::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D12FunctionReflection_ImplVtbl<T: ID3D12FunctionReflection_Impl>(std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D12FunctionReflection_Impl> ID3D12FunctionReflection_ImplVtbl<T> {
    const VTABLE: ID3D12FunctionReflection_Vtbl = ID3D12FunctionReflection_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12FunctionReflection {
    pub fn new<'a, T: ID3D12FunctionReflection_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D12FunctionReflection_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList_Impl: Sized + ID3D12CommandList_Impl {
    fn Close(&self) -> windows_core::Result<()>;
    fn Reset(&self, pallocator: Option<&ID3D12CommandAllocator>, pinitialstate: Option<&ID3D12PipelineState>) -> windows_core::Result<()>;
    fn ClearState(&self, ppipelinestate: Option<&ID3D12PipelineState>);
    fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32);
    fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32);
    fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32);
    fn CopyBufferRegion(&self, pdstbuffer: Option<&ID3D12Resource>, dstoffset: u64, psrcbuffer: Option<&ID3D12Resource>, srcoffset: u64, numbytes: u64);
    fn CopyTextureRegion(&self, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: *const D3D12_BOX);
    fn CopyResource(&self, pdstresource: Option<&ID3D12Resource>, psrcresource: Option<&ID3D12Resource>);
    fn CopyTiles(&self, ptiledresource: Option<&ID3D12Resource>, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: Option<&ID3D12Resource>, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS);
    fn ResolveSubresource(&self, pdstresource: Option<&ID3D12Resource>, dstsubresource: u32, psrcresource: Option<&ID3D12Resource>, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT);
    fn IASetPrimitiveTopology(&self, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn RSSetViewports(&self, numviewports: u32, pviewports: *const D3D12_VIEWPORT);
    fn RSSetScissorRects(&self, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn OMSetBlendFactor(&self, blendfactor: *const f32);
    fn OMSetStencilRef(&self, stencilref: u32);
    fn SetPipelineState(&self, ppipelinestate: Option<&ID3D12PipelineState>);
    fn ResourceBarrier(&self, numbarriers: u32, pbarriers: *const D3D12_RESOURCE_BARRIER);
    fn ExecuteBundle(&self, pcommandlist: Option<&ID3D12GraphicsCommandList>);
    fn SetDescriptorHeaps(&self, numdescriptorheaps: u32, ppdescriptorheaps: *const Option<ID3D12DescriptorHeap>);
    fn SetComputeRootSignature(&self, prootsignature: Option<&ID3D12RootSignature>);
    fn SetGraphicsRootSignature(&self, prootsignature: Option<&ID3D12RootSignature>);
    fn SetComputeRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: &D3D12_GPU_DESCRIPTOR_HANDLE);
    fn SetGraphicsRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: &D3D12_GPU_DESCRIPTOR_HANDLE);
    fn SetComputeRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32);
    fn SetGraphicsRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32);
    fn SetComputeRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const core::ffi::c_void, destoffsetin32bitvalues: u32);
    fn SetGraphicsRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const core::ffi::c_void, destoffsetin32bitvalues: u32);
    fn SetComputeRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64);
    fn SetGraphicsRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64);
    fn SetComputeRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64);
    fn SetGraphicsRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64);
    fn SetComputeRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64);
    fn SetGraphicsRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64);
    fn IASetIndexBuffer(&self, pview: *const D3D12_INDEX_BUFFER_VIEW);
    fn IASetVertexBuffers(&self, startslot: u32, numviews: u32, pviews: *const D3D12_VERTEX_BUFFER_VIEW);
    fn SOSetTargets(&self, startslot: u32, numviews: u32, pviews: *const D3D12_STREAM_OUTPUT_BUFFER_VIEW);
    fn OMSetRenderTargets(&self, numrendertargetdescriptors: u32, prendertargetdescriptors: *const D3D12_CPU_DESCRIPTOR_HANDLE, rtssinglehandletodescriptorrange: super::super::Foundation::BOOL, pdepthstencildescriptor: *const D3D12_CPU_DESCRIPTOR_HANDLE);
    fn ClearDepthStencilView(&self, depthstencilview: &D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn ClearRenderTargetView(&self, rendertargetview: &D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn ClearUnorderedAccessViewUint(&self, viewgpuhandleincurrentheap: &D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: &D3D12_CPU_DESCRIPTOR_HANDLE, presource: Option<&ID3D12Resource>, values: *const u32, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn ClearUnorderedAccessViewFloat(&self, viewgpuhandleincurrentheap: &D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: &D3D12_CPU_DESCRIPTOR_HANDLE, presource: Option<&ID3D12Resource>, values: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn DiscardResource(&self, presource: Option<&ID3D12Resource>, pregion: *const D3D12_DISCARD_REGION);
    fn BeginQuery(&self, pqueryheap: Option<&ID3D12QueryHeap>, r#type: D3D12_QUERY_TYPE, index: u32);
    fn EndQuery(&self, pqueryheap: Option<&ID3D12QueryHeap>, r#type: D3D12_QUERY_TYPE, index: u32);
    fn ResolveQueryData(&self, pqueryheap: Option<&ID3D12QueryHeap>, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: Option<&ID3D12Resource>, aligneddestinationbufferoffset: u64);
    fn SetPredication(&self, pbuffer: Option<&ID3D12Resource>, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP);
    fn SetMarker(&self, metadata: u32, pdata: *const core::ffi::c_void, size: u32);
    fn BeginEvent(&self, metadata: u32, pdata: *const core::ffi::c_void, size: u32);
    fn EndEvent(&self);
    fn ExecuteIndirect(&self, pcommandsignature: Option<&ID3D12CommandSignature>, maxcommandcount: u32, pargumentbuffer: Option<&ID3D12Resource>, argumentbufferoffset: u64, pcountbuffer: Option<&ID3D12Resource>, countbufferoffset: u64);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D12GraphicsCommandList {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList_Vtbl {
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::Close(this).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallocator: *mut core::ffi::c_void, pinitialstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::Reset(this, windows_core::from_raw_borrowed(&pallocator), windows_core::from_raw_borrowed(&pinitialstate)).into()
        }
        unsafe extern "system" fn ClearState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppipelinestate: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::ClearState(this, windows_core::from_raw_borrowed(&ppipelinestate))
        }
        unsafe extern "system" fn DrawInstanced<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::DrawInstanced(this, core::mem::transmute_copy(&vertexcountperinstance), core::mem::transmute_copy(&instancecount), core::mem::transmute_copy(&startvertexlocation), core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn DrawIndexedInstanced<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::DrawIndexedInstanced(this, core::mem::transmute_copy(&indexcountperinstance), core::mem::transmute_copy(&instancecount), core::mem::transmute_copy(&startindexlocation), core::mem::transmute_copy(&basevertexlocation), core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn Dispatch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::Dispatch(this, core::mem::transmute_copy(&threadgroupcountx), core::mem::transmute_copy(&threadgroupcounty), core::mem::transmute_copy(&threadgroupcountz))
        }
        unsafe extern "system" fn CopyBufferRegion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstbuffer: *mut core::ffi::c_void, dstoffset: u64, psrcbuffer: *mut core::ffi::c_void, srcoffset: u64, numbytes: u64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::CopyBufferRegion(this, windows_core::from_raw_borrowed(&pdstbuffer), core::mem::transmute_copy(&dstoffset), windows_core::from_raw_borrowed(&psrcbuffer), core::mem::transmute_copy(&srcoffset), core::mem::transmute_copy(&numbytes))
        }
        unsafe extern "system" fn CopyTextureRegion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: *const D3D12_BOX) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::CopyTextureRegion(this, core::mem::transmute_copy(&pdst), core::mem::transmute_copy(&dstx), core::mem::transmute_copy(&dsty), core::mem::transmute_copy(&dstz), core::mem::transmute_copy(&psrc), core::mem::transmute_copy(&psrcbox))
        }
        unsafe extern "system" fn CopyResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, psrcresource: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::CopyResource(this, windows_core::from_raw_borrowed(&pdstresource), windows_core::from_raw_borrowed(&psrcresource))
        }
        unsafe extern "system" fn CopyTiles<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptiledresource: *mut core::ffi::c_void, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: *mut core::ffi::c_void, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::CopyTiles(this, windows_core::from_raw_borrowed(&ptiledresource), core::mem::transmute_copy(&ptileregionstartcoordinate), core::mem::transmute_copy(&ptileregionsize), windows_core::from_raw_borrowed(&pbuffer), core::mem::transmute_copy(&bufferstartoffsetinbytes), core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn ResolveSubresource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::ResolveSubresource(this, windows_core::from_raw_borrowed(&pdstresource), core::mem::transmute_copy(&dstsubresource), windows_core::from_raw_borrowed(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&format))
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::IASetPrimitiveTopology(this, core::mem::transmute_copy(&primitivetopology))
        }
        unsafe extern "system" fn RSSetViewports<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviewports: u32, pviewports: *const D3D12_VIEWPORT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::RSSetViewports(this, core::mem::transmute_copy(&numviewports), core::mem::transmute_copy(&pviewports))
        }
        unsafe extern "system" fn RSSetScissorRects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::RSSetScissorRects(this, core::mem::transmute_copy(&numrects), core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn OMSetBlendFactor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blendfactor: *const f32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::OMSetBlendFactor(this, core::mem::transmute_copy(&blendfactor))
        }
        unsafe extern "system" fn OMSetStencilRef<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stencilref: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::OMSetStencilRef(this, core::mem::transmute_copy(&stencilref))
        }
        unsafe extern "system" fn SetPipelineState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppipelinestate: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetPipelineState(this, windows_core::from_raw_borrowed(&ppipelinestate))
        }
        unsafe extern "system" fn ResourceBarrier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbarriers: u32, pbarriers: *const D3D12_RESOURCE_BARRIER) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::ResourceBarrier(this, core::mem::transmute_copy(&numbarriers), core::mem::transmute_copy(&pbarriers))
        }
        unsafe extern "system" fn ExecuteBundle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommandlist: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::ExecuteBundle(this, windows_core::from_raw_borrowed(&pcommandlist))
        }
        unsafe extern "system" fn SetDescriptorHeaps<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numdescriptorheaps: u32, ppdescriptorheaps: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetDescriptorHeaps(this, core::mem::transmute_copy(&numdescriptorheaps), core::mem::transmute_copy(&ppdescriptorheaps))
        }
        unsafe extern "system" fn SetComputeRootSignature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prootsignature: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetComputeRootSignature(this, windows_core::from_raw_borrowed(&prootsignature))
        }
        unsafe extern "system" fn SetGraphicsRootSignature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prootsignature: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetGraphicsRootSignature(this, windows_core::from_raw_borrowed(&prootsignature))
        }
        unsafe extern "system" fn SetComputeRootDescriptorTable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetComputeRootDescriptorTable(this, core::mem::transmute_copy(&rootparameterindex), core::mem::transmute(&basedescriptor))
        }
        unsafe extern "system" fn SetGraphicsRootDescriptorTable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetGraphicsRootDescriptorTable(this, core::mem::transmute_copy(&rootparameterindex), core::mem::transmute(&basedescriptor))
        }
        unsafe extern "system" fn SetComputeRoot32BitConstant<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetComputeRoot32BitConstant(this, core::mem::transmute_copy(&rootparameterindex), core::mem::transmute_copy(&srcdata), core::mem::transmute_copy(&destoffsetin32bitvalues))
        }
        unsafe extern "system" fn SetGraphicsRoot32BitConstant<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetGraphicsRoot32BitConstant(this, core::mem::transmute_copy(&rootparameterindex), core::mem::transmute_copy(&srcdata), core::mem::transmute_copy(&destoffsetin32bitvalues))
        }
        unsafe extern "system" fn SetComputeRoot32BitConstants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const core::ffi::c_void, destoffsetin32bitvalues: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetComputeRoot32BitConstants(this, core::mem::transmute_copy(&rootparameterindex), core::mem::transmute_copy(&num32bitvaluestoset), core::mem::transmute_copy(&psrcdata), core::mem::transmute_copy(&destoffsetin32bitvalues))
        }
        unsafe extern "system" fn SetGraphicsRoot32BitConstants<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const core::ffi::c_void, destoffsetin32bitvalues: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetGraphicsRoot32BitConstants(this, core::mem::transmute_copy(&rootparameterindex), core::mem::transmute_copy(&num32bitvaluestoset), core::mem::transmute_copy(&psrcdata), core::mem::transmute_copy(&destoffsetin32bitvalues))
        }
        unsafe extern "system" fn SetComputeRootConstantBufferView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetComputeRootConstantBufferView(this, core::mem::transmute_copy(&rootparameterindex), core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn SetGraphicsRootConstantBufferView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetGraphicsRootConstantBufferView(this, core::mem::transmute_copy(&rootparameterindex), core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn SetComputeRootShaderResourceView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetComputeRootShaderResourceView(this, core::mem::transmute_copy(&rootparameterindex), core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn SetGraphicsRootShaderResourceView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetGraphicsRootShaderResourceView(this, core::mem::transmute_copy(&rootparameterindex), core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn SetComputeRootUnorderedAccessView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetComputeRootUnorderedAccessView(this, core::mem::transmute_copy(&rootparameterindex), core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn SetGraphicsRootUnorderedAccessView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rootparameterindex: u32, bufferlocation: u64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetGraphicsRootUnorderedAccessView(this, core::mem::transmute_copy(&rootparameterindex), core::mem::transmute_copy(&bufferlocation))
        }
        unsafe extern "system" fn IASetIndexBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pview: *const D3D12_INDEX_BUFFER_VIEW) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::IASetIndexBuffer(this, core::mem::transmute_copy(&pview))
        }
        unsafe extern "system" fn IASetVertexBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, pviews: *const D3D12_VERTEX_BUFFER_VIEW) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::IASetVertexBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&pviews))
        }
        unsafe extern "system" fn SOSetTargets<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, pviews: *const D3D12_STREAM_OUTPUT_BUFFER_VIEW) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SOSetTargets(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&pviews))
        }
        unsafe extern "system" fn OMSetRenderTargets<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numrendertargetdescriptors: u32, prendertargetdescriptors: *const D3D12_CPU_DESCRIPTOR_HANDLE, rtssinglehandletodescriptorrange: super::super::Foundation::BOOL, pdepthstencildescriptor: *const D3D12_CPU_DESCRIPTOR_HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::OMSetRenderTargets(this, core::mem::transmute_copy(&numrendertargetdescriptors), core::mem::transmute_copy(&prendertargetdescriptors), core::mem::transmute_copy(&rtssinglehandletodescriptorrange), core::mem::transmute_copy(&pdepthstencildescriptor))
        }
        unsafe extern "system" fn ClearDepthStencilView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depthstencilview: D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::ClearDepthStencilView(this, core::mem::transmute(&depthstencilview), core::mem::transmute_copy(&clearflags), core::mem::transmute_copy(&depth), core::mem::transmute_copy(&stencil), core::mem::transmute_copy(&numrects), core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn ClearRenderTargetView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rendertargetview: D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::ClearRenderTargetView(this, core::mem::transmute(&rendertargetview), core::mem::transmute_copy(&colorrgba), core::mem::transmute_copy(&numrects), core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewUint<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: *mut core::ffi::c_void, values: *const u32, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::ClearUnorderedAccessViewUint(this, core::mem::transmute(&viewgpuhandleincurrentheap), core::mem::transmute(&viewcpuhandle), windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&values), core::mem::transmute_copy(&numrects), core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewFloat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: *mut core::ffi::c_void, values: *const f32, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::ClearUnorderedAccessViewFloat(this, core::mem::transmute(&viewgpuhandleincurrentheap), core::mem::transmute(&viewcpuhandle), windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&values), core::mem::transmute_copy(&numrects), core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn DiscardResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pregion: *const D3D12_DISCARD_REGION) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::DiscardResource(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pregion))
        }
        unsafe extern "system" fn BeginQuery<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqueryheap: *mut core::ffi::c_void, r#type: D3D12_QUERY_TYPE, index: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::BeginQuery(this, windows_core::from_raw_borrowed(&pqueryheap), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn EndQuery<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqueryheap: *mut core::ffi::c_void, r#type: D3D12_QUERY_TYPE, index: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::EndQuery(this, windows_core::from_raw_borrowed(&pqueryheap), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn ResolveQueryData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqueryheap: *mut core::ffi::c_void, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: *mut core::ffi::c_void, aligneddestinationbufferoffset: u64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::ResolveQueryData(this, windows_core::from_raw_borrowed(&pqueryheap), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&numqueries), windows_core::from_raw_borrowed(&pdestinationbuffer), core::mem::transmute_copy(&aligneddestinationbufferoffset))
        }
        unsafe extern "system" fn SetPredication<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetPredication(this, windows_core::from_raw_borrowed(&pbuffer), core::mem::transmute_copy(&alignedbufferoffset), core::mem::transmute_copy(&operation))
        }
        unsafe extern "system" fn SetMarker<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadata: u32, pdata: *const core::ffi::c_void, size: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::SetMarker(this, core::mem::transmute_copy(&metadata), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size))
        }
        unsafe extern "system" fn BeginEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadata: u32, pdata: *const core::ffi::c_void, size: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::BeginEvent(this, core::mem::transmute_copy(&metadata), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&size))
        }
        unsafe extern "system" fn EndEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::EndEvent(this)
        }
        unsafe extern "system" fn ExecuteIndirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommandsignature: *mut core::ffi::c_void, maxcommandcount: u32, pargumentbuffer: *mut core::ffi::c_void, argumentbufferoffset: u64, pcountbuffer: *mut core::ffi::c_void, countbufferoffset: u64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList_Impl::ExecuteIndirect(this, windows_core::from_raw_borrowed(&pcommandsignature), core::mem::transmute_copy(&maxcommandcount), windows_core::from_raw_borrowed(&pargumentbuffer), core::mem::transmute_copy(&argumentbufferoffset), windows_core::from_raw_borrowed(&pcountbuffer), core::mem::transmute_copy(&countbufferoffset))
        }
        Self {
            base__: ID3D12CommandList_Vtbl::new::<Identity, Impl, OFFSET>(),
            Close: Close::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            ClearState: ClearState::<Identity, Impl, OFFSET>,
            DrawInstanced: DrawInstanced::<Identity, Impl, OFFSET>,
            DrawIndexedInstanced: DrawIndexedInstanced::<Identity, Impl, OFFSET>,
            Dispatch: Dispatch::<Identity, Impl, OFFSET>,
            CopyBufferRegion: CopyBufferRegion::<Identity, Impl, OFFSET>,
            CopyTextureRegion: CopyTextureRegion::<Identity, Impl, OFFSET>,
            CopyResource: CopyResource::<Identity, Impl, OFFSET>,
            CopyTiles: CopyTiles::<Identity, Impl, OFFSET>,
            ResolveSubresource: ResolveSubresource::<Identity, Impl, OFFSET>,
            IASetPrimitiveTopology: IASetPrimitiveTopology::<Identity, Impl, OFFSET>,
            RSSetViewports: RSSetViewports::<Identity, Impl, OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Identity, Impl, OFFSET>,
            OMSetBlendFactor: OMSetBlendFactor::<Identity, Impl, OFFSET>,
            OMSetStencilRef: OMSetStencilRef::<Identity, Impl, OFFSET>,
            SetPipelineState: SetPipelineState::<Identity, Impl, OFFSET>,
            ResourceBarrier: ResourceBarrier::<Identity, Impl, OFFSET>,
            ExecuteBundle: ExecuteBundle::<Identity, Impl, OFFSET>,
            SetDescriptorHeaps: SetDescriptorHeaps::<Identity, Impl, OFFSET>,
            SetComputeRootSignature: SetComputeRootSignature::<Identity, Impl, OFFSET>,
            SetGraphicsRootSignature: SetGraphicsRootSignature::<Identity, Impl, OFFSET>,
            SetComputeRootDescriptorTable: SetComputeRootDescriptorTable::<Identity, Impl, OFFSET>,
            SetGraphicsRootDescriptorTable: SetGraphicsRootDescriptorTable::<Identity, Impl, OFFSET>,
            SetComputeRoot32BitConstant: SetComputeRoot32BitConstant::<Identity, Impl, OFFSET>,
            SetGraphicsRoot32BitConstant: SetGraphicsRoot32BitConstant::<Identity, Impl, OFFSET>,
            SetComputeRoot32BitConstants: SetComputeRoot32BitConstants::<Identity, Impl, OFFSET>,
            SetGraphicsRoot32BitConstants: SetGraphicsRoot32BitConstants::<Identity, Impl, OFFSET>,
            SetComputeRootConstantBufferView: SetComputeRootConstantBufferView::<Identity, Impl, OFFSET>,
            SetGraphicsRootConstantBufferView: SetGraphicsRootConstantBufferView::<Identity, Impl, OFFSET>,
            SetComputeRootShaderResourceView: SetComputeRootShaderResourceView::<Identity, Impl, OFFSET>,
            SetGraphicsRootShaderResourceView: SetGraphicsRootShaderResourceView::<Identity, Impl, OFFSET>,
            SetComputeRootUnorderedAccessView: SetComputeRootUnorderedAccessView::<Identity, Impl, OFFSET>,
            SetGraphicsRootUnorderedAccessView: SetGraphicsRootUnorderedAccessView::<Identity, Impl, OFFSET>,
            IASetIndexBuffer: IASetIndexBuffer::<Identity, Impl, OFFSET>,
            IASetVertexBuffers: IASetVertexBuffers::<Identity, Impl, OFFSET>,
            SOSetTargets: SOSetTargets::<Identity, Impl, OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Identity, Impl, OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Identity, Impl, OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Identity, Impl, OFFSET>,
            ClearUnorderedAccessViewUint: ClearUnorderedAccessViewUint::<Identity, Impl, OFFSET>,
            ClearUnorderedAccessViewFloat: ClearUnorderedAccessViewFloat::<Identity, Impl, OFFSET>,
            DiscardResource: DiscardResource::<Identity, Impl, OFFSET>,
            BeginQuery: BeginQuery::<Identity, Impl, OFFSET>,
            EndQuery: EndQuery::<Identity, Impl, OFFSET>,
            ResolveQueryData: ResolveQueryData::<Identity, Impl, OFFSET>,
            SetPredication: SetPredication::<Identity, Impl, OFFSET>,
            SetMarker: SetMarker::<Identity, Impl, OFFSET>,
            BeginEvent: BeginEvent::<Identity, Impl, OFFSET>,
            EndEvent: EndEvent::<Identity, Impl, OFFSET>,
            ExecuteIndirect: ExecuteIndirect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12CommandList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList1_Impl: Sized + ID3D12GraphicsCommandList_Impl {
    fn AtomicCopyBufferUINT(&self, pdstbuffer: Option<&ID3D12Resource>, dstoffset: u64, psrcbuffer: Option<&ID3D12Resource>, srcoffset: u64, dependencies: u32, ppdependentresources: *const Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64);
    fn AtomicCopyBufferUINT64(&self, pdstbuffer: Option<&ID3D12Resource>, dstoffset: u64, psrcbuffer: Option<&ID3D12Resource>, srcoffset: u64, dependencies: u32, ppdependentresources: *const Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64);
    fn OMSetDepthBounds(&self, min: f32, max: f32);
    fn SetSamplePositions(&self, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION);
    fn ResolveSubresourceRegion(&self, pdstresource: Option<&ID3D12Resource>, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: Option<&ID3D12Resource>, srcsubresource: u32, psrcrect: *const super::super::Foundation::RECT, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE);
    fn SetViewInstanceMask(&self, mask: u32);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D12GraphicsCommandList1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList1_Vtbl {
        unsafe extern "system" fn AtomicCopyBufferUINT<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstbuffer: *mut core::ffi::c_void, dstoffset: u64, psrcbuffer: *mut core::ffi::c_void, srcoffset: u64, dependencies: u32, ppdependentresources: *const *mut core::ffi::c_void, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList1_Impl::AtomicCopyBufferUINT(this, windows_core::from_raw_borrowed(&pdstbuffer), core::mem::transmute_copy(&dstoffset), windows_core::from_raw_borrowed(&psrcbuffer), core::mem::transmute_copy(&srcoffset), core::mem::transmute_copy(&dependencies), core::mem::transmute_copy(&ppdependentresources), core::mem::transmute_copy(&pdependentsubresourceranges))
        }
        unsafe extern "system" fn AtomicCopyBufferUINT64<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstbuffer: *mut core::ffi::c_void, dstoffset: u64, psrcbuffer: *mut core::ffi::c_void, srcoffset: u64, dependencies: u32, ppdependentresources: *const *mut core::ffi::c_void, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList1_Impl::AtomicCopyBufferUINT64(this, windows_core::from_raw_borrowed(&pdstbuffer), core::mem::transmute_copy(&dstoffset), windows_core::from_raw_borrowed(&psrcbuffer), core::mem::transmute_copy(&srcoffset), core::mem::transmute_copy(&dependencies), core::mem::transmute_copy(&ppdependentresources), core::mem::transmute_copy(&pdependentsubresourceranges))
        }
        unsafe extern "system" fn OMSetDepthBounds<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, min: f32, max: f32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList1_Impl::OMSetDepthBounds(this, core::mem::transmute_copy(&min), core::mem::transmute_copy(&max))
        }
        unsafe extern "system" fn SetSamplePositions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList1_Impl::SetSamplePositions(this, core::mem::transmute_copy(&numsamplesperpixel), core::mem::transmute_copy(&numpixels), core::mem::transmute_copy(&psamplepositions))
        }
        unsafe extern "system" fn ResolveSubresourceRegion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, psrcrect: *const super::super::Foundation::RECT, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList1_Impl::ResolveSubresourceRegion(this, windows_core::from_raw_borrowed(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&dstx), core::mem::transmute_copy(&dsty), windows_core::from_raw_borrowed(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&psrcrect), core::mem::transmute_copy(&format), core::mem::transmute_copy(&resolvemode))
        }
        unsafe extern "system" fn SetViewInstanceMask<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mask: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList1_Impl::SetViewInstanceMask(this, core::mem::transmute_copy(&mask))
        }
        Self {
            base__: ID3D12GraphicsCommandList_Vtbl::new::<Identity, Impl, OFFSET>(),
            AtomicCopyBufferUINT: AtomicCopyBufferUINT::<Identity, Impl, OFFSET>,
            AtomicCopyBufferUINT64: AtomicCopyBufferUINT64::<Identity, Impl, OFFSET>,
            OMSetDepthBounds: OMSetDepthBounds::<Identity, Impl, OFFSET>,
            SetSamplePositions: SetSamplePositions::<Identity, Impl, OFFSET>,
            ResolveSubresourceRegion: ResolveSubresourceRegion::<Identity, Impl, OFFSET>,
            SetViewInstanceMask: SetViewInstanceMask::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList1 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList2_Impl: Sized + ID3D12GraphicsCommandList1_Impl {
    fn WriteBufferImmediate(&self, count: u32, pparams: *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const D3D12_WRITEBUFFERIMMEDIATE_MODE);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D12GraphicsCommandList2 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList2_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList2_Vtbl {
        unsafe extern "system" fn WriteBufferImmediate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, pparams: *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: *const D3D12_WRITEBUFFERIMMEDIATE_MODE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList2_Impl::WriteBufferImmediate(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&pparams), core::mem::transmute_copy(&pmodes))
        }
        Self { base__: ID3D12GraphicsCommandList1_Vtbl::new::<Identity, Impl, OFFSET>(), WriteBufferImmediate: WriteBufferImmediate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList2 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList3_Impl: Sized + ID3D12GraphicsCommandList2_Impl {
    fn SetProtectedResourceSession(&self, pprotectedresourcesession: Option<&ID3D12ProtectedResourceSession>);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D12GraphicsCommandList3 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList3_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList3_Vtbl {
        unsafe extern "system" fn SetProtectedResourceSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotectedresourcesession: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList3_Impl::SetProtectedResourceSession(this, windows_core::from_raw_borrowed(&pprotectedresourcesession))
        }
        Self {
            base__: ID3D12GraphicsCommandList2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetProtectedResourceSession: SetProtectedResourceSession::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList3 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList1 as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList4_Impl: Sized + ID3D12GraphicsCommandList3_Impl {
    fn BeginRenderPass(&self, numrendertargets: u32, prendertargets: *const D3D12_RENDER_PASS_RENDER_TARGET_DESC, pdepthstencil: *const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC, flags: D3D12_RENDER_PASS_FLAGS);
    fn EndRenderPass(&self);
    fn InitializeMetaCommand(&self, pmetacommand: Option<&ID3D12MetaCommand>, pinitializationparametersdata: *const core::ffi::c_void, initializationparametersdatasizeinbytes: usize);
    fn ExecuteMetaCommand(&self, pmetacommand: Option<&ID3D12MetaCommand>, pexecutionparametersdata: *const core::ffi::c_void, executionparametersdatasizeinbytes: usize);
    fn BuildRaytracingAccelerationStructure(&self, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, numpostbuildinfodescs: u32, ppostbuildinfodescs: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC);
    fn EmitRaytracingAccelerationStructurePostbuildInfo(&self, pdesc: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, numsourceaccelerationstructures: u32, psourceaccelerationstructuredata: *const u64);
    fn CopyRaytracingAccelerationStructure(&self, destaccelerationstructuredata: u64, sourceaccelerationstructuredata: u64, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE);
    fn SetPipelineState1(&self, pstateobject: Option<&ID3D12StateObject>);
    fn DispatchRays(&self, pdesc: *const D3D12_DISPATCH_RAYS_DESC);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D12GraphicsCommandList4 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList4_Vtbl {
        unsafe extern "system" fn BeginRenderPass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numrendertargets: u32, prendertargets: *const D3D12_RENDER_PASS_RENDER_TARGET_DESC, pdepthstencil: *const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC, flags: D3D12_RENDER_PASS_FLAGS) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList4_Impl::BeginRenderPass(this, core::mem::transmute_copy(&numrendertargets), core::mem::transmute_copy(&prendertargets), core::mem::transmute_copy(&pdepthstencil), core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn EndRenderPass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList4_Impl::EndRenderPass(this)
        }
        unsafe extern "system" fn InitializeMetaCommand<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmetacommand: *mut core::ffi::c_void, pinitializationparametersdata: *const core::ffi::c_void, initializationparametersdatasizeinbytes: usize) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList4_Impl::InitializeMetaCommand(this, windows_core::from_raw_borrowed(&pmetacommand), core::mem::transmute_copy(&pinitializationparametersdata), core::mem::transmute_copy(&initializationparametersdatasizeinbytes))
        }
        unsafe extern "system" fn ExecuteMetaCommand<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmetacommand: *mut core::ffi::c_void, pexecutionparametersdata: *const core::ffi::c_void, executionparametersdatasizeinbytes: usize) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList4_Impl::ExecuteMetaCommand(this, windows_core::from_raw_borrowed(&pmetacommand), core::mem::transmute_copy(&pexecutionparametersdata), core::mem::transmute_copy(&executionparametersdatasizeinbytes))
        }
        unsafe extern "system" fn BuildRaytracingAccelerationStructure<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, numpostbuildinfodescs: u32, ppostbuildinfodescs: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList4_Impl::BuildRaytracingAccelerationStructure(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&numpostbuildinfodescs), core::mem::transmute_copy(&ppostbuildinfodescs))
        }
        unsafe extern "system" fn EmitRaytracingAccelerationStructurePostbuildInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, numsourceaccelerationstructures: u32, psourceaccelerationstructuredata: *const u64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList4_Impl::EmitRaytracingAccelerationStructurePostbuildInfo(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&numsourceaccelerationstructures), core::mem::transmute_copy(&psourceaccelerationstructuredata))
        }
        unsafe extern "system" fn CopyRaytracingAccelerationStructure<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destaccelerationstructuredata: u64, sourceaccelerationstructuredata: u64, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList4_Impl::CopyRaytracingAccelerationStructure(this, core::mem::transmute_copy(&destaccelerationstructuredata), core::mem::transmute_copy(&sourceaccelerationstructuredata), core::mem::transmute_copy(&mode))
        }
        unsafe extern "system" fn SetPipelineState1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstateobject: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList4_Impl::SetPipelineState1(this, windows_core::from_raw_borrowed(&pstateobject))
        }
        unsafe extern "system" fn DispatchRays<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D12_DISPATCH_RAYS_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList4_Impl::DispatchRays(this, core::mem::transmute_copy(&pdesc))
        }
        Self {
            base__: ID3D12GraphicsCommandList3_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginRenderPass: BeginRenderPass::<Identity, Impl, OFFSET>,
            EndRenderPass: EndRenderPass::<Identity, Impl, OFFSET>,
            InitializeMetaCommand: InitializeMetaCommand::<Identity, Impl, OFFSET>,
            ExecuteMetaCommand: ExecuteMetaCommand::<Identity, Impl, OFFSET>,
            BuildRaytracingAccelerationStructure: BuildRaytracingAccelerationStructure::<Identity, Impl, OFFSET>,
            EmitRaytracingAccelerationStructurePostbuildInfo: EmitRaytracingAccelerationStructurePostbuildInfo::<Identity, Impl, OFFSET>,
            CopyRaytracingAccelerationStructure: CopyRaytracingAccelerationStructure::<Identity, Impl, OFFSET>,
            SetPipelineState1: SetPipelineState1::<Identity, Impl, OFFSET>,
            DispatchRays: DispatchRays::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList4 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList1 as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList2 as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList5_Impl: Sized + ID3D12GraphicsCommandList4_Impl {
    fn RSSetShadingRate(&self, baseshadingrate: D3D12_SHADING_RATE, combiners: *const D3D12_SHADING_RATE_COMBINER);
    fn RSSetShadingRateImage(&self, shadingrateimage: Option<&ID3D12Resource>);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D12GraphicsCommandList5 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList5_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList5_Vtbl {
        unsafe extern "system" fn RSSetShadingRate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseshadingrate: D3D12_SHADING_RATE, combiners: *const D3D12_SHADING_RATE_COMBINER) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList5_Impl::RSSetShadingRate(this, core::mem::transmute_copy(&baseshadingrate), core::mem::transmute_copy(&combiners))
        }
        unsafe extern "system" fn RSSetShadingRateImage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shadingrateimage: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList5_Impl::RSSetShadingRateImage(this, windows_core::from_raw_borrowed(&shadingrateimage))
        }
        Self {
            base__: ID3D12GraphicsCommandList4_Vtbl::new::<Identity, Impl, OFFSET>(),
            RSSetShadingRate: RSSetShadingRate::<Identity, Impl, OFFSET>,
            RSSetShadingRateImage: RSSetShadingRateImage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList5 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList1 as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList2 as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList3 as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList6_Impl: Sized + ID3D12GraphicsCommandList5_Impl {
    fn DispatchMesh(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D12GraphicsCommandList6 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList6_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList6_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList6_Vtbl {
        unsafe extern "system" fn DispatchMesh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList6_Impl::DispatchMesh(this, core::mem::transmute_copy(&threadgroupcountx), core::mem::transmute_copy(&threadgroupcounty), core::mem::transmute_copy(&threadgroupcountz))
        }
        Self { base__: ID3D12GraphicsCommandList5_Vtbl::new::<Identity, Impl, OFFSET>(), DispatchMesh: DispatchMesh::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList6 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList1 as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList2 as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList3 as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList4 as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList7_Impl: Sized + ID3D12GraphicsCommandList6_Impl {
    fn Barrier(&self, numbarriergroups: u32, pbarriergroups: *const D3D12_BARRIER_GROUP);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D12GraphicsCommandList7 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList7_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList7_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList7_Vtbl {
        unsafe extern "system" fn Barrier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbarriergroups: u32, pbarriergroups: *const D3D12_BARRIER_GROUP) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList7_Impl::Barrier(this, core::mem::transmute_copy(&numbarriergroups), core::mem::transmute_copy(&pbarriergroups))
        }
        Self { base__: ID3D12GraphicsCommandList6_Vtbl::new::<Identity, Impl, OFFSET>(), Barrier: Barrier::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList7 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12CommandList as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList1 as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList2 as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList3 as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList4 as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList5 as windows_core::Interface>::IID || iid == &<ID3D12GraphicsCommandList6 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList8_Impl: Sized + ID3D12GraphicsCommandList7_Impl {
    fn OMSetFrontAndBackStencilRef(&self, frontstencilref: u32, backstencilref: u32);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D12GraphicsCommandList8 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList8_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList8_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList8_Vtbl {
        unsafe extern "system" fn OMSetFrontAndBackStencilRef<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frontstencilref: u32, backstencilref: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList8_Impl::OMSetFrontAndBackStencilRef(this, core::mem::transmute_copy(&frontstencilref), core::mem::transmute_copy(&backstencilref))
        }
        Self {
            base__: ID3D12GraphicsCommandList7_Vtbl::new::<Identity, Impl, OFFSET>(),
            OMSetFrontAndBackStencilRef: OMSetFrontAndBackStencilRef::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList8 as windows_core::Interface>::IID
            || iid == &<ID3D12Object as windows_core::Interface>::IID
            || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID
            || iid == &<ID3D12CommandList as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList1 as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList2 as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList3 as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList4 as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList5 as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList6 as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList7 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D12GraphicsCommandList9_Impl: Sized + ID3D12GraphicsCommandList8_Impl {
    fn RSSetDepthBias(&self, depthbias: f32, depthbiasclamp: f32, slopescaleddepthbias: f32);
    fn IASetIndexBufferStripCutValue(&self, ibstripcutvalue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D12GraphicsCommandList9 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D12GraphicsCommandList9_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList9_Impl, const OFFSET: isize>() -> ID3D12GraphicsCommandList9_Vtbl {
        unsafe extern "system" fn RSSetDepthBias<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depthbias: f32, depthbiasclamp: f32, slopescaleddepthbias: f32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList9_Impl::RSSetDepthBias(this, core::mem::transmute_copy(&depthbias), core::mem::transmute_copy(&depthbiasclamp), core::mem::transmute_copy(&slopescaleddepthbias))
        }
        unsafe extern "system" fn IASetIndexBufferStripCutValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12GraphicsCommandList9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ibstripcutvalue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12GraphicsCommandList9_Impl::IASetIndexBufferStripCutValue(this, core::mem::transmute_copy(&ibstripcutvalue))
        }
        Self {
            base__: ID3D12GraphicsCommandList8_Vtbl::new::<Identity, Impl, OFFSET>(),
            RSSetDepthBias: RSSetDepthBias::<Identity, Impl, OFFSET>,
            IASetIndexBufferStripCutValue: IASetIndexBufferStripCutValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12GraphicsCommandList9 as windows_core::Interface>::IID
            || iid == &<ID3D12Object as windows_core::Interface>::IID
            || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID
            || iid == &<ID3D12CommandList as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList1 as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList2 as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList3 as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList4 as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList5 as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList6 as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList7 as windows_core::Interface>::IID
            || iid == &<ID3D12GraphicsCommandList8 as windows_core::Interface>::IID
    }
}
pub trait ID3D12Heap_Impl: Sized + ID3D12Pageable_Impl {
    fn GetDesc(&self) -> D3D12_HEAP_DESC;
}
impl windows_core::RuntimeName for ID3D12Heap {}
impl ID3D12Heap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Heap_Impl, const OFFSET: isize>() -> ID3D12Heap_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Heap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_HEAP_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12Heap_Impl::GetDesc(this)
        }
        Self { base__: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Heap as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Pageable as windows_core::Interface>::IID
    }
}
pub trait ID3D12Heap1_Impl: Sized + ID3D12Heap_Impl {
    fn GetProtectedResourceSession(&self, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D12Heap1 {}
impl ID3D12Heap1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Heap1_Impl, const OFFSET: isize>() -> ID3D12Heap1_Vtbl {
        unsafe extern "system" fn GetProtectedResourceSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Heap1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Heap1_Impl::GetProtectedResourceSession(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppprotectedsession)).into()
        }
        Self { base__: ID3D12Heap_Vtbl::new::<Identity, Impl, OFFSET>(), GetProtectedResourceSession: GetProtectedResourceSession::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Heap1 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Pageable as windows_core::Interface>::IID || iid == &<ID3D12Heap as windows_core::Interface>::IID
    }
}
pub trait ID3D12InfoQueue_Impl: Sized {
    fn SetMessageCountLimit(&self, messagecountlimit: u64) -> windows_core::Result<()>;
    fn ClearStoredMessages(&self);
    fn GetMessage(&self, messageindex: u64, pmessage: *mut D3D12_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::Result<()>;
    fn GetNumMessagesAllowedByStorageFilter(&self) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&self) -> u64;
    fn GetNumStoredMessages(&self) -> u64;
    fn GetNumStoredMessagesAllowedByRetrievalFilter(&self) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&self) -> u64;
    fn GetMessageCountLimit(&self) -> u64;
    fn AddStorageFilterEntries(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetStorageFilter(&self, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearStorageFilter(&self);
    fn PushEmptyStorageFilter(&self) -> windows_core::Result<()>;
    fn PushCopyOfStorageFilter(&self) -> windows_core::Result<()>;
    fn PushStorageFilter(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopStorageFilter(&self);
    fn GetStorageFilterStackSize(&self) -> u32;
    fn AddRetrievalFilterEntries(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetRetrievalFilter(&self, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearRetrievalFilter(&self);
    fn PushEmptyRetrievalFilter(&self) -> windows_core::Result<()>;
    fn PushCopyOfRetrievalFilter(&self) -> windows_core::Result<()>;
    fn PushRetrievalFilter(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopRetrievalFilter(&self);
    fn GetRetrievalFilterStackSize(&self) -> u32;
    fn AddMessage(&self, category: D3D12_MESSAGE_CATEGORY, severity: D3D12_MESSAGE_SEVERITY, id: D3D12_MESSAGE_ID, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn AddApplicationMessage(&self, severity: D3D12_MESSAGE_SEVERITY, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn SetBreakOnCategory(&self, category: D3D12_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnSeverity(&self, severity: D3D12_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnID(&self, id: D3D12_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetBreakOnCategory(&self, category: D3D12_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(&self, severity: D3D12_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(&self, id: D3D12_MESSAGE_ID) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(&self, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for ID3D12InfoQueue {}
impl ID3D12InfoQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>() -> ID3D12InfoQueue_Vtbl {
        unsafe extern "system" fn SetMessageCountLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagecountlimit: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::SetMessageCountLimit(this, core::mem::transmute_copy(&messagecountlimit)).into()
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::ClearStoredMessages(this)
        }
        unsafe extern "system" fn GetMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messageindex: u64, pmessage: *mut D3D12_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::GetMessage(this, core::mem::transmute_copy(&messageindex), core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&pmessagebytelength)).into()
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::GetNumMessagesAllowedByStorageFilter(this)
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::GetNumMessagesDeniedByStorageFilter(this)
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::GetNumStoredMessages(this)
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::GetNumStoredMessagesAllowedByRetrievalFilter(this)
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::GetNumMessagesDiscardedByMessageCountLimit(this)
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::GetMessageCountLimit(this)
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::AddStorageFilterEntries(this, core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::GetStorageFilter(this, core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::ClearStorageFilter(this)
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::PushEmptyStorageFilter(this).into()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::PushCopyOfStorageFilter(this).into()
        }
        unsafe extern "system" fn PushStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::PushStorageFilter(this, core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::PopStorageFilter(this)
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::GetStorageFilterStackSize(this)
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::AddRetrievalFilterEntries(this, core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut D3D12_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::GetRetrievalFilter(this, core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::ClearRetrievalFilter(this)
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::PushEmptyRetrievalFilter(this).into()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::PushCopyOfRetrievalFilter(this).into()
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::PushRetrievalFilter(this, core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::PopRetrievalFilter(this)
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::GetRetrievalFilterStackSize(this)
        }
        unsafe extern "system" fn AddMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY, severity: D3D12_MESSAGE_SEVERITY, id: D3D12_MESSAGE_ID, pdescription: windows_core::PCSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::AddMessage(this, core::mem::transmute_copy(&category), core::mem::transmute_copy(&severity), core::mem::transmute_copy(&id), core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY, pdescription: windows_core::PCSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::AddApplicationMessage(this, core::mem::transmute_copy(&severity), core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::SetBreakOnCategory(this, core::mem::transmute_copy(&category), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::SetBreakOnSeverity(this, core::mem::transmute_copy(&severity), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: D3D12_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::SetBreakOnID(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: D3D12_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::GetBreakOnCategory(this, core::mem::transmute_copy(&category))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: D3D12_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::GetBreakOnSeverity(this, core::mem::transmute_copy(&severity))
        }
        unsafe extern "system" fn GetBreakOnID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: D3D12_MESSAGE_ID) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::GetBreakOnID(this, core::mem::transmute_copy(&id))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::SetMuteDebugOutput(this, core::mem::transmute_copy(&bmute))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue_Impl::GetMuteDebugOutput(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Identity, Impl, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, Impl, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter: GetNumStoredMessagesAllowedByRetrievalFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, Impl, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, Impl, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, Impl, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, Impl, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, Impl, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, Impl, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, Impl, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, Impl, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, Impl, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, Impl, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, Impl, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, Impl, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, Impl, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, Impl, OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Identity, Impl, OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Identity, Impl, OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Identity, Impl, OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Identity, Impl, OFFSET>,
            AddMessage: AddMessage::<Identity, Impl, OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Identity, Impl, OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Identity, Impl, OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Identity, Impl, OFFSET>,
            SetBreakOnID: SetBreakOnID::<Identity, Impl, OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Identity, Impl, OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Identity, Impl, OFFSET>,
            GetBreakOnID: GetBreakOnID::<Identity, Impl, OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, Impl, OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12InfoQueue as windows_core::Interface>::IID
    }
}
pub trait ID3D12InfoQueue1_Impl: Sized + ID3D12InfoQueue_Impl {
    fn RegisterMessageCallback(&self, callbackfunc: D3D12MessageFunc, callbackfilterflags: D3D12_MESSAGE_CALLBACK_FLAGS, pcontext: *mut core::ffi::c_void, pcallbackcookie: *mut u32) -> windows_core::Result<()>;
    fn UnregisterMessageCallback(&self, callbackcookie: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D12InfoQueue1 {}
impl ID3D12InfoQueue1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue1_Impl, const OFFSET: isize>() -> ID3D12InfoQueue1_Vtbl {
        unsafe extern "system" fn RegisterMessageCallback<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callbackfunc: D3D12MessageFunc, callbackfilterflags: D3D12_MESSAGE_CALLBACK_FLAGS, pcontext: *mut core::ffi::c_void, pcallbackcookie: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue1_Impl::RegisterMessageCallback(this, core::mem::transmute_copy(&callbackfunc), core::mem::transmute_copy(&callbackfilterflags), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pcallbackcookie)).into()
        }
        unsafe extern "system" fn UnregisterMessageCallback<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12InfoQueue1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callbackcookie: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12InfoQueue1_Impl::UnregisterMessageCallback(this, core::mem::transmute_copy(&callbackcookie)).into()
        }
        Self {
            base__: ID3D12InfoQueue_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegisterMessageCallback: RegisterMessageCallback::<Identity, Impl, OFFSET>,
            UnregisterMessageCallback: UnregisterMessageCallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12InfoQueue1 as windows_core::Interface>::IID || iid == &<ID3D12InfoQueue as windows_core::Interface>::IID
    }
}
pub trait ID3D12LibraryReflection_Impl: Sized {
    fn GetDesc(&self) -> windows_core::Result<D3D12_LIBRARY_DESC>;
    fn GetFunctionByIndex(&self, functionindex: i32) -> Option<ID3D12FunctionReflection>;
}
impl windows_core::RuntimeName for ID3D12LibraryReflection {}
impl ID3D12LibraryReflection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12LibraryReflection_Impl, const OFFSET: isize>() -> ID3D12LibraryReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12LibraryReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D12_LIBRARY_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D12LibraryReflection_Impl::GetDesc(this) {
                Ok(ok__) => {
                    core::ptr::write(pdesc, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionByIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12LibraryReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionindex: i32) -> Option<ID3D12FunctionReflection> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12LibraryReflection_Impl::GetFunctionByIndex(this, core::mem::transmute_copy(&functionindex))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetFunctionByIndex: GetFunctionByIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12LibraryReflection as windows_core::Interface>::IID
    }
}
pub trait ID3D12LifetimeOwner_Impl: Sized {
    fn LifetimeStateUpdated(&self, newstate: D3D12_LIFETIME_STATE);
}
impl windows_core::RuntimeName for ID3D12LifetimeOwner {}
impl ID3D12LifetimeOwner_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12LifetimeOwner_Impl, const OFFSET: isize>() -> ID3D12LifetimeOwner_Vtbl {
        unsafe extern "system" fn LifetimeStateUpdated<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12LifetimeOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstate: D3D12_LIFETIME_STATE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12LifetimeOwner_Impl::LifetimeStateUpdated(this, core::mem::transmute_copy(&newstate))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LifetimeStateUpdated: LifetimeStateUpdated::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12LifetimeOwner as windows_core::Interface>::IID
    }
}
pub trait ID3D12LifetimeTracker_Impl: Sized + ID3D12DeviceChild_Impl {
    fn DestroyOwnedObject(&self, pobject: Option<&ID3D12DeviceChild>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D12LifetimeTracker {}
impl ID3D12LifetimeTracker_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12LifetimeTracker_Impl, const OFFSET: isize>() -> ID3D12LifetimeTracker_Vtbl {
        unsafe extern "system" fn DestroyOwnedObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12LifetimeTracker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12LifetimeTracker_Impl::DestroyOwnedObject(this, windows_core::from_raw_borrowed(&pobject)).into()
        }
        Self { base__: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), DestroyOwnedObject: DestroyOwnedObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12LifetimeTracker as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D12ManualWriteTrackingResource_Impl: Sized {
    fn TrackWrite(&self, subresource: u32, pwrittenrange: *const D3D12_RANGE);
}
impl windows_core::RuntimeName for ID3D12ManualWriteTrackingResource {}
impl ID3D12ManualWriteTrackingResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ManualWriteTrackingResource_Impl, const OFFSET: isize>() -> ID3D12ManualWriteTrackingResource_Vtbl {
        unsafe extern "system" fn TrackWrite<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ManualWriteTrackingResource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32, pwrittenrange: *const D3D12_RANGE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ManualWriteTrackingResource_Impl::TrackWrite(this, core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&pwrittenrange))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TrackWrite: TrackWrite::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12ManualWriteTrackingResource as windows_core::Interface>::IID
    }
}
pub trait ID3D12MetaCommand_Impl: Sized + ID3D12Pageable_Impl {
    fn GetRequiredParameterResourceSize(&self, stage: D3D12_META_COMMAND_PARAMETER_STAGE, parameterindex: u32) -> u64;
}
impl windows_core::RuntimeName for ID3D12MetaCommand {}
impl ID3D12MetaCommand_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12MetaCommand_Impl, const OFFSET: isize>() -> ID3D12MetaCommand_Vtbl {
        unsafe extern "system" fn GetRequiredParameterResourceSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12MetaCommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: D3D12_META_COMMAND_PARAMETER_STAGE, parameterindex: u32) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12MetaCommand_Impl::GetRequiredParameterResourceSize(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&parameterindex))
        }
        Self {
            base__: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetRequiredParameterResourceSize: GetRequiredParameterResourceSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12MetaCommand as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Pageable as windows_core::Interface>::IID
    }
}
pub trait ID3D12Object_Impl: Sized {
    fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const windows_core::GUID, pdata: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetName(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D12Object {}
impl ID3D12Object_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Object_Impl, const OFFSET: isize>() -> ID3D12Object_Vtbl {
        unsafe extern "system" fn GetPrivateData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Object_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Object_Impl::GetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Object_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Object_Impl::SetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Object_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Object_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&guid), windows_core::from_raw_borrowed(&pdata)).into()
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Object_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Object_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Object as windows_core::Interface>::IID
    }
}
pub trait ID3D12Pageable_Impl: Sized + ID3D12DeviceChild_Impl {}
impl windows_core::RuntimeName for ID3D12Pageable {}
impl ID3D12Pageable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Pageable_Impl, const OFFSET: isize>() -> ID3D12Pageable_Vtbl {
        Self { base__: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Pageable as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D12PipelineLibrary_Impl: Sized + ID3D12DeviceChild_Impl {
    fn StorePipeline(&self, pname: &windows_core::PCWSTR, ppipeline: Option<&ID3D12PipelineState>) -> windows_core::Result<()>;
    fn LoadGraphicsPipeline(&self, pname: &windows_core::PCWSTR, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const windows_core::GUID, pppipelinestate: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn LoadComputePipeline(&self, pname: &windows_core::PCWSTR, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const windows_core::GUID, pppipelinestate: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetSerializedSize(&self) -> usize;
    fn Serialize(&self, pdata: *mut core::ffi::c_void, datasizeinbytes: usize) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D12PipelineLibrary {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D12PipelineLibrary_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>() -> ID3D12PipelineLibrary_Vtbl {
        unsafe extern "system" fn StorePipeline<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCWSTR, ppipeline: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12PipelineLibrary_Impl::StorePipeline(this, core::mem::transmute(&pname), windows_core::from_raw_borrowed(&ppipeline)).into()
        }
        unsafe extern "system" fn LoadGraphicsPipeline<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCWSTR, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: *const windows_core::GUID, pppipelinestate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12PipelineLibrary_Impl::LoadGraphicsPipeline(this, core::mem::transmute(&pname), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pppipelinestate)).into()
        }
        unsafe extern "system" fn LoadComputePipeline<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCWSTR, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: *const windows_core::GUID, pppipelinestate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12PipelineLibrary_Impl::LoadComputePipeline(this, core::mem::transmute(&pname), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pppipelinestate)).into()
        }
        unsafe extern "system" fn GetSerializedSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12PipelineLibrary_Impl::GetSerializedSize(this)
        }
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12PipelineLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, datasizeinbytes: usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12PipelineLibrary_Impl::Serialize(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&datasizeinbytes)).into()
        }
        Self {
            base__: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            StorePipeline: StorePipeline::<Identity, Impl, OFFSET>,
            LoadGraphicsPipeline: LoadGraphicsPipeline::<Identity, Impl, OFFSET>,
            LoadComputePipeline: LoadComputePipeline::<Identity, Impl, OFFSET>,
            GetSerializedSize: GetSerializedSize::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12PipelineLibrary as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D12PipelineLibrary1_Impl: Sized + ID3D12PipelineLibrary_Impl {
    fn LoadPipeline(&self, pname: &windows_core::PCWSTR, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const windows_core::GUID, pppipelinestate: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D12PipelineLibrary1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D12PipelineLibrary1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12PipelineLibrary1_Impl, const OFFSET: isize>() -> ID3D12PipelineLibrary1_Vtbl {
        unsafe extern "system" fn LoadPipeline<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12PipelineLibrary1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCWSTR, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC, riid: *const windows_core::GUID, pppipelinestate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12PipelineLibrary1_Impl::LoadPipeline(this, core::mem::transmute(&pname), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pppipelinestate)).into()
        }
        Self { base__: ID3D12PipelineLibrary_Vtbl::new::<Identity, Impl, OFFSET>(), LoadPipeline: LoadPipeline::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12PipelineLibrary1 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12PipelineLibrary as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D12PipelineState_Impl: Sized + ID3D12Pageable_Impl {
    fn GetCachedBlob(&self) -> windows_core::Result<super::Direct3D::ID3DBlob>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::RuntimeName for ID3D12PipelineState {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12PipelineState_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12PipelineState_Impl, const OFFSET: isize>() -> ID3D12PipelineState_Vtbl {
        unsafe extern "system" fn GetCachedBlob<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12PipelineState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppblob: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D12PipelineState_Impl::GetCachedBlob(this) {
                Ok(ok__) => {
                    core::ptr::write(ppblob, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(), GetCachedBlob: GetCachedBlob::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12PipelineState as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Pageable as windows_core::Interface>::IID
    }
}
pub trait ID3D12ProtectedResourceSession_Impl: Sized + ID3D12ProtectedSession_Impl {
    fn GetDesc(&self) -> D3D12_PROTECTED_RESOURCE_SESSION_DESC;
}
impl windows_core::RuntimeName for ID3D12ProtectedResourceSession {}
impl ID3D12ProtectedResourceSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ProtectedResourceSession_Impl, const OFFSET: isize>() -> ID3D12ProtectedResourceSession_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ProtectedResourceSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_PROTECTED_RESOURCE_SESSION_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12ProtectedResourceSession_Impl::GetDesc(this)
        }
        Self { base__: ID3D12ProtectedSession_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12ProtectedResourceSession as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12ProtectedSession as windows_core::Interface>::IID
    }
}
pub trait ID3D12ProtectedResourceSession1_Impl: Sized + ID3D12ProtectedResourceSession_Impl {
    fn GetDesc1(&self) -> D3D12_PROTECTED_RESOURCE_SESSION_DESC1;
}
impl windows_core::RuntimeName for ID3D12ProtectedResourceSession1 {}
impl ID3D12ProtectedResourceSession1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ProtectedResourceSession1_Impl, const OFFSET: isize>() -> ID3D12ProtectedResourceSession1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ProtectedResourceSession1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_PROTECTED_RESOURCE_SESSION_DESC1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12ProtectedResourceSession1_Impl::GetDesc1(this)
        }
        Self { base__: ID3D12ProtectedResourceSession_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12ProtectedResourceSession1 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12ProtectedSession as windows_core::Interface>::IID || iid == &<ID3D12ProtectedResourceSession as windows_core::Interface>::IID
    }
}
pub trait ID3D12ProtectedSession_Impl: Sized + ID3D12DeviceChild_Impl {
    fn GetStatusFence(&self, riid: *const windows_core::GUID, ppfence: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetSessionStatus(&self) -> D3D12_PROTECTED_SESSION_STATUS;
}
impl windows_core::RuntimeName for ID3D12ProtectedSession {}
impl ID3D12ProtectedSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ProtectedSession_Impl, const OFFSET: isize>() -> ID3D12ProtectedSession_Vtbl {
        unsafe extern "system" fn GetStatusFence<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ProtectedSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppfence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ProtectedSession_Impl::GetStatusFence(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppfence)).into()
        }
        unsafe extern "system" fn GetSessionStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ProtectedSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D12_PROTECTED_SESSION_STATUS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ProtectedSession_Impl::GetSessionStatus(this)
        }
        Self {
            base__: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStatusFence: GetStatusFence::<Identity, Impl, OFFSET>,
            GetSessionStatus: GetSessionStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12ProtectedSession as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D12QueryHeap_Impl: Sized + ID3D12Pageable_Impl {}
impl windows_core::RuntimeName for ID3D12QueryHeap {}
impl ID3D12QueryHeap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12QueryHeap_Impl, const OFFSET: isize>() -> ID3D12QueryHeap_Vtbl {
        Self { base__: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12QueryHeap as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Pageable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D12Resource_Impl: Sized + ID3D12Pageable_Impl {
    fn Map(&self, subresource: u32, preadrange: *const D3D12_RANGE, ppdata: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Unmap(&self, subresource: u32, pwrittenrange: *const D3D12_RANGE);
    fn GetDesc(&self) -> D3D12_RESOURCE_DESC;
    fn GetGPUVirtualAddress(&self) -> u64;
    fn WriteToSubresource(&self, dstsubresource: u32, pdstbox: *const D3D12_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) -> windows_core::Result<()>;
    fn ReadFromSubresource(&self, pdstdata: *mut core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, srcsubresource: u32, psrcbox: *const D3D12_BOX) -> windows_core::Result<()>;
    fn GetHeapProperties(&self, pheapproperties: *mut D3D12_HEAP_PROPERTIES, pheapflags: *mut D3D12_HEAP_FLAGS) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D12Resource {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D12Resource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: isize>() -> ID3D12Resource_Vtbl {
        unsafe extern "system" fn Map<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32, preadrange: *const D3D12_RANGE, ppdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Resource_Impl::Map(this, core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&preadrange), core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn Unmap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32, pwrittenrange: *const D3D12_RANGE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Resource_Impl::Unmap(this, core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&pwrittenrange))
        }
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_RESOURCE_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12Resource_Impl::GetDesc(this)
        }
        unsafe extern "system" fn GetGPUVirtualAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Resource_Impl::GetGPUVirtualAddress(this)
        }
        unsafe extern "system" fn WriteToSubresource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D12_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Resource_Impl::WriteToSubresource(this, core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&pdstbox), core::mem::transmute_copy(&psrcdata), core::mem::transmute_copy(&srcrowpitch), core::mem::transmute_copy(&srcdepthpitch)).into()
        }
        unsafe extern "system" fn ReadFromSubresource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstdata: *mut core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, srcsubresource: u32, psrcbox: *const D3D12_BOX) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Resource_Impl::ReadFromSubresource(this, core::mem::transmute_copy(&pdstdata), core::mem::transmute_copy(&dstrowpitch), core::mem::transmute_copy(&dstdepthpitch), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&psrcbox)).into()
        }
        unsafe extern "system" fn GetHeapProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheapproperties: *mut D3D12_HEAP_PROPERTIES, pheapflags: *mut D3D12_HEAP_FLAGS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Resource_Impl::GetHeapProperties(this, core::mem::transmute_copy(&pheapproperties), core::mem::transmute_copy(&pheapflags)).into()
        }
        Self {
            base__: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>(),
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetGPUVirtualAddress: GetGPUVirtualAddress::<Identity, Impl, OFFSET>,
            WriteToSubresource: WriteToSubresource::<Identity, Impl, OFFSET>,
            ReadFromSubresource: ReadFromSubresource::<Identity, Impl, OFFSET>,
            GetHeapProperties: GetHeapProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Resource as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Pageable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D12Resource1_Impl: Sized + ID3D12Resource_Impl {
    fn GetProtectedResourceSession(&self, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D12Resource1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D12Resource1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Resource1_Impl, const OFFSET: isize>() -> ID3D12Resource1_Vtbl {
        unsafe extern "system" fn GetProtectedResourceSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Resource1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppprotectedsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Resource1_Impl::GetProtectedResourceSession(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppprotectedsession)).into()
        }
        Self { base__: ID3D12Resource_Vtbl::new::<Identity, Impl, OFFSET>(), GetProtectedResourceSession: GetProtectedResourceSession::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Resource1 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Pageable as windows_core::Interface>::IID || iid == &<ID3D12Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D12Resource2_Impl: Sized + ID3D12Resource1_Impl {
    fn GetDesc1(&self) -> D3D12_RESOURCE_DESC1;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D12Resource2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D12Resource2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Resource2_Impl, const OFFSET: isize>() -> ID3D12Resource2_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Resource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_RESOURCE_DESC1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12Resource2_Impl::GetDesc1(this)
        }
        Self { base__: ID3D12Resource1_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Resource2 as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Pageable as windows_core::Interface>::IID || iid == &<ID3D12Resource as windows_core::Interface>::IID || iid == &<ID3D12Resource1 as windows_core::Interface>::IID
    }
}
pub trait ID3D12RootSignature_Impl: Sized + ID3D12DeviceChild_Impl {}
impl windows_core::RuntimeName for ID3D12RootSignature {}
impl ID3D12RootSignature_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12RootSignature_Impl, const OFFSET: isize>() -> ID3D12RootSignature_Vtbl {
        Self { base__: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12RootSignature as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D12RootSignatureDeserializer_Impl: Sized {
    fn GetRootSignatureDesc(&self) -> *mut D3D12_ROOT_SIGNATURE_DESC;
}
impl windows_core::RuntimeName for ID3D12RootSignatureDeserializer {}
impl ID3D12RootSignatureDeserializer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12RootSignatureDeserializer_Impl, const OFFSET: isize>() -> ID3D12RootSignatureDeserializer_Vtbl {
        unsafe extern "system" fn GetRootSignatureDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12RootSignatureDeserializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut D3D12_ROOT_SIGNATURE_DESC {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12RootSignatureDeserializer_Impl::GetRootSignatureDesc(this)
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRootSignatureDesc: GetRootSignatureDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12RootSignatureDeserializer as windows_core::Interface>::IID
    }
}
pub trait ID3D12SDKConfiguration_Impl: Sized {
    fn SetSDKVersion(&self, sdkversion: u32, sdkpath: &windows_core::PCSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D12SDKConfiguration {}
impl ID3D12SDKConfiguration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12SDKConfiguration_Impl, const OFFSET: isize>() -> ID3D12SDKConfiguration_Vtbl {
        unsafe extern "system" fn SetSDKVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12SDKConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sdkversion: u32, sdkpath: windows_core::PCSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12SDKConfiguration_Impl::SetSDKVersion(this, core::mem::transmute_copy(&sdkversion), core::mem::transmute(&sdkpath)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetSDKVersion: SetSDKVersion::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12SDKConfiguration as windows_core::Interface>::IID
    }
}
pub trait ID3D12SDKConfiguration1_Impl: Sized + ID3D12SDKConfiguration_Impl {
    fn CreateDeviceFactory(&self, sdkversion: u32, sdkpath: &windows_core::PCSTR, riid: *const windows_core::GUID, ppvfactory: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FreeUnusedSDKs(&self);
}
impl windows_core::RuntimeName for ID3D12SDKConfiguration1 {}
impl ID3D12SDKConfiguration1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12SDKConfiguration1_Impl, const OFFSET: isize>() -> ID3D12SDKConfiguration1_Vtbl {
        unsafe extern "system" fn CreateDeviceFactory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12SDKConfiguration1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sdkversion: u32, sdkpath: windows_core::PCSTR, riid: *const windows_core::GUID, ppvfactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12SDKConfiguration1_Impl::CreateDeviceFactory(this, core::mem::transmute_copy(&sdkversion), core::mem::transmute(&sdkpath), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvfactory)).into()
        }
        unsafe extern "system" fn FreeUnusedSDKs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12SDKConfiguration1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12SDKConfiguration1_Impl::FreeUnusedSDKs(this)
        }
        Self {
            base__: ID3D12SDKConfiguration_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateDeviceFactory: CreateDeviceFactory::<Identity, Impl, OFFSET>,
            FreeUnusedSDKs: FreeUnusedSDKs::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12SDKConfiguration1 as windows_core::Interface>::IID || iid == &<ID3D12SDKConfiguration as windows_core::Interface>::IID
    }
}
pub trait ID3D12ShaderCacheSession_Impl: Sized + ID3D12DeviceChild_Impl {
    fn FindValue(&self, pkey: *const core::ffi::c_void, keysize: u32, pvalue: *mut core::ffi::c_void, pvaluesize: *mut u32) -> windows_core::Result<()>;
    fn StoreValue(&self, pkey: *const core::ffi::c_void, keysize: u32, pvalue: *const core::ffi::c_void, valuesize: u32) -> windows_core::Result<()>;
    fn SetDeleteOnDestroy(&self);
    fn GetDesc(&self) -> D3D12_SHADER_CACHE_SESSION_DESC;
}
impl windows_core::RuntimeName for ID3D12ShaderCacheSession {}
impl ID3D12ShaderCacheSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: isize>() -> ID3D12ShaderCacheSession_Vtbl {
        unsafe extern "system" fn FindValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkey: *const core::ffi::c_void, keysize: u32, pvalue: *mut core::ffi::c_void, pvaluesize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderCacheSession_Impl::FindValue(this, core::mem::transmute_copy(&pkey), core::mem::transmute_copy(&keysize), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&pvaluesize)).into()
        }
        unsafe extern "system" fn StoreValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkey: *const core::ffi::c_void, keysize: u32, pvalue: *const core::ffi::c_void, valuesize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderCacheSession_Impl::StoreValue(this, core::mem::transmute_copy(&pkey), core::mem::transmute_copy(&keysize), core::mem::transmute_copy(&pvalue), core::mem::transmute_copy(&valuesize)).into()
        }
        unsafe extern "system" fn SetDeleteOnDestroy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderCacheSession_Impl::SetDeleteOnDestroy(this)
        }
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderCacheSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D3D12_SHADER_CACHE_SESSION_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12ShaderCacheSession_Impl::GetDesc(this)
        }
        Self {
            base__: ID3D12DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            FindValue: FindValue::<Identity, Impl, OFFSET>,
            StoreValue: StoreValue::<Identity, Impl, OFFSET>,
            SetDeleteOnDestroy: SetDeleteOnDestroy::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12ShaderCacheSession as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D12ShaderReflection_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D12_SHADER_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetPatchConstantParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &windows_core::PCSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetMovInstructionCount(&self) -> u32;
    fn GetMovcInstructionCount(&self) -> u32;
    fn GetConversionInstructionCount(&self) -> u32;
    fn GetBitwiseInstructionCount(&self) -> u32;
    fn GetGSInputPrimitive(&self) -> super::Direct3D::D3D_PRIMITIVE;
    fn IsSampleFrequencyShader(&self) -> super::super::Foundation::BOOL;
    fn GetNumInterfaceSlots(&self) -> u32;
    fn GetMinFeatureLevel(&self) -> windows_core::Result<super::Direct3D::D3D_FEATURE_LEVEL>;
    fn GetThreadGroupSize(&self, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32;
    fn GetRequiresFlags(&self) -> u64;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::RuntimeName for ID3D12ShaderReflection {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12ShaderReflection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>() -> ID3D12ShaderReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D12_SHADER_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetResourceBindingDesc(this, core::mem::transmute_copy(&resourceindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetInputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetOutputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetPatchConstantParameterDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D12_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetPatchConstantParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetVariableByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, pdesc: *mut D3D12_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetResourceBindingDescByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetMovInstructionCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetMovInstructionCount(this)
        }
        unsafe extern "system" fn GetMovcInstructionCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetMovcInstructionCount(this)
        }
        unsafe extern "system" fn GetConversionInstructionCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetConversionInstructionCount(this)
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetBitwiseInstructionCount(this)
        }
        unsafe extern "system" fn GetGSInputPrimitive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::Direct3D::D3D_PRIMITIVE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetGSInputPrimitive(this)
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::IsSampleFrequencyShader(this)
        }
        unsafe extern "system" fn GetNumInterfaceSlots<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetNumInterfaceSlots(this)
        }
        unsafe extern "system" fn GetMinFeatureLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D12ShaderReflection_Impl::GetMinFeatureLevel(this) {
                Ok(ok__) => {
                    core::ptr::write(plevel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadGroupSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetThreadGroupSize(this, core::mem::transmute_copy(&psizex), core::mem::transmute_copy(&psizey), core::mem::transmute_copy(&psizez))
        }
        unsafe extern "system" fn GetRequiresFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12ShaderReflection_Impl::GetRequiresFlags(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, Impl, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, Impl, OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Identity, Impl, OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Identity, Impl, OFFSET>,
            GetPatchConstantParameterDesc: GetPatchConstantParameterDesc::<Identity, Impl, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Identity, Impl, OFFSET>,
            GetMovInstructionCount: GetMovInstructionCount::<Identity, Impl, OFFSET>,
            GetMovcInstructionCount: GetMovcInstructionCount::<Identity, Impl, OFFSET>,
            GetConversionInstructionCount: GetConversionInstructionCount::<Identity, Impl, OFFSET>,
            GetBitwiseInstructionCount: GetBitwiseInstructionCount::<Identity, Impl, OFFSET>,
            GetGSInputPrimitive: GetGSInputPrimitive::<Identity, Impl, OFFSET>,
            IsSampleFrequencyShader: IsSampleFrequencyShader::<Identity, Impl, OFFSET>,
            GetNumInterfaceSlots: GetNumInterfaceSlots::<Identity, Impl, OFFSET>,
            GetMinFeatureLevel: GetMinFeatureLevel::<Identity, Impl, OFFSET>,
            GetThreadGroupSize: GetThreadGroupSize::<Identity, Impl, OFFSET>,
            GetRequiresFlags: GetRequiresFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12ShaderReflection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D12ShaderReflectionConstantBuffer_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D12_SHADER_BUFFER_DESC) -> windows_core::Result<()>;
    fn GetVariableByIndex(&self, index: u32) -> Option<ID3D12ShaderReflectionVariable>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12ShaderReflectionConstantBuffer_Vtbl {
    pub const fn new<Impl: ID3D12ShaderReflectionConstantBuffer_Impl>() -> ID3D12ShaderReflectionConstantBuffer_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D12_SHADER_BUFFER_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionConstantBuffer_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D12ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionConstantBuffer_Impl::GetVariableByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D12ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D12ShaderReflectionVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionConstantBuffer_Impl::GetVariableByName(this, core::mem::transmute(&name))
        }
        Self { GetDesc: GetDesc::<Impl>, GetVariableByIndex: GetVariableByIndex::<Impl>, GetVariableByName: GetVariableByName::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D12ShaderReflectionConstantBuffer_ImplVtbl<T: ID3D12ShaderReflectionConstantBuffer_Impl>(std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D12ShaderReflectionConstantBuffer_Impl> ID3D12ShaderReflectionConstantBuffer_ImplVtbl<T> {
    const VTABLE: ID3D12ShaderReflectionConstantBuffer_Vtbl = ID3D12ShaderReflectionConstantBuffer_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12ShaderReflectionConstantBuffer {
    pub fn new<'a, T: ID3D12ShaderReflectionConstantBuffer_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D12ShaderReflectionConstantBuffer_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D12ShaderReflectionType_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D12_SHADER_TYPE_DESC) -> windows_core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D12ShaderReflectionType>;
    fn GetMemberTypeByName(&self, name: &windows_core::PCSTR) -> Option<ID3D12ShaderReflectionType>;
    fn GetMemberTypeName(&self, index: u32) -> windows_core::PCSTR;
    fn IsEqual(&self, ptype: Option<&ID3D12ShaderReflectionType>) -> windows_core::Result<()>;
    fn GetSubType(&self) -> Option<ID3D12ShaderReflectionType>;
    fn GetBaseClass(&self) -> Option<ID3D12ShaderReflectionType>;
    fn GetNumInterfaces(&self) -> u32;
    fn GetInterfaceByIndex(&self, uindex: u32) -> Option<ID3D12ShaderReflectionType>;
    fn IsOfType(&self, ptype: Option<&ID3D12ShaderReflectionType>) -> windows_core::Result<()>;
    fn ImplementsInterface(&self, pbase: Option<&ID3D12ShaderReflectionType>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12ShaderReflectionType_Vtbl {
    pub const fn new<Impl: ID3D12ShaderReflectionType_Impl>() -> ID3D12ShaderReflectionType_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D12_SHADER_TYPE_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionType_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionType_Impl::GetMemberTypeByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionType_Impl::GetMemberTypeByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberTypeName<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, index: u32) -> windows_core::PCSTR {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionType_Impl::GetMemberTypeName(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn IsEqual<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, ptype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionType_Impl::IsEqual(this, windows_core::from_raw_borrowed(&ptype)).into()
        }
        unsafe extern "system" fn GetSubType<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionType_Impl::GetSubType(this)
        }
        unsafe extern "system" fn GetBaseClass<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionType_Impl::GetBaseClass(this)
        }
        unsafe extern "system" fn GetNumInterfaces<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionType_Impl::GetNumInterfaces(this)
        }
        unsafe extern "system" fn GetInterfaceByIndex<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, uindex: u32) -> Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionType_Impl::GetInterfaceByIndex(this, core::mem::transmute_copy(&uindex))
        }
        unsafe extern "system" fn IsOfType<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, ptype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionType_Impl::IsOfType(this, windows_core::from_raw_borrowed(&ptype)).into()
        }
        unsafe extern "system" fn ImplementsInterface<Impl: ID3D12ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, pbase: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionType_Impl::ImplementsInterface(this, windows_core::from_raw_borrowed(&pbase)).into()
        }
        Self {
            GetDesc: GetDesc::<Impl>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Impl>,
            GetMemberTypeByName: GetMemberTypeByName::<Impl>,
            GetMemberTypeName: GetMemberTypeName::<Impl>,
            IsEqual: IsEqual::<Impl>,
            GetSubType: GetSubType::<Impl>,
            GetBaseClass: GetBaseClass::<Impl>,
            GetNumInterfaces: GetNumInterfaces::<Impl>,
            GetInterfaceByIndex: GetInterfaceByIndex::<Impl>,
            IsOfType: IsOfType::<Impl>,
            ImplementsInterface: ImplementsInterface::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D12ShaderReflectionType_ImplVtbl<T: ID3D12ShaderReflectionType_Impl>(std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D12ShaderReflectionType_Impl> ID3D12ShaderReflectionType_ImplVtbl<T> {
    const VTABLE: ID3D12ShaderReflectionType_Vtbl = ID3D12ShaderReflectionType_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D12ShaderReflectionType {
    pub fn new<'a, T: ID3D12ShaderReflectionType_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D12ShaderReflectionType_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D12ShaderReflectionVariable_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D12_SHADER_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetType(&self) -> Option<ID3D12ShaderReflectionType>;
    fn GetBuffer(&self) -> Option<ID3D12ShaderReflectionConstantBuffer>;
    fn GetInterfaceSlot(&self, uarrayindex: u32) -> u32;
}
impl ID3D12ShaderReflectionVariable_Vtbl {
    pub const fn new<Impl: ID3D12ShaderReflectionVariable_Impl>() -> ID3D12ShaderReflectionVariable_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D12ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D12_SHADER_VARIABLE_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetType<Impl: ID3D12ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionVariable_Impl::GetType(this)
        }
        unsafe extern "system" fn GetBuffer<Impl: ID3D12ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D12ShaderReflectionConstantBuffer> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionVariable_Impl::GetBuffer(this)
        }
        unsafe extern "system" fn GetInterfaceSlot<Impl: ID3D12ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void, uarrayindex: u32) -> u32 {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D12ShaderReflectionVariable_Impl::GetInterfaceSlot(this, core::mem::transmute_copy(&uarrayindex))
        }
        Self { GetDesc: GetDesc::<Impl>, GetType: GetType::<Impl>, GetBuffer: GetBuffer::<Impl>, GetInterfaceSlot: GetInterfaceSlot::<Impl> }
    }
}
#[doc(hidden)]
struct ID3D12ShaderReflectionVariable_ImplVtbl<T: ID3D12ShaderReflectionVariable_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D12ShaderReflectionVariable_Impl> ID3D12ShaderReflectionVariable_ImplVtbl<T> {
    const VTABLE: ID3D12ShaderReflectionVariable_Vtbl = ID3D12ShaderReflectionVariable_Vtbl::new::<T>();
}
impl ID3D12ShaderReflectionVariable {
    pub fn new<'a, T: ID3D12ShaderReflectionVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D12ShaderReflectionVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D12SharingContract_Impl: Sized {
    fn Present(&self, presource: Option<&ID3D12Resource>, subresource: u32, window: super::super::Foundation::HWND);
    fn SharedFenceSignal(&self, pfence: Option<&ID3D12Fence>, fencevalue: u64);
    fn BeginCapturableWork(&self, guid: *const windows_core::GUID);
    fn EndCapturableWork(&self, guid: *const windows_core::GUID);
}
impl windows_core::RuntimeName for ID3D12SharingContract {}
impl ID3D12SharingContract_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12SharingContract_Impl, const OFFSET: isize>() -> ID3D12SharingContract_Vtbl {
        unsafe extern "system" fn Present<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12SharingContract_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, subresource: u32, window: super::super::Foundation::HWND) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12SharingContract_Impl::Present(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&window))
        }
        unsafe extern "system" fn SharedFenceSignal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12SharingContract_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfence: *mut core::ffi::c_void, fencevalue: u64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12SharingContract_Impl::SharedFenceSignal(this, windows_core::from_raw_borrowed(&pfence), core::mem::transmute_copy(&fencevalue))
        }
        unsafe extern "system" fn BeginCapturableWork<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12SharingContract_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12SharingContract_Impl::BeginCapturableWork(this, core::mem::transmute_copy(&guid))
        }
        unsafe extern "system" fn EndCapturableWork<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12SharingContract_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12SharingContract_Impl::EndCapturableWork(this, core::mem::transmute_copy(&guid))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Present: Present::<Identity, Impl, OFFSET>,
            SharedFenceSignal: SharedFenceSignal::<Identity, Impl, OFFSET>,
            BeginCapturableWork: BeginCapturableWork::<Identity, Impl, OFFSET>,
            EndCapturableWork: EndCapturableWork::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12SharingContract as windows_core::Interface>::IID
    }
}
pub trait ID3D12StateObject_Impl: Sized + ID3D12Pageable_Impl {}
impl windows_core::RuntimeName for ID3D12StateObject {}
impl ID3D12StateObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12StateObject_Impl, const OFFSET: isize>() -> ID3D12StateObject_Vtbl {
        Self { base__: ID3D12Pageable_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12StateObject as windows_core::Interface>::IID || iid == &<ID3D12Object as windows_core::Interface>::IID || iid == &<ID3D12DeviceChild as windows_core::Interface>::IID || iid == &<ID3D12Pageable as windows_core::Interface>::IID
    }
}
pub trait ID3D12StateObjectProperties_Impl: Sized {
    fn GetShaderIdentifier(&self, pexportname: &windows_core::PCWSTR) -> *mut core::ffi::c_void;
    fn GetShaderStackSize(&self, pexportname: &windows_core::PCWSTR) -> u64;
    fn GetPipelineStackSize(&self) -> u64;
    fn SetPipelineStackSize(&self, pipelinestacksizeinbytes: u64);
}
impl windows_core::RuntimeName for ID3D12StateObjectProperties {}
impl ID3D12StateObjectProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12StateObjectProperties_Impl, const OFFSET: isize>() -> ID3D12StateObjectProperties_Vtbl {
        unsafe extern "system" fn GetShaderIdentifier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12StateObjectProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexportname: windows_core::PCWSTR) -> *mut core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12StateObjectProperties_Impl::GetShaderIdentifier(this, core::mem::transmute(&pexportname))
        }
        unsafe extern "system" fn GetShaderStackSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12StateObjectProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexportname: windows_core::PCWSTR) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12StateObjectProperties_Impl::GetShaderStackSize(this, core::mem::transmute(&pexportname))
        }
        unsafe extern "system" fn GetPipelineStackSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12StateObjectProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12StateObjectProperties_Impl::GetPipelineStackSize(this)
        }
        unsafe extern "system" fn SetPipelineStackSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12StateObjectProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipelinestacksizeinbytes: u64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12StateObjectProperties_Impl::SetPipelineStackSize(this, core::mem::transmute_copy(&pipelinestacksizeinbytes))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetShaderIdentifier: GetShaderIdentifier::<Identity, Impl, OFFSET>,
            GetShaderStackSize: GetShaderStackSize::<Identity, Impl, OFFSET>,
            GetPipelineStackSize: GetPipelineStackSize::<Identity, Impl, OFFSET>,
            SetPipelineStackSize: SetPipelineStackSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12StateObjectProperties as windows_core::Interface>::IID
    }
}
pub trait ID3D12SwapChainAssistant_Impl: Sized {
    fn GetLUID(&self) -> super::super::Foundation::LUID;
    fn GetSwapChainObject(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetCurrentResourceAndCommandQueue(&self, riidresource: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void, riidqueue: *const windows_core::GUID, ppvqueue: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn InsertImplicitSync(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D12SwapChainAssistant {}
impl ID3D12SwapChainAssistant_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: isize>() -> ID3D12SwapChainAssistant_Vtbl {
        unsafe extern "system" fn GetLUID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = ID3D12SwapChainAssistant_Impl::GetLUID(this)
        }
        unsafe extern "system" fn GetSwapChainObject<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12SwapChainAssistant_Impl::GetSwapChainObject(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetCurrentResourceAndCommandQueue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riidresource: *const windows_core::GUID, ppvresource: *mut *mut core::ffi::c_void, riidqueue: *const windows_core::GUID, ppvqueue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12SwapChainAssistant_Impl::GetCurrentResourceAndCommandQueue(this, core::mem::transmute_copy(&riidresource), core::mem::transmute_copy(&ppvresource), core::mem::transmute_copy(&riidqueue), core::mem::transmute_copy(&ppvqueue)).into()
        }
        unsafe extern "system" fn InsertImplicitSync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12SwapChainAssistant_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12SwapChainAssistant_Impl::InsertImplicitSync(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLUID: GetLUID::<Identity, Impl, OFFSET>,
            GetSwapChainObject: GetSwapChainObject::<Identity, Impl, OFFSET>,
            GetCurrentResourceAndCommandQueue: GetCurrentResourceAndCommandQueue::<Identity, Impl, OFFSET>,
            InsertImplicitSync: InsertImplicitSync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12SwapChainAssistant as windows_core::Interface>::IID
    }
}
pub trait ID3D12Tools_Impl: Sized {
    fn EnableShaderInstrumentation(&self, benable: super::super::Foundation::BOOL);
    fn ShaderInstrumentationEnabled(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for ID3D12Tools {}
impl ID3D12Tools_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Tools_Impl, const OFFSET: isize>() -> ID3D12Tools_Vtbl {
        unsafe extern "system" fn EnableShaderInstrumentation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Tools_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benable: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Tools_Impl::EnableShaderInstrumentation(this, core::mem::transmute_copy(&benable))
        }
        unsafe extern "system" fn ShaderInstrumentationEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12Tools_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12Tools_Impl::ShaderInstrumentationEnabled(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableShaderInstrumentation: EnableShaderInstrumentation::<Identity, Impl, OFFSET>,
            ShaderInstrumentationEnabled: ShaderInstrumentationEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12Tools as windows_core::Interface>::IID
    }
}
pub trait ID3D12VersionedRootSignatureDeserializer_Impl: Sized {
    fn GetRootSignatureDescAtVersion(&self, converttoversion: D3D_ROOT_SIGNATURE_VERSION) -> windows_core::Result<*mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC>;
    fn GetUnconvertedRootSignatureDesc(&self) -> *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC;
}
impl windows_core::RuntimeName for ID3D12VersionedRootSignatureDeserializer {}
impl ID3D12VersionedRootSignatureDeserializer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12VersionedRootSignatureDeserializer_Impl, const OFFSET: isize>() -> ID3D12VersionedRootSignatureDeserializer_Vtbl {
        unsafe extern "system" fn GetRootSignatureDescAtVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12VersionedRootSignatureDeserializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, converttoversion: D3D_ROOT_SIGNATURE_VERSION, ppdesc: *mut *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D12VersionedRootSignatureDeserializer_Impl::GetRootSignatureDescAtVersion(this, core::mem::transmute_copy(&converttoversion)) {
                Ok(ok__) => {
                    core::ptr::write(ppdesc, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUnconvertedRootSignatureDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12VersionedRootSignatureDeserializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D12VersionedRootSignatureDeserializer_Impl::GetUnconvertedRootSignatureDesc(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRootSignatureDescAtVersion: GetRootSignatureDescAtVersion::<Identity, Impl, OFFSET>,
            GetUnconvertedRootSignatureDesc: GetUnconvertedRootSignatureDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VersionedRootSignatureDeserializer as windows_core::Interface>::IID
    }
}
pub trait ID3D12VirtualizationGuestDevice_Impl: Sized {
    fn ShareWithHost(&self, pobject: Option<&ID3D12DeviceChild>) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn CreateFenceFd(&self, pfence: Option<&ID3D12Fence>, fencevalue: u64) -> windows_core::Result<i32>;
}
impl windows_core::RuntimeName for ID3D12VirtualizationGuestDevice {}
impl ID3D12VirtualizationGuestDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12VirtualizationGuestDevice_Impl, const OFFSET: isize>() -> ID3D12VirtualizationGuestDevice_Vtbl {
        unsafe extern "system" fn ShareWithHost<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12VirtualizationGuestDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, phandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D12VirtualizationGuestDevice_Impl::ShareWithHost(this, windows_core::from_raw_borrowed(&pobject)) {
                Ok(ok__) => {
                    core::ptr::write(phandle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFenceFd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D12VirtualizationGuestDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfence: *mut core::ffi::c_void, fencevalue: u64, pfencefd: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D12VirtualizationGuestDevice_Impl::CreateFenceFd(this, windows_core::from_raw_borrowed(&pfence), core::mem::transmute_copy(&fencevalue)) {
                Ok(ok__) => {
                    core::ptr::write(pfencefd, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ShareWithHost: ShareWithHost::<Identity, Impl, OFFSET>,
            CreateFenceFd: CreateFenceFd::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D12VirtualizationGuestDevice as windows_core::Interface>::IID
    }
}
