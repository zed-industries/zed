/*!
# DirectX12 API internals.

Generally the mapping is straightforward.

## Resource transitions

D3D12 API matches WebGPU internal states very well. The only
caveat here is issuing a special UAV barrier whenever both source
and destination states match, and they are for storage sync.

## Memory

For now, all resources are created with "committed" memory.

## Sampler Descriptor Management

At most one descriptor heap of each type can be bound at once. This
means that the descriptors from all bind groups need to be present
in the same heap, and they need to be contiguous within that heap.
This is not a problem for the SRV/CBV/UAV heap as it can be sized into
the millions of entries. However the sampler heap is limited to 2048 entries.

In order to work around this limitation, we refer to samplers indirectly by index.
The entire sampler heap is bound at once and a buffer containing all sampler indexes
for that bind group is bound. The shader then uses the index to look up the sampler
in the heap. To help visualize this, the generated HLSL looks like this:

```wgsl
@group(0) @binding(2) var myLinearSampler: sampler;
@group(1) @binding(1) var myAnisoSampler: sampler;
@group(1) @binding(4) var myCompSampler: sampler;
```

```cpp
// These bindings alias the same descriptors. Depending on the type, the shader will use the correct one.
SamplerState nagaSamplerHeap[2048]: register(s0, space0);
SamplerComparisonState nagaComparisonSamplerHeap[2048]: register(s2048, space1);

StructuredBuffer<uint> nagaGroup0SamplerIndexArray : register(t0, space0);
StructuredBuffer<uint> nagaGroup1SamplerIndexArray : register(t1, space0);

// Indexes into group 0 index array
static const SamplerState myLinearSampler = nagaSamplerHeap[nagaGroup0SamplerIndexArray[0]];

// Indexes into group 1 index array
static const SamplerState myAnisoSampler = nagaSamplerHeap[nagaGroup1SamplerIndexArray[0]];
static const SamplerComparisonState myCompSampler = nagaComparisonSamplerHeap[nagaGroup1SamplerIndexArray[1]];
```

Without this transform we would need separate set of sampler descriptors for each unique combination of samplers
in a bind group. This results in a lot of duplication and makes it easy to hit the 2048 limit. With the transform
the limit is merely 2048 unique samplers in existence, which is much more reasonable.

## Resource binding

See [`crate::Device::create_pipeline_layout`] documentation for the structure
of the root signature corresponding to WebGPU pipeline layout.

Binding groups is mostly straightforward, with one big caveat:
all bindings have to be reset whenever the root signature changes.
This is the rule of D3D12, and we can do nothing to help it.

We detect this change at both [`crate::CommandEncoder::set_bind_group`]
and [`crate::CommandEncoder::set_render_pipeline`] with
[`crate::CommandEncoder::set_compute_pipeline`].

For this reason, in order avoid repeating the binding code,
we are binding everything in `CommandEncoder::update_root_elements`.
When the pipeline layout is changed, we reset all bindings.
Otherwise, we pass a range corresponding only to the current bind group.

!*/

mod adapter;
mod command;
mod conv;
mod dcomp;
mod descriptor;
mod device;
mod device_creation;
mod instance;
mod pipeline_desc;
mod sampler;
mod shader_compilation;
mod suballocation;
mod types;
mod view;

use alloc::{borrow::ToOwned as _, string::String, sync::Arc, vec::Vec};
use core::{ffi, fmt, mem, ops::Deref};

use arrayvec::ArrayVec;
use hashbrown::HashMap;
use parking_lot::{Mutex, RwLock};
use suballocation::Allocator;
use windows::{
    core::{Free as _, Interface},
    Win32::{
        Foundation,
        Graphics::{
            Direct3D,
            Direct3D12::{self, D3D12_TEXTURE_DATA_PLACEMENT_ALIGNMENT},
            DirectComposition, Dxgi,
        },
        System::Threading,
    },
};

use self::dcomp::DCompLib;
use crate::auxil::{
    self,
    dxgi::{
        factory::{DxgiAdapter, DxgiFactory},
        result::HResult,
    },
};

#[derive(Debug)]
struct DynLib {
    inner: libloading::Library,
}

impl DynLib {
    unsafe fn new<P>(filename: P) -> Result<Self, libloading::Error>
    where
        P: AsRef<std::ffi::OsStr>,
    {
        unsafe { libloading::Library::new(filename) }.map(|inner| Self { inner })
    }

    unsafe fn get<T>(
        &self,
        symbol: &[u8],
    ) -> Result<libloading::Symbol<'_, T>, crate::DeviceError> {
        unsafe { self.inner.get(symbol) }.map_err(|e| match e {
            libloading::Error::GetProcAddress { .. } | libloading::Error::GetProcAddressUnknown => {
                crate::DeviceError::Unexpected
            }
            libloading::Error::IncompatibleSize
            | libloading::Error::CreateCString { .. }
            | libloading::Error::CreateCStringWithTrailing { .. } => crate::hal_internal_error(e),
            _ => crate::DeviceError::Unexpected, // could be unreachable!() but we prefer to be more robust
        })
    }
}

#[derive(Debug)]
struct D3D12Lib {
    lib: DynLib,
}

#[derive(Clone, Copy)]
pub enum CreateDeviceError {
    GetProcAddress,
    D3D12CreateDevice(windows_core::HRESULT),
    RetDeviceIsNull,
}

impl D3D12Lib {
    fn new() -> Result<Self, libloading::Error> {
        unsafe { DynLib::new("d3d12.dll").map(|lib| Self { lib }) }
    }

    fn create_device(
        &self,
        adapter: &DxgiAdapter,
        feature_level: Direct3D::D3D_FEATURE_LEVEL,
    ) -> Result<Direct3D12::ID3D12Device, CreateDeviceError> {
        // Calls windows::Win32::Graphics::Direct3D12::D3D12CreateDevice on d3d12.dll
        type Fun = extern "system" fn(
            padapter: *mut ffi::c_void,
            minimumfeaturelevel: Direct3D::D3D_FEATURE_LEVEL,
            riid: *const windows_core::GUID,
            ppdevice: *mut *mut ffi::c_void,
        ) -> windows_core::HRESULT;
        let func: libloading::Symbol<Fun> =
            unsafe { self.lib.get(c"D3D12CreateDevice".to_bytes()) }
                .map_err(|_| CreateDeviceError::GetProcAddress)?;

        let mut result__: Option<Direct3D12::ID3D12Device> = None;

        let res = (func)(
            adapter.as_raw(),
            feature_level,
            // TODO: Generic?
            &Direct3D12::ID3D12Device::IID,
            <*mut _>::cast(&mut result__),
        );

        if res.is_err() {
            return Err(CreateDeviceError::D3D12CreateDevice(res));
        }

        result__.ok_or(CreateDeviceError::RetDeviceIsNull)
    }

    fn serialize_root_signature(
        &self,
        version: Direct3D12::D3D_ROOT_SIGNATURE_VERSION,
        parameters: &[Direct3D12::D3D12_ROOT_PARAMETER],
        static_samplers: &[Direct3D12::D3D12_STATIC_SAMPLER_DESC],
        flags: Direct3D12::D3D12_ROOT_SIGNATURE_FLAGS,
    ) -> Result<D3DBlob, crate::DeviceError> {
        // Calls windows::Win32::Graphics::Direct3D12::D3D12SerializeRootSignature on d3d12.dll
        type Fun = extern "system" fn(
            prootsignature: *const Direct3D12::D3D12_ROOT_SIGNATURE_DESC,
            version: Direct3D12::D3D_ROOT_SIGNATURE_VERSION,
            ppblob: *mut *mut ffi::c_void,
            pperrorblob: *mut *mut ffi::c_void,
        ) -> windows_core::HRESULT;
        let func: libloading::Symbol<Fun> =
            unsafe { self.lib.get(c"D3D12SerializeRootSignature".to_bytes()) }?;

        let desc = Direct3D12::D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: parameters.len() as _,
            pParameters: parameters.as_ptr(),
            NumStaticSamplers: static_samplers.len() as _,
            pStaticSamplers: static_samplers.as_ptr(),
            Flags: flags,
        };

        let mut blob = None;
        let mut error = None::<Direct3D::ID3DBlob>;
        (func)(
            &desc,
            version,
            <*mut _>::cast(&mut blob),
            <*mut _>::cast(&mut error),
        )
        .ok()
        .into_device_result("Root signature serialization")?;

        if let Some(error) = error {
            let error = D3DBlob(error);
            log::error!(
                "Root signature serialization error: {:?}",
                unsafe { error.as_c_str() }.unwrap().to_str().unwrap()
            );
            return Err(crate::DeviceError::Unexpected); // could be hal_usage_error or hal_internal_error
        }

        blob.ok_or(crate::DeviceError::Unexpected)
    }

    fn debug_interface(&self) -> Result<Option<Direct3D12::ID3D12Debug>, crate::DeviceError> {
        // Calls windows::Win32::Graphics::Direct3D12::D3D12GetDebugInterface on d3d12.dll
        type Fun = extern "system" fn(
            riid: *const windows_core::GUID,
            ppvdebug: *mut *mut ffi::c_void,
        ) -> windows_core::HRESULT;
        let func: libloading::Symbol<Fun> =
            unsafe { self.lib.get(c"D3D12GetDebugInterface".to_bytes()) }?;

        let mut result__ = None;

        let res = (func)(&Direct3D12::ID3D12Debug::IID, <*mut _>::cast(&mut result__)).ok();

        if let Err(ref err) = res {
            match err.code() {
                Dxgi::DXGI_ERROR_SDK_COMPONENT_MISSING => return Ok(None),
                _ => {}
            }
        }

        res.into_device_result("GetDebugInterface")?;

        result__.ok_or(crate::DeviceError::Unexpected).map(Some)
    }

    /// Calls D3D12GetInterface to obtain a COM interface by CLSID and IID.
    ///
    /// This is used by the Independent Devices API to obtain `ID3D12SDKConfiguration1`.
    fn get_interface<T: Interface>(
        &self,
        clsid: &windows_core::GUID,
    ) -> Result<T, GetInterfaceError> {
        // Calls windows::Win32::Graphics::Direct3D12::D3D12GetInterface on d3d12.dll
        type Fun = extern "system" fn(
            rclsid: *const windows_core::GUID,
            riid: *const windows_core::GUID,
            ppvdebug: *mut *mut ffi::c_void,
        ) -> windows_core::HRESULT;
        let func: libloading::Symbol<Fun> =
            unsafe { self.lib.get(c"D3D12GetInterface".to_bytes()) }
                .map_err(|_| GetInterfaceError::GetProcAddress)?;

        let mut result__: Option<T> = None;

        let res = (func)(clsid, &T::IID, <*mut _>::cast(&mut result__));

        if res.is_err() {
            return Err(GetInterfaceError::D3D12GetInterface(res));
        }

        result__.ok_or(GetInterfaceError::RetIsNull)
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum GetInterfaceError {
    GetProcAddress,
    D3D12GetInterface(windows_core::HRESULT),
    RetIsNull,
}

impl fmt::Display for GetInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GetProcAddress => write!(f, "D3D12GetInterface not found in d3d12.dll"),
            Self::D3D12GetInterface(hr) => write!(f, "D3D12GetInterface failed: {hr}"),
            Self::RetIsNull => write!(f, "D3D12GetInterface returned null"),
        }
    }
}

impl core::error::Error for GetInterfaceError {}

#[derive(Debug)]
pub(super) struct DxgiLib {
    lib: DynLib,
}

impl DxgiLib {
    pub fn new() -> Result<Self, libloading::Error> {
        unsafe { DynLib::new("dxgi.dll").map(|lib| Self { lib }) }
    }

    /// Will error with crate::DeviceError::Unexpected if DXGI 1.3 is not available.
    pub fn debug_interface1(&self) -> Result<Option<Dxgi::IDXGIInfoQueue>, crate::DeviceError> {
        // Calls windows::Win32::Graphics::Dxgi::DXGIGetDebugInterface1 on dxgi.dll
        type Fun = extern "system" fn(
            flags: u32,
            riid: *const windows_core::GUID,
            pdebug: *mut *mut ffi::c_void,
        ) -> windows_core::HRESULT;
        let func: libloading::Symbol<Fun> =
            unsafe { self.lib.get(c"DXGIGetDebugInterface1".to_bytes()) }?;

        let mut result__ = None;

        let res = (func)(0, &Dxgi::IDXGIInfoQueue::IID, <*mut _>::cast(&mut result__)).ok();

        if let Err(ref err) = res {
            match err.code() {
                Dxgi::DXGI_ERROR_SDK_COMPONENT_MISSING => return Ok(None),
                _ => {}
            }
        }

        res.into_device_result("debug_interface1")?;

        result__.ok_or(crate::DeviceError::Unexpected).map(Some)
    }

    /// Will error with crate::DeviceError::Unexpected if DXGI 1.4 is not available.
    pub fn create_factory4(
        &self,
        factory_flags: Dxgi::DXGI_CREATE_FACTORY_FLAGS,
    ) -> Result<Dxgi::IDXGIFactory4, crate::DeviceError> {
        // Calls windows::Win32::Graphics::Dxgi::CreateDXGIFactory2 on dxgi.dll
        type Fun = extern "system" fn(
            flags: Dxgi::DXGI_CREATE_FACTORY_FLAGS,
            riid: *const windows_core::GUID,
            ppfactory: *mut *mut ffi::c_void,
        ) -> windows_core::HRESULT;
        let func: libloading::Symbol<Fun> =
            unsafe { self.lib.get(c"CreateDXGIFactory2".to_bytes()) }?;

        let mut result__ = None;

        (func)(
            factory_flags,
            &Dxgi::IDXGIFactory4::IID,
            <*mut _>::cast(&mut result__),
        )
        .ok()
        .into_device_result("create_factory4")?;

        result__.ok_or(crate::DeviceError::Unexpected)
    }

    /// Will error with crate::DeviceError::Unexpected if DXGI 1.3 is not available.
    pub fn create_factory_media(&self) -> Result<Dxgi::IDXGIFactoryMedia, crate::DeviceError> {
        // Calls windows::Win32::Graphics::Dxgi::CreateDXGIFactory1 on dxgi.dll
        type Fun = extern "system" fn(
            riid: *const windows_core::GUID,
            ppfactory: *mut *mut ffi::c_void,
        ) -> windows_core::HRESULT;
        let func: libloading::Symbol<Fun> =
            unsafe { self.lib.get(c"CreateDXGIFactory1".to_bytes()) }?;

        let mut result__ = None;

        // https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nn-dxgi1_3-idxgifactorymedia
        (func)(&Dxgi::IDXGIFactoryMedia::IID, <*mut _>::cast(&mut result__))
            .ok()
            .into_device_result("create_factory_media")?;

        result__.ok_or(crate::DeviceError::Unexpected)
    }
}

/// Create a temporary "owned" copy inside a [`mem::ManuallyDrop`] without increasing the refcount or
/// moving away the source variable.
///
/// This is a common pattern when needing to pass interface pointers ("borrows") into Windows
/// structs.  Moving/cloning ownership is impossible/inconvenient because:
///
/// - The caller does _not_ assume ownership (and decrement the refcount at a later time);
/// - Unnecessarily increasing and decrementing the refcount;
/// - [`Drop`] destructors cannot run inside `union` structures (when the created structure is
///   implicitly dropped after a call).
///
/// See also <https://github.com/microsoft/windows-rs/pull/2361#discussion_r1150799401> and
/// <https://github.com/microsoft/windows-rs/issues/2386>.
///
/// # Safety
/// Performs a [`mem::transmute_copy()`] on a refcounted [`Interface`] type.  The returned
/// [`mem::ManuallyDrop`] should _not_ be dropped.
pub unsafe fn borrow_interface_temporarily<I: Interface>(src: &I) -> mem::ManuallyDrop<Option<I>> {
    unsafe { mem::transmute_copy(src) }
}

/// See [`borrow_interface_temporarily()`]
pub unsafe fn borrow_optional_interface_temporarily<I: Interface>(
    src: &Option<I>,
) -> mem::ManuallyDrop<Option<I>> {
    unsafe { mem::transmute_copy(src) }
}

struct D3DBlob(Direct3D::ID3DBlob);

impl Deref for D3DBlob {
    type Target = Direct3D::ID3DBlob;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl D3DBlob {
    unsafe fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.GetBufferPointer().cast(), self.GetBufferSize()) }
    }

    unsafe fn as_c_str(&self) -> Result<&ffi::CStr, ffi::FromBytesUntilNulError> {
        ffi::CStr::from_bytes_until_nul(unsafe { self.as_slice() })
    }
}

#[derive(Clone, Debug)]
pub struct Api;

impl crate::Api for Api {
    const VARIANT: wgt::Backend = wgt::Backend::Dx12;

    type Instance = Instance;
    type Surface = Surface;
    type Adapter = Adapter;
    type Device = Device;

    type Queue = Queue;
    type CommandEncoder = CommandEncoder;
    type CommandBuffer = CommandBuffer;

    type Buffer = Buffer;
    type Texture = Texture;
    type SurfaceTexture = Texture;
    type TextureView = TextureView;
    type Sampler = Sampler;
    type QuerySet = QuerySet;
    type Fence = Fence;

    type BindGroupLayout = BindGroupLayout;
    type BindGroup = BindGroup;
    type PipelineLayout = PipelineLayout;
    type ShaderModule = ShaderModule;
    type RenderPipeline = RenderPipeline;
    type ComputePipeline = ComputePipeline;
    type PipelineCache = PipelineCache;

    type AccelerationStructure = AccelerationStructure;
}

crate::impl_dyn_resource!(
    Adapter,
    AccelerationStructure,
    BindGroup,
    BindGroupLayout,
    Buffer,
    CommandBuffer,
    CommandEncoder,
    ComputePipeline,
    Device,
    Fence,
    Instance,
    PipelineCache,
    PipelineLayout,
    QuerySet,
    Queue,
    RenderPipeline,
    Sampler,
    ShaderModule,
    Surface,
    Texture,
    TextureView
);

// Limited by D3D12's root signature size of 64. Each element takes 1 or 2 entries.
const MAX_ROOT_ELEMENTS: usize = 64;
const ZERO_BUFFER_SIZE: wgt::BufferAddress = 256 << 10;

pub struct Instance {
    factory: DxgiFactory,
    factory_media: Option<Dxgi::IDXGIFactoryMedia>,
    // `device_factory` must be dropped before `library` because the COM
    // object's Release call goes through the d3d12.dll vtable.  If
    // `library` (which unloads d3d12.dll) is dropped first the Release
    // segfaults.
    device_factory: Arc<device_creation::DeviceFactory>,
    library: Arc<D3D12Lib>,
    dcomp_lib: Arc<DCompLib>,
    supports_allow_tearing: bool,
    presentation_system: wgt::Dx12SwapchainKind,
    _lib_dxgi: DxgiLib,
    flags: wgt::InstanceFlags,
    memory_budget_thresholds: wgt::MemoryBudgetThresholds,
    compiler_container: Arc<shader_compilation::CompilerContainer>,
    options: wgt::Dx12BackendOptions,
    telemetry: Option<crate::Telemetry>,
}

impl Instance {
    /// Get the raw DXGI factory associated with this instance.
    pub unsafe fn raw_factory4(&self) -> &Dxgi::IDXGIFactory4 {
        self.factory.deref()
    }

    pub unsafe fn create_surface_from_visual(&self, visual: *mut ffi::c_void) -> Surface {
        let visual = unsafe { DirectComposition::IDCompositionVisual::from_raw_borrowed(&visual) }
            .expect("COM pointer should not be NULL");
        Surface {
            factory: self.factory.clone(),
            factory_media: self.factory_media.clone(),
            target: SurfaceTarget::Visual(visual.to_owned()),
            supports_allow_tearing: self.supports_allow_tearing,
            swap_chain: RwLock::new(None),
            options: self.options.clone(),
        }
    }

    pub unsafe fn create_surface_from_surface_handle(
        &self,
        surface_handle: *mut ffi::c_void,
    ) -> Surface {
        // TODO: We're not given ownership, so we shouldn't call HANDLE::free(). This puts an extra burden on the caller to keep it alive.
        // https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-duplicatehandle could help us, even though DirectComposition is not in the list?
        // Or we make all these types owned, require an ownership transition, and replace SurfaceTargetUnsafe with SurfaceTarget.
        let surface_handle = Foundation::HANDLE(surface_handle);
        Surface {
            factory: self.factory.clone(),
            factory_media: self.factory_media.clone(),
            target: SurfaceTarget::SurfaceHandle(surface_handle),
            supports_allow_tearing: self.supports_allow_tearing,
            swap_chain: RwLock::new(None),
            options: self.options.clone(),
        }
    }

    pub unsafe fn create_surface_from_swap_chain_panel(
        &self,
        swap_chain_panel: *mut ffi::c_void,
    ) -> Surface {
        let swap_chain_panel =
            unsafe { types::ISwapChainPanelNative::from_raw_borrowed(&swap_chain_panel) }
                .expect("COM pointer should not be NULL");
        Surface {
            factory: self.factory.clone(),
            factory_media: self.factory_media.clone(),
            target: SurfaceTarget::SwapChainPanel(swap_chain_panel.to_owned()),
            supports_allow_tearing: self.supports_allow_tearing,
            swap_chain: RwLock::new(None),
            options: self.options.clone(),
        }
    }
}

unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}

struct SwapChain {
    // TODO: Drop order frees the SWC before the raw image pointers...?
    raw: Dxgi::IDXGISwapChain3,
    // need to associate raw image pointers with the swapchain so they can be properly released
    // when the swapchain is destroyed
    resources: Vec<Direct3D12::ID3D12Resource>,
    /// Handle is freed in [`Self::release_resources()`]
    waitable: Option<Foundation::HANDLE>,
    acquired_count: usize,
    present_mode: wgt::PresentMode,
    format: wgt::TextureFormat,
    size: wgt::Extent3d,
}

enum SurfaceTarget {
    /// Borrowed, lifetime externally managed
    WndHandle(Foundation::HWND),
    /// `handle` is borrowed, lifetime externally managed
    VisualFromWndHandle {
        handle: Foundation::HWND,
        dcomp_state: Mutex<dcomp::DCompState>,
    },
    Visual(DirectComposition::IDCompositionVisual),
    /// Borrowed, lifetime externally managed
    SurfaceHandle(Foundation::HANDLE),
    SwapChainPanel(types::ISwapChainPanelNative),
}

pub struct Surface {
    factory: DxgiFactory,
    factory_media: Option<Dxgi::IDXGIFactoryMedia>,
    target: SurfaceTarget,
    supports_allow_tearing: bool,
    swap_chain: RwLock<Option<SwapChain>>,
    options: wgt::Dx12BackendOptions,
}

unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}

impl Surface {
    pub fn swap_chain(&self) -> Option<Dxgi::IDXGISwapChain3> {
        Some(self.swap_chain.read().as_ref()?.raw.clone())
    }

    /// Returns the waitable handle associated with this swap chain, if any.
    /// Handle is only valid while the swap chain is alive.
    pub unsafe fn waitable_handle(&self) -> Option<Foundation::HANDLE> {
        self.swap_chain.read().as_ref()?.waitable
    }
}

#[derive(Debug, Clone, Copy)]
enum MemoryArchitecture {
    Unified {
        #[allow(unused)]
        cache_coherent: bool,
    },
    NonUnified,
}

#[derive(Debug, Clone, Copy)]
struct PrivateCapabilities {
    instance_flags: wgt::InstanceFlags,
    workarounds: Workarounds,
    #[allow(unused)]
    heterogeneous_resource_heaps: bool,
    memory_architecture: MemoryArchitecture,
    heap_create_not_zeroed: bool,
    casting_fully_typed_format_supported: bool,
    suballocation_supported: bool,
    shader_model: naga::back::hlsl::ShaderModel,
    max_sampler_descriptor_heap_size: u32,
    unrestricted_buffer_texture_copy_pitch_supported: bool,
}

impl PrivateCapabilities {
    fn texture_data_placement_alignment(&self) -> u64 {
        if self.unrestricted_buffer_texture_copy_pitch_supported {
            4
        } else {
            D3D12_TEXTURE_DATA_PLACEMENT_ALIGNMENT.into()
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
struct Workarounds {
    // On WARP 1.0.13+, debug information in shaders in certain situations causes the device
    // to hang. https://github.com/gfx-rs/wgpu/issues/8368
    avoid_shader_debug_info: bool,
}

pub struct Adapter {
    raw: DxgiAdapter,
    device: Direct3D12::ID3D12Device,
    library: Arc<D3D12Lib>,
    dcomp_lib: Arc<DCompLib>,
    private_caps: PrivateCapabilities,
    presentation_timer: auxil::dxgi::time::PresentationTimer,
    memory_budget_thresholds: wgt::MemoryBudgetThresholds,
    compiler_container: Arc<shader_compilation::CompilerContainer>,
    options: wgt::Dx12BackendOptions,
}

unsafe impl Send for Adapter {}
unsafe impl Sync for Adapter {}

impl Adapter {
    pub fn as_raw(&self) -> &Dxgi::IDXGIAdapter3 {
        &self.raw
    }
}

struct Event(pub Foundation::HANDLE);
impl Event {
    pub fn create(manual_reset: bool, initial_state: bool) -> Result<Self, crate::DeviceError> {
        Ok(Self(
            unsafe { Threading::CreateEventA(None, manual_reset, initial_state, None) }
                .into_device_result("CreateEventA")?,
        ))
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe { Foundation::HANDLE::free(&mut self.0) }
    }
}

/// Helper structure for waiting for GPU.
struct Idler {
    fence: Direct3D12::ID3D12Fence,
}

#[derive(Debug, Clone)]
struct CommandSignatures {
    draw: Direct3D12::ID3D12CommandSignature,
    draw_indexed: Direct3D12::ID3D12CommandSignature,
    draw_mesh: Option<Direct3D12::ID3D12CommandSignature>,
    dispatch: Direct3D12::ID3D12CommandSignature,
}

struct DeviceShared {
    adapter: DxgiAdapter,
    zero_buffer: Direct3D12::ID3D12Resource,
    cmd_signatures: CommandSignatures,
    heap_views: descriptor::GeneralHeap,
    sampler_heap: sampler::SamplerHeap,
    private_caps: PrivateCapabilities,
}

unsafe impl Send for DeviceShared {}
unsafe impl Sync for DeviceShared {}

pub struct Device {
    raw: Direct3D12::ID3D12Device,
    present_queue: Direct3D12::ID3D12CommandQueue,
    idler: Idler,
    features: wgt::Features,
    shared: Arc<DeviceShared>,
    options: wgt::Dx12BackendOptions,
    // CPU only pools
    rtv_pool: Arc<Mutex<descriptor::CpuPool>>,
    dsv_pool: Mutex<descriptor::CpuPool>,
    srv_uav_pool: Mutex<descriptor::CpuPool>,
    // library
    library: Arc<D3D12Lib>,
    dcomp_lib: Arc<DCompLib>,
    #[cfg(feature = "renderdoc")]
    render_doc: auxil::renderdoc::RenderDoc,
    null_rtv_handle: descriptor::Handle,
    mem_allocator: Allocator,
    compiler_container: Arc<shader_compilation::CompilerContainer>,
    shader_cache: Mutex<ShaderCache>,
    counters: Arc<wgt::HalCounters>,
}

impl Drop for Device {
    fn drop(&mut self) {
        self.rtv_pool.lock().free_handle(self.null_rtv_handle);
        if self
            .shared
            .private_caps
            .instance_flags
            .contains(wgt::InstanceFlags::VALIDATION)
        {
            auxil::dxgi::exception::unregister_exception_handler();
        }
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

pub struct Queue {
    raw: Direct3D12::ID3D12CommandQueue,
    temp_lists: Mutex<Vec<Option<Direct3D12::ID3D12CommandList>>>,
}

impl Queue {
    pub fn as_raw(&self) -> &Direct3D12::ID3D12CommandQueue {
        &self.raw
    }
}

unsafe impl Send for Queue {}
unsafe impl Sync for Queue {}

#[derive(Default)]
struct Temp {
    marker: Vec<u16>,
    barriers: Vec<Direct3D12::D3D12_RESOURCE_BARRIER>,
}

impl Temp {
    fn clear(&mut self) {
        self.marker.clear();
        self.barriers.clear();
    }
}

struct PassResolve {
    src: (Direct3D12::ID3D12Resource, u32),
    dst: (Direct3D12::ID3D12Resource, u32),
    format: Dxgi::Common::DXGI_FORMAT,
}

#[derive(Clone, Copy, Debug)]
enum RootElement {
    Empty,
    Constant,
    SpecialConstantBuffer {
        /// The first vertex in an indirect draw call, _or_ the `x` of a compute dispatch.
        first_vertex: i32,
        /// The first instance in an indirect draw call, _or_ the `y` of a compute dispatch.
        first_instance: u32,
        /// Unused in an indirect draw call, _or_ the `z` of a compute dispatch.
        other: u32,
    },
    /// Descriptor table.
    Table(Direct3D12::D3D12_GPU_DESCRIPTOR_HANDLE),
    /// Descriptor for an uniform buffer that has dynamic offset.
    DynamicUniformBuffer {
        address: Direct3D12::D3D12_GPU_DESCRIPTOR_HANDLE,
    },
    /// Descriptor table referring to the entire sampler heap.
    SamplerHeap,
    /// Root constants for dynamic offsets.
    ///
    /// start..end is the range of values in [`PassState::dynamic_storage_buffer_offsets`]
    /// that will be used to update the root constants.
    DynamicOffsetsBuffer {
        start: usize,
        end: usize,
    },
}

#[derive(Clone, Copy)]
enum PassKind {
    Render,
    Compute,
    Transfer,
}

struct PassState {
    has_label: bool,
    resolves: ArrayVec<PassResolve, { crate::MAX_COLOR_ATTACHMENTS }>,
    layout: PipelineLayoutShared,
    root_elements: [RootElement; MAX_ROOT_ELEMENTS],
    constant_data: [u32; MAX_ROOT_ELEMENTS],
    dynamic_storage_buffer_offsets: Vec<u32>,
    dirty_root_elements: u64,
    vertex_buffers: [Direct3D12::D3D12_VERTEX_BUFFER_VIEW; crate::MAX_VERTEX_BUFFERS],
    dirty_vertex_buffers: usize,
    kind: PassKind,
}

#[test]
fn test_dirty_mask() {
    assert_eq!(MAX_ROOT_ELEMENTS, u64::BITS as usize);
}

impl PassState {
    fn new() -> Self {
        PassState {
            has_label: false,
            resolves: ArrayVec::new(),
            layout: PipelineLayoutShared {
                signature: None,
                total_root_elements: 0,
                special_constants: None,
                root_constant_info: None,
                sampler_heap_root_index: None,
            },
            root_elements: [RootElement::Empty; MAX_ROOT_ELEMENTS],
            constant_data: [0; MAX_ROOT_ELEMENTS],
            dynamic_storage_buffer_offsets: Vec::new(),
            dirty_root_elements: 0,
            vertex_buffers: [Default::default(); crate::MAX_VERTEX_BUFFERS],
            dirty_vertex_buffers: 0,
            kind: PassKind::Transfer,
        }
    }

    fn clear(&mut self) {
        // careful about heap allocations!
        *self = Self::new();
    }
}

pub struct CommandEncoder {
    allocator: Direct3D12::ID3D12CommandAllocator,
    device: Direct3D12::ID3D12Device,
    shared: Arc<DeviceShared>,
    mem_allocator: Allocator,

    rtv_pool: Arc<Mutex<descriptor::CpuPool>>,
    temp_rtv_handles: Vec<descriptor::Handle>,

    intermediate_copy_bufs: Vec<Buffer>,

    null_rtv_handle: descriptor::Handle,
    list: Option<Direct3D12::ID3D12GraphicsCommandList>,
    free_lists: Vec<Direct3D12::ID3D12GraphicsCommandList>,
    pass: PassState,
    temp: Temp,

    /// If set, the end of the next render/compute pass will write a timestamp at
    /// the given pool & location.
    end_of_pass_timer_query: Option<(Direct3D12::ID3D12QueryHeap, u32)>,

    counters: Arc<wgt::HalCounters>,
}

unsafe impl Send for CommandEncoder {}
unsafe impl Sync for CommandEncoder {}

impl fmt::Debug for CommandEncoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandEncoder")
            .field("allocator", &self.allocator)
            .field("device", &self.allocator)
            .finish()
    }
}

#[derive(Debug)]
pub struct CommandBuffer {
    raw: Direct3D12::ID3D12GraphicsCommandList,
}

impl crate::DynCommandBuffer for CommandBuffer {}

unsafe impl Send for CommandBuffer {}
unsafe impl Sync for CommandBuffer {}

#[derive(Debug)]
pub struct Buffer {
    resource: Direct3D12::ID3D12Resource,
    // While the allocation also has _a_ size, it may not
    // be the same as the original size of the buffer,
    // as the allocation size varies for assorted reasons.
    size: wgt::BufferAddress,
    allocation: suballocation::Allocation,
}

unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}

impl crate::DynBuffer for Buffer {}

impl crate::BufferBinding<'_, Buffer> {
    fn resolve_size(&self) -> wgt::BufferAddress {
        match self.size {
            Some(size) => size.get(),
            None => self.buffer.size - self.offset,
        }
    }

    // TODO: Return GPU handle directly?
    fn resolve_address(&self) -> wgt::BufferAddress {
        (unsafe { self.buffer.resource.GetGPUVirtualAddress() }) + self.offset
    }
}

#[derive(Debug)]
pub struct Texture {
    resource: Direct3D12::ID3D12Resource,
    format: wgt::TextureFormat,
    dimension: wgt::TextureDimension,
    size: wgt::Extent3d,
    mip_level_count: u32,
    sample_count: u32,
    allocation: suballocation::Allocation,
}

impl Texture {
    pub unsafe fn raw_resource(&self) -> &Direct3D12::ID3D12Resource {
        &self.resource
    }
}

impl crate::DynTexture for Texture {}
impl crate::DynSurfaceTexture for Texture {}

impl core::borrow::Borrow<dyn crate::DynTexture> for Texture {
    fn borrow(&self) -> &dyn crate::DynTexture {
        self
    }
}

unsafe impl Send for Texture {}
unsafe impl Sync for Texture {}

impl Texture {
    fn array_layer_count(&self) -> u32 {
        match self.dimension {
            wgt::TextureDimension::D1 | wgt::TextureDimension::D3 => 1,
            wgt::TextureDimension::D2 => self.size.depth_or_array_layers,
        }
    }

    /// see <https://learn.microsoft.com/en-us/windows/win32/direct3d12/subresources#plane-slice>
    fn calc_subresource(&self, mip_level: u32, array_layer: u32, plane: u32) -> u32 {
        mip_level + (array_layer + plane * self.array_layer_count()) * self.mip_level_count
    }

    fn calc_subresource_for_copy(&self, base: &crate::TextureCopyBase) -> u32 {
        let plane = match base.aspect {
            crate::FormatAspects::COLOR
            | crate::FormatAspects::DEPTH
            | crate::FormatAspects::PLANE_0 => 0,
            crate::FormatAspects::STENCIL | crate::FormatAspects::PLANE_1 => 1,
            crate::FormatAspects::PLANE_2 => 2,
            _ => unreachable!(),
        };
        self.calc_subresource(base.mip_level, base.array_layer, plane)
    }
}

#[derive(Debug)]
pub struct TextureView {
    raw_format: Dxgi::Common::DXGI_FORMAT,
    aspects: crate::FormatAspects,
    dimension: wgt::TextureViewDimension,
    texture: Direct3D12::ID3D12Resource,
    subresource_index: u32,
    mip_slice: u32,
    handle_srv: Option<descriptor::Handle>,
    handle_uav: Option<descriptor::Handle>,
    handle_rtv: Option<descriptor::Handle>,
    handle_dsv_ro: Option<descriptor::Handle>,
    handle_dsv_rw: Option<descriptor::Handle>,
}

impl crate::DynTextureView for TextureView {}

unsafe impl Send for TextureView {}
unsafe impl Sync for TextureView {}

#[derive(Debug)]
pub struct Sampler {
    index: sampler::SamplerIndex,
    desc: Direct3D12::D3D12_SAMPLER_DESC,
}

impl crate::DynSampler for Sampler {}

unsafe impl Send for Sampler {}
unsafe impl Sync for Sampler {}

#[derive(Debug)]
pub struct QuerySet {
    raw: Direct3D12::ID3D12QueryHeap,
    raw_ty: Direct3D12::D3D12_QUERY_TYPE,
}

impl crate::DynQuerySet for QuerySet {}

unsafe impl Send for QuerySet {}
unsafe impl Sync for QuerySet {}

#[derive(Debug)]
pub struct Fence {
    raw: Direct3D12::ID3D12Fence,
}

impl crate::DynFence for Fence {}

unsafe impl Send for Fence {}
unsafe impl Sync for Fence {}

impl Fence {
    pub fn raw_fence(&self) -> &Direct3D12::ID3D12Fence {
        &self.raw
    }
}

#[derive(Debug)]
pub struct BindGroupLayout {
    /// Sorted list of entries.
    entries: Vec<wgt::BindGroupLayoutEntry>,
    cpu_heap_views: Option<descriptor::CpuHeap>,
    copy_counts: Vec<u32>, // all 1's
}

impl crate::DynBindGroupLayout for BindGroupLayout {}

#[derive(Debug, Clone, Copy)]
enum DynamicBuffer {
    Uniform(Direct3D12::D3D12_GPU_DESCRIPTOR_HANDLE),
    Storage,
}

#[derive(Debug)]
struct SamplerIndexBuffer {
    buffer: Direct3D12::ID3D12Resource,
    allocation: suballocation::Allocation,
}

#[derive(Debug)]
pub struct BindGroup {
    handle_views: Option<descriptor::DualHandle>,
    sampler_index_buffer: Option<SamplerIndexBuffer>,
    dynamic_buffers: Vec<DynamicBuffer>,
}

impl crate::DynBindGroup for BindGroup {}

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    struct TableTypes: u8 {
        const SRV_CBV_UAV = 1 << 0;
        const SAMPLERS = 1 << 1;
    }
}

// Element (also known as parameter) index into the root signature.
type RootIndex = u32;

#[derive(Debug)]
struct BindGroupInfo {
    base_root_index: RootIndex,
    tables: TableTypes,
    dynamic_storage_buffer_offsets: Option<DynamicStorageBufferOffsets>,
}

#[derive(Debug, Clone)]
struct RootConstantInfo {
    root_index: RootIndex,
    range: core::ops::Range<u32>,
}

#[derive(Debug, Clone)]
struct DynamicStorageBufferOffsets {
    root_index: RootIndex,
    range: core::ops::Range<usize>,
}

#[derive(Debug, Clone)]
struct PipelineLayoutShared {
    signature: Option<Direct3D12::ID3D12RootSignature>,
    total_root_elements: RootIndex,
    special_constants: Option<PipelineLayoutSpecialConstants>,
    root_constant_info: Option<RootConstantInfo>,
    sampler_heap_root_index: Option<RootIndex>,
}

unsafe impl Send for PipelineLayoutShared {}
unsafe impl Sync for PipelineLayoutShared {}

#[derive(Debug, Clone)]
struct PipelineLayoutSpecialConstants {
    root_index: RootIndex,
    indirect_cmd_signatures: Option<CommandSignatures>,
}

unsafe impl Send for PipelineLayoutSpecialConstants {}
unsafe impl Sync for PipelineLayoutSpecialConstants {}

#[derive(Debug)]
pub struct PipelineLayout {
    shared: PipelineLayoutShared,
    // Storing for each associated bind group, which tables we created
    // in the root signature. This is required for binding descriptor sets.
    bind_group_infos: [Option<BindGroupInfo>; crate::MAX_BIND_GROUPS],
    naga_options: naga::back::hlsl::Options,
}

impl crate::DynPipelineLayout for PipelineLayout {}

#[derive(Debug)]
pub struct ShaderModule {
    source: ShaderModuleSource,
    raw_name: Option<alloc::ffi::CString>,
    runtime_checks: wgt::ShaderRuntimeChecks,
}

impl crate::DynShaderModule for ShaderModule {}

#[derive(Default)]
pub struct ShaderCache {
    nr_of_shaders_compiled: u32,
    entries: HashMap<ShaderCacheKey, ShaderCacheValue>,
}

#[derive(PartialEq, Eq, Hash)]
pub(super) struct ShaderCacheKey {
    source: String,
    entry_point: String,
    stage: naga::ShaderStage,
    shader_model: naga::back::hlsl::ShaderModel,
}

pub(super) struct ShaderCacheValue {
    /// This is the value of [`ShaderCache::nr_of_shaders_compiled`]
    /// at the time the cache entry was last used.
    last_used: u32,
    shader: CompiledShader,
}

#[derive(Clone)]
pub(super) enum CompiledShader {
    Dxc(Direct3D::Dxc::IDxcBlob),
    Fxc(Direct3D::ID3DBlob),
    Precompiled(Vec<u8>),
}

impl CompiledShader {
    fn create_native_shader(&self) -> Direct3D12::D3D12_SHADER_BYTECODE {
        match self {
            CompiledShader::Dxc(shader) => Direct3D12::D3D12_SHADER_BYTECODE {
                pShaderBytecode: unsafe { shader.GetBufferPointer() },
                BytecodeLength: unsafe { shader.GetBufferSize() },
            },
            CompiledShader::Fxc(shader) => Direct3D12::D3D12_SHADER_BYTECODE {
                pShaderBytecode: unsafe { shader.GetBufferPointer() },
                BytecodeLength: unsafe { shader.GetBufferSize() },
            },
            CompiledShader::Precompiled(shader) => Direct3D12::D3D12_SHADER_BYTECODE {
                pShaderBytecode: shader.as_ptr().cast(),
                BytecodeLength: shader.len(),
            },
        }
    }
}

#[derive(Debug)]
pub struct RenderPipeline {
    raw: Direct3D12::ID3D12PipelineState,
    layout: PipelineLayoutShared,
    topology: Direct3D::D3D_PRIMITIVE_TOPOLOGY,
    vertex_strides: [Option<u32>; crate::MAX_VERTEX_BUFFERS],
}

impl crate::DynRenderPipeline for RenderPipeline {}

unsafe impl Send for RenderPipeline {}
unsafe impl Sync for RenderPipeline {}

#[derive(Debug)]
pub struct ComputePipeline {
    raw: Direct3D12::ID3D12PipelineState,
    layout: PipelineLayoutShared,
}

impl crate::DynComputePipeline for ComputePipeline {}

unsafe impl Send for ComputePipeline {}
unsafe impl Sync for ComputePipeline {}

#[derive(Debug)]
pub struct PipelineCache;

impl crate::DynPipelineCache for PipelineCache {}

#[derive(Debug)]
pub struct AccelerationStructure {
    resource: Direct3D12::ID3D12Resource,
    allocation: suballocation::Allocation,
}

impl crate::DynAccelerationStructure for AccelerationStructure {}

impl SwapChain {
    unsafe fn release_resources(mut self) -> Dxgi::IDXGISwapChain3 {
        if let Some(mut waitable) = self.waitable.take() {
            unsafe { Foundation::HANDLE::free(&mut waitable) };
        }
        self.raw
    }

    unsafe fn wait(
        &mut self,
        timeout: Option<core::time::Duration>,
    ) -> Result<bool, crate::SurfaceError> {
        let timeout_ms = match timeout {
            Some(duration) => duration.as_millis() as u32,
            None => Threading::INFINITE,
        };

        if let Some(waitable) = self.waitable {
            match unsafe { Threading::WaitForSingleObject(waitable, timeout_ms) } {
                Foundation::WAIT_ABANDONED | Foundation::WAIT_FAILED => {
                    Err(crate::SurfaceError::Lost)
                }
                Foundation::WAIT_OBJECT_0 => Ok(true),
                Foundation::WAIT_TIMEOUT => Ok(false),
                other => {
                    log::error!("Unexpected wait status: 0x{other:x?}");
                    Err(crate::SurfaceError::Lost)
                }
            }
        } else {
            Ok(true)
        }
    }
}

impl crate::Surface for Surface {
    type A = Api;

    unsafe fn configure(
        &self,
        device: &Device,
        config: &crate::SurfaceConfiguration,
    ) -> Result<(), crate::SurfaceError> {
        let mut flags = Dxgi::DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT;
        // We always set ALLOW_TEARING on the swapchain no matter
        // what kind of swapchain we want because ResizeBuffers
        // cannot change the swapchain's ALLOW_TEARING flag.
        //
        // This does not change the behavior of the swapchain, just
        // allow present calls to use tearing.
        if self.supports_allow_tearing {
            flags |= Dxgi::DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING;
        }

        // While `configure`s contract ensures that no work on the GPU's main queues
        // are in flight, we still need to wait for the present queue to be idle.
        unsafe { device.wait_for_present_queue_idle() }?;

        let non_srgb_format = auxil::dxgi::conv::map_texture_format_nosrgb(config.format);

        // The range for `SetMaximumFrameLatency` is 1-16 so the maximum latency requested should be 15 because we add 1.
        // https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice1-setmaximumframelatency
        debug_assert!(config.maximum_frame_latency <= 15);

        // Nvidia recommends to use 1-2 more buffers than the maximum latency
        // https://developer.nvidia.com/blog/advanced-api-performance-swap-chains/
        // For high latency extra buffers seems excessive, so go with a minimum of 3 and beyond that add 1.
        let swap_chain_buffer = (config.maximum_frame_latency + 1).min(16);

        let swap_chain = match self.swap_chain.write().take() {
            //Note: this path doesn't properly re-initialize all of the things
            Some(sc) => {
                let raw = unsafe { sc.release_resources() };
                let result = unsafe {
                    raw.ResizeBuffers(
                        swap_chain_buffer,
                        config.extent.width,
                        config.extent.height,
                        non_srgb_format,
                        flags,
                    )
                };
                if let Err(err) = result {
                    log::error!("ResizeBuffers failed: {err}");
                    return Err(crate::SurfaceError::Other("window is in use"));
                }
                raw
            }
            None => {
                let desc = Dxgi::DXGI_SWAP_CHAIN_DESC1 {
                    AlphaMode: auxil::dxgi::conv::map_acomposite_alpha_mode(
                        config.composite_alpha_mode,
                    ),
                    Width: config.extent.width,
                    Height: config.extent.height,
                    Format: non_srgb_format,
                    Stereo: false.into(),
                    SampleDesc: Dxgi::Common::DXGI_SAMPLE_DESC {
                        Count: 1,
                        Quality: 0,
                    },
                    BufferUsage: Dxgi::DXGI_USAGE_RENDER_TARGET_OUTPUT,
                    BufferCount: swap_chain_buffer,
                    Scaling: Dxgi::DXGI_SCALING_STRETCH,
                    SwapEffect: Dxgi::DXGI_SWAP_EFFECT_FLIP_DISCARD,
                    Flags: flags.0 as u32,
                };
                let swap_chain1 = match self.target {
                    SurfaceTarget::Visual(_)
                    | SurfaceTarget::VisualFromWndHandle { .. }
                    | SurfaceTarget::SwapChainPanel(_) => {
                        profiling::scope!("IDXGIFactory2::CreateSwapChainForComposition");
                        unsafe {
                            self.factory.CreateSwapChainForComposition(
                                &device.present_queue,
                                &desc,
                                None,
                            )
                        }
                    }
                    SurfaceTarget::SurfaceHandle(handle) => {
                        profiling::scope!(
                            "IDXGIFactoryMedia::CreateSwapChainForCompositionSurfaceHandle"
                        );
                        unsafe {
                            self.factory_media
                                .as_ref()
                                .ok_or(crate::SurfaceError::Other("IDXGIFactoryMedia not found"))?
                                .CreateSwapChainForCompositionSurfaceHandle(
                                    &device.present_queue,
                                    Some(handle),
                                    &desc,
                                    None,
                                )
                        }
                    }
                    SurfaceTarget::WndHandle(hwnd) => {
                        profiling::scope!("IDXGIFactory2::CreateSwapChainForHwnd");
                        unsafe {
                            self.factory.CreateSwapChainForHwnd(
                                &device.present_queue,
                                hwnd,
                                &desc,
                                None,
                                None,
                            )
                        }
                    }
                };

                let swap_chain1 = swap_chain1.map_err(|err| {
                    log::error!("SwapChain creation error: {err}");
                    crate::SurfaceError::Other("swapchain creation")
                })?;

                match &self.target {
                    SurfaceTarget::WndHandle(_) | SurfaceTarget::SurfaceHandle(_) => {}
                    SurfaceTarget::VisualFromWndHandle {
                        handle,
                        dcomp_state,
                    } => {
                        let mut dcomp_state = dcomp_state.lock();
                        let dcomp_state =
                            unsafe { dcomp_state.get_or_init(&device.dcomp_lib, handle) }?;
                        // Set the new swap chain as the content for the backing visual
                        // and commit the changes to the composition visual tree.
                        {
                            profiling::scope!("IDCompositionVisual::SetContent");
                            unsafe { dcomp_state.visual.SetContent(&swap_chain1) }.map_err(
                                |err| {
                                    log::error!("IDCompositionVisual::SetContent failed: {err}");
                                    crate::SurfaceError::Other("IDCompositionVisual::SetContent")
                                },
                            )?;
                        }

                        // Commit the changes to the composition device.
                        {
                            profiling::scope!("IDCompositionDevice::Commit");
                            unsafe { dcomp_state.device.Commit() }.map_err(|err| {
                                log::error!("IDCompositionDevice::Commit failed: {err}");
                                crate::SurfaceError::Other("IDCompositionDevice::Commit")
                            })?;
                        }
                    }
                    SurfaceTarget::Visual(visual) => {
                        if let Err(err) = unsafe { visual.SetContent(&swap_chain1) } {
                            log::error!("Unable to SetContent: {err}");
                            return Err(crate::SurfaceError::Other(
                                "IDCompositionVisual::SetContent",
                            ));
                        }
                    }
                    SurfaceTarget::SwapChainPanel(swap_chain_panel) => {
                        if let Err(err) = unsafe { swap_chain_panel.SetSwapChain(&swap_chain1) } {
                            log::error!("Unable to SetSwapChain: {err}");
                            return Err(crate::SurfaceError::Other(
                                "ISwapChainPanelNative::SetSwapChain",
                            ));
                        }
                    }
                }

                swap_chain1.cast::<Dxgi::IDXGISwapChain3>().map_err(|err| {
                    log::error!("Unable to cast swapchain: {err}");
                    crate::SurfaceError::Other("swapchain cast to version 3")
                })?
            }
        };

        match self.target {
            SurfaceTarget::WndHandle(wnd_handle) => {
                // Disable automatic Alt+Enter handling by DXGI.
                unsafe {
                    self.factory.MakeWindowAssociation(
                        wnd_handle,
                        Dxgi::DXGI_MWA_NO_WINDOW_CHANGES | Dxgi::DXGI_MWA_NO_ALT_ENTER,
                    )
                }
                .into_device_result("MakeWindowAssociation")?;
            }
            SurfaceTarget::Visual(_)
            | SurfaceTarget::VisualFromWndHandle { .. }
            | SurfaceTarget::SurfaceHandle(_)
            | SurfaceTarget::SwapChainPanel(_) => {}
        }

        unsafe { swap_chain.SetMaximumFrameLatency(config.maximum_frame_latency) }
            .into_device_result("SetMaximumFrameLatency")?;

        let waitable = match device.options.latency_waitable_object {
            wgt::Dx12UseFrameLatencyWaitableObject::None => None,
            wgt::Dx12UseFrameLatencyWaitableObject::Wait
            | wgt::Dx12UseFrameLatencyWaitableObject::DontWait => {
                Some(unsafe { swap_chain.GetFrameLatencyWaitableObject() })
            }
        };

        let mut resources = Vec::with_capacity(swap_chain_buffer as usize);
        for i in 0..swap_chain_buffer {
            let resource = unsafe { swap_chain.GetBuffer(i) }
                .into_device_result("Failed to get swapchain buffer")?;
            resources.push(resource);
        }

        let mut swapchain = self.swap_chain.write();
        *swapchain = Some(SwapChain {
            raw: swap_chain,
            resources,
            waitable,
            acquired_count: 0,
            present_mode: config.present_mode,
            format: config.format,
            size: config.extent,
        });

        Ok(())
    }

    unsafe fn unconfigure(&self, device: &Device) {
        if let Some(sc) = self.swap_chain.write().take() {
            unsafe {
                // While `unconfigure`s contract ensures that no work on the GPU's main queues
                // are in flight, we still need to wait for the present queue to be idle.

                // The major failure mode of this function is device loss,
                // which if we have lost the device, we should just continue
                // cleaning up, without error.
                let _ = device.wait_for_present_queue_idle();

                let _raw = sc.release_resources();
            }
        }
    }

    unsafe fn acquire_texture(
        &self,
        timeout: Option<core::time::Duration>,
        _fence: &Fence,
    ) -> Result<crate::AcquiredSurfaceTexture<Api>, crate::SurfaceError> {
        let mut swapchain = self.swap_chain.write();
        let sc = swapchain.as_mut().unwrap();

        match self.options.latency_waitable_object {
            wgt::Dx12UseFrameLatencyWaitableObject::None
            | wgt::Dx12UseFrameLatencyWaitableObject::DontWait => {}
            wgt::Dx12UseFrameLatencyWaitableObject::Wait => {
                unsafe { sc.wait(timeout) }?;
            }
        }

        let base_index = unsafe { sc.raw.GetCurrentBackBufferIndex() } as usize;
        let index = (base_index + sc.acquired_count) % sc.resources.len();
        sc.acquired_count += 1;

        let texture = Texture {
            resource: sc.resources[index].clone(),
            format: sc.format,
            dimension: wgt::TextureDimension::D2,
            size: sc.size,
            mip_level_count: 1,
            sample_count: 1,
            allocation: suballocation::Allocation::none(
                suballocation::AllocationType::Texture,
                sc.format.theoretical_memory_footprint(sc.size),
            ),
        };
        Ok(crate::AcquiredSurfaceTexture {
            texture,
            suboptimal: false,
        })
    }
    unsafe fn discard_texture(&self, _texture: Texture) {
        let mut swapchain = self.swap_chain.write();
        let sc = swapchain.as_mut().unwrap();
        sc.acquired_count -= 1;
    }
}

impl crate::Queue for Queue {
    type A = Api;

    unsafe fn submit(
        &self,
        command_buffers: &[&CommandBuffer],
        _surface_textures: &[&Texture],
        (signal_fence, signal_value): (&mut Fence, crate::FenceValue),
    ) -> Result<(), crate::DeviceError> {
        let mut temp_lists = self.temp_lists.lock();
        temp_lists.clear();
        for cmd_buf in command_buffers {
            temp_lists.push(Some(cmd_buf.raw.clone().into()));
        }

        {
            profiling::scope!("ID3D12CommandQueue::ExecuteCommandLists");
            unsafe { self.raw.ExecuteCommandLists(&temp_lists) }
        }

        unsafe { self.raw.Signal(&signal_fence.raw, signal_value) }
            .into_device_result("Signal fence")?;

        // Note the lack of synchronization here between the main Direct queue
        // and the dedicated presentation queue. This is automatically handled
        // by the D3D runtime by detecting uses of resources derived from the
        // swapchain. This automatic detection is why you cannot use a swapchain
        // as an UAV in D3D12.

        Ok(())
    }
    unsafe fn present(
        &self,
        surface: &Surface,
        _texture: Texture,
    ) -> Result<(), crate::SurfaceError> {
        let mut swapchain = surface.swap_chain.write();
        let sc = swapchain.as_mut().unwrap();
        sc.acquired_count -= 1;

        let (interval, flags) = match sc.present_mode {
            // We only allow immediate if ALLOW_TEARING is valid.
            wgt::PresentMode::Immediate => (0, Dxgi::DXGI_PRESENT_ALLOW_TEARING),
            wgt::PresentMode::Mailbox => (0, Dxgi::DXGI_PRESENT::default()),
            wgt::PresentMode::Fifo => (1, Dxgi::DXGI_PRESENT::default()),
            m => unreachable!("Cannot make surface with present mode {m:?}"),
        };

        profiling::scope!("IDXGISwapchain3::Present");
        unsafe { sc.raw.Present(interval, flags) }
            .ok()
            .into_device_result("Present")?;

        Ok(())
    }

    unsafe fn get_timestamp_period(&self) -> f32 {
        let frequency = unsafe { self.raw.GetTimestampFrequency() }.expect("GetTimestampFrequency");
        (1_000_000_000.0 / frequency as f64) as f32
    }
}
#[derive(Debug)]
pub struct DxilPassthroughShader {
    pub shader: Vec<u8>,
    pub num_workgroups: (u32, u32, u32),
}

#[derive(Debug)]
pub struct HlslPassthroughShader {
    pub shader: String,
    pub num_workgroups: (u32, u32, u32),
}

#[derive(Debug)]
pub enum ShaderModuleSource {
    Naga(crate::NagaShader),
    DxilPassthrough(DxilPassthroughShader),
    HlslPassthrough(HlslPassthroughShader),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeatureLevel {
    _11_0,
    _11_1,
    _12_0,
    _12_1,
    _12_2,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShaderModel {
    _5_1,
    _6_0,
    _6_1,
    _6_2,
    _6_3,
    _6_4,
    _6_5,
    _6_6,
    _6_7,
    _6_8,
    _6_9,
}
