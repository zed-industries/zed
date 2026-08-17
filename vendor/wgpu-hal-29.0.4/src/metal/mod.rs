/*!
# Metal API internals.

## Pipeline Layout

In Metal, immediates, vertex buffers, and resources in the bind groups
are all placed together in the native resource bindings, which work similarly to D3D11:
there are tables of textures, buffers, and samplers.

We put immediates first (if any) in the table, followed by bind group 0
resources, followed by other bind groups. The vertex buffers are bound at the very
end of the VS buffer table.

!*/

// `MTLFeatureSet` is superseded by `MTLGpuFamily`.
// However, `MTLGpuFamily` is only supported starting MacOS 10.15, whereas our minimum target is MacOS 10.13,
// See https://github.com/gpuweb/gpuweb/issues/1069 for minimum spec.
// TODO: Eventually all deprecated features should be abstracted and use new api when available.
#[allow(deprecated)]
mod adapter;
mod command;
mod conv;
mod device;
mod library_from_metallib;
mod surface;
mod time;

use alloc::{
    string::{String, ToString as _},
    sync::Arc,
    vec::Vec,
};
use core::{fmt, iter, ops, ptr::NonNull, sync::atomic};

use bitflags::bitflags;
use hashbrown::HashMap;
use naga::FastHashMap;
use objc2::{
    available,
    rc::{autoreleasepool, Retained},
    runtime::ProtocolObject,
};
use objc2_foundation::ns_string;
use objc2_metal::{
    MTLAccelerationStructure, MTLAccelerationStructureCommandEncoder, MTLArgumentBuffersTier,
    MTLBlitCommandEncoder, MTLBuffer, MTLCommandBuffer, MTLCommandBufferStatus, MTLCommandQueue,
    MTLComputeCommandEncoder, MTLComputePipelineState, MTLCounterSampleBuffer, MTLCullMode,
    MTLDepthClipMode, MTLDepthStencilState, MTLDevice, MTLDrawable, MTLIndexType,
    MTLLanguageVersion, MTLLibrary, MTLPrimitiveType, MTLReadWriteTextureTier,
    MTLRenderCommandEncoder, MTLRenderPipelineState, MTLRenderStages, MTLResource,
    MTLResourceUsage, MTLSamplerState, MTLSharedEvent, MTLSize, MTLTexture, MTLTextureType,
    MTLTriangleFillMode, MTLWinding,
};
use objc2_quartz_core::CAMetalLayer;
use parking_lot::{Mutex, RwLock};

#[derive(Clone, Debug)]
pub struct Api;

type ResourceIndex = u32;

impl crate::Api for Api {
    const VARIANT: wgt::Backend = wgt::Backend::Metal;

    type Instance = Instance;
    type Surface = Surface;
    type Adapter = Adapter;
    type Device = Device;

    type Queue = Queue;
    type CommandEncoder = CommandEncoder;
    type CommandBuffer = CommandBuffer;

    type Buffer = Buffer;
    type Texture = Texture;
    type SurfaceTexture = SurfaceTexture;
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
    SurfaceTexture,
    Texture,
    TextureView
);

/// Provides availability information about Mac APIs.
///
/// This may include Metal features that depend only on software support.
/// Features with varying hardware support are in [`CapabilitiesQuery`]
///
/// When feature detection is only needed once, it may also be done inline.
struct OsFeatures;

impl OsFeatures {
    fn display_sync() -> bool {
        // https://developer.apple.com/documentation/quartzcore/cametallayer/displaysyncenabled
        available!(macos = 10.13) || cfg!(target_abi = "macabi")
    }
}

pub struct Instance {}

impl Instance {
    pub fn create_surface_from_layer(&self, layer: &CAMetalLayer) -> Surface {
        Surface::from_layer(layer)
    }
}

impl crate::Instance for Instance {
    type A = Api;

    unsafe fn init(_desc: &crate::InstanceDescriptor<'_>) -> Result<Self, crate::InstanceError> {
        profiling::scope!("Init Metal Backend");
        // We do not enable metal validation based on the validation flags as it affects the entire
        // process. Instead, we enable the validation inside the test harness itself in tests/src/native.rs.
        Ok(Instance {})
    }

    unsafe fn create_surface(
        &self,
        display_handle: raw_window_handle::RawDisplayHandle,
        window_handle: raw_window_handle::RawWindowHandle,
    ) -> Result<Surface, crate::InstanceError> {
        let layer = match (display_handle, window_handle) {
            (
                raw_window_handle::RawDisplayHandle::AppKit(_),
                raw_window_handle::RawWindowHandle::AppKit(handle),
            ) => unsafe { raw_window_metal::Layer::from_ns_view(handle.ns_view) },
            (
                raw_window_handle::RawDisplayHandle::UiKit(_),
                raw_window_handle::RawWindowHandle::UiKit(handle),
            ) => unsafe { raw_window_metal::Layer::from_ui_view(handle.ui_view) },
            _ => {
                return Err(crate::InstanceError::new(format!(
                    "window handle {window_handle:?} is not a Metal-compatible handle"
                )))
            }
        };

        // SAFETY: The layer is an initialized instance of `CAMetalLayer`, and
        // we transfer the retain count to `Retained` using `into_raw`.
        let layer = unsafe {
            Retained::from_raw(layer.into_raw().cast::<CAMetalLayer>().as_ptr()).unwrap()
        };

        Ok(Surface::new(layer))
    }

    unsafe fn enumerate_adapters(
        &self,
        _surface_hint: Option<&Surface>,
    ) -> Vec<crate::ExposedAdapter<Api>> {
        let devices = objc2_metal::MTLCopyAllDevices();
        let mut adapters: Vec<crate::ExposedAdapter<Api>> =
            devices.into_iter().map(AdapterShared::expose).collect();
        adapters.sort_by_key(|ad| {
            (
                ad.adapter.shared.private_caps.low_power,
                ad.adapter.shared.private_caps.headless,
            )
        });
        adapters
    }
}

bitflags!(
    /// Similar to `MTLCounterSamplingPoint`, but a bit higher abstracted for our purposes.
    #[derive(Debug, Copy, Clone)]
    pub struct TimestampQuerySupport: u32 {
        /// On creating Metal encoders.
        const STAGE_BOUNDARIES = 1 << 1;
        /// Within existing draw encoders.
        const ON_RENDER_ENCODER = Self::STAGE_BOUNDARIES.bits() | (1 << 2);
        /// Within existing dispatch encoders.
        const ON_COMPUTE_ENCODER = Self::STAGE_BOUNDARIES.bits() | (1 << 3);
        /// Within existing blit encoders.
        const ON_BLIT_ENCODER = Self::STAGE_BOUNDARIES.bits() | (1 << 4);

        /// Within any wgpu render/compute pass.
        const INSIDE_WGPU_PASSES = Self::ON_RENDER_ENCODER.bits() | Self::ON_COMPUTE_ENCODER.bits();
    }
);

#[allow(dead_code)]
struct CapabilitiesQuery {
    msl_version: MTLLanguageVersion,
    fragment_rw_storage: bool,
    read_write_texture_tier: MTLReadWriteTextureTier,
    msaa_desktop: bool,
    msaa_apple3: bool,
    msaa_apple7: bool,
    resource_heaps: bool,
    argument_buffers: Option<MTLArgumentBuffersTier>,
    mutable_comparison_samplers: bool,
    sampler_clamp_to_border: bool,
    indirect_draw_dispatch: bool,
    base_vertex_first_instance_drawing: bool,
    dual_source_blending: bool,
    low_power: bool,
    headless: bool,
    layered_rendering: bool,
    function_specialization: bool,
    depth_clip_mode: bool,
    texture_cube_array: bool,
    supports_float_filtering: bool,
    format_depth24_stencil8: bool,
    format_depth32_stencil8_filter: bool,
    format_depth32_stencil8_none: bool,
    format_min_srgb_channels: u8,
    format_b5: bool,
    format_bc: bool,
    format_eac_etc: bool,
    format_astc: bool,
    format_astc_hdr: bool,
    format_astc_3d: bool,
    format_any8_unorm_srgb_all: bool,
    format_any8_unorm_srgb_no_write: bool,
    format_any8_snorm_all: bool,
    format_r16_norm_all: bool,
    format_r32_all: bool,
    format_r32_no_write: bool,
    format_r32float_no_write_no_filter: bool,
    format_r32float_no_filter: bool,
    format_r32float_all: bool,
    format_rgba8_srgb_all: bool,
    format_rgba8_srgb_no_write: bool,
    format_rgb10a2_unorm_all: bool,
    format_rgb10a2_unorm_no_write: bool,
    format_rgb10a2_uint_write: bool,
    format_rg11b10_all: bool,
    format_rg11b10_no_write: bool,
    format_rgb9e5_all: bool,
    format_rgb9e5_no_write: bool,
    format_rgb9e5_filter_only: bool,
    format_rg32_color: bool,
    format_rg32_color_write: bool,
    format_rg32float_all: bool,
    format_rg32float_color_blend: bool,
    format_rg32float_no_filter: bool,
    format_rgba32int_color: bool,
    format_rgba32int_color_write: bool,
    format_rgba32float_color: bool,
    format_rgba32float_color_write: bool,
    format_rgba32float_all: bool,
    format_depth16unorm: bool,
    format_depth16unorm_filter: bool,
    format_depth32float_filter: bool,
    format_depth32float_none: bool,
    format_bgr10a2_all: bool,
    format_bgr10a2_no_write: bool,
    max_buffers_per_stage: ResourceIndex,
    max_vertex_buffers: ResourceIndex,
    max_textures_per_stage: ResourceIndex,
    max_samplers_per_stage: ResourceIndex,
    max_binding_array_elements: ResourceIndex,
    max_sampler_binding_array_elements: ResourceIndex,
    buffer_alignment: u64,

    /// Platform-reported maximum buffer size
    ///
    /// This value is clamped to `u32::MAX` for `wgt::Limits`, so you probably
    /// shouldn't be looking at this copy.
    max_buffer_size: u64,
    max_texture_size: u64,
    max_texture_3d_size: u64,
    max_texture_layers: u64,
    max_fragment_input_components: u64,
    max_color_render_targets: u8,
    max_color_attachment_bytes_per_sample: u8,
    max_varying_components: u32,
    max_threads_per_group: u32,
    max_total_threadgroup_memory: u32,
    sample_count_mask: crate::TextureFormatCapabilities,
    supports_debug_markers: bool,
    supports_binary_archives: bool,
    supports_arrays_of_textures: bool,
    supports_arrays_of_textures_write: bool,
    supports_depth_clip_control: bool,
    supports_shader_primitive_index: bool,
    has_unified_memory: Option<bool>,
    timestamp_query_support: TimestampQuerySupport,
    supports_simd_scoped_operations: bool,
    supports_cooperative_matrix: bool,
    int64: bool,
    int64_atomics_min_max: bool,
    int64_atomics: bool,
    float_atomics: bool,
    mesh_shaders: bool,
    max_mesh_task_workgroup_count: u32,
    max_task_payload_size: u32,
    supported_vertex_amplification_factor: u32,
    shader_barycentrics: bool,
    supports_memoryless_storage: bool,
    supports_raytracing: bool,
}

#[derive(Debug)]
struct PrivateCapabilities {
    msl_version: MTLLanguageVersion,
    low_power: bool,
    headless: bool,
    has_unified_memory: Option<bool>,
    timestamp_query_support: TimestampQuerySupport,
    supports_memoryless_storage: bool,
    mesh_shaders: bool,
    max_buffers_per_stage: ResourceIndex,
    max_vertex_buffers: ResourceIndex,
    max_textures_per_stage: ResourceIndex,
    max_samplers_per_stage: ResourceIndex,
}

#[derive(Debug)]
struct PrivateTextureFormatCapabilities {
    read_write_texture_tier: MTLReadWriteTextureTier,
    sample_count_mask: crate::TextureFormatCapabilities,
    int64_atomics: bool,
    msaa_desktop: bool,
    msaa_apple3: bool,
    msaa_apple7: bool,
    format_r32float_all: bool,
    format_rgba8_srgb_all: bool,
    format_rgb10a2_uint_write: bool,
    format_rgb10a2_unorm_all: bool,
    format_rg11b10_all: bool,
    format_rg32float_all: bool,
    format_rgba32float_all: bool,
    format_depth16unorm: bool,
    format_depth16unorm_filter: bool,
    format_depth32float_filter: bool,
    format_depth24_stencil8: bool,
    format_bc: bool,
    format_eac_etc: bool,
    format_astc: bool,
    format_astc_hdr: bool,
}

#[derive(Clone, Debug)]
struct PrivateDisabilities {
    /// Near depth is not respected properly on some Intel GPUs.
    broken_viewport_near_depth: bool,
    /// Multi-target clears don't appear to work properly on Intel GPUs.
    #[allow(dead_code)]
    broken_layered_clear_image: bool,
}

#[derive(Debug)]
struct Settings {
    retain_command_buffer_references: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            retain_command_buffer_references: true,
        }
    }
}

struct AdapterShared {
    device: Retained<ProtocolObject<dyn MTLDevice>>,
    disabilities: PrivateDisabilities,
    private_caps: PrivateCapabilities,
    private_texture_format_caps: PrivateTextureFormatCapabilities,
    settings: Settings,
    presentation_timer: time::PresentationTimer,
}

unsafe impl Send for AdapterShared {}
unsafe impl Sync for AdapterShared {}

impl AdapterShared {
    fn new(
        device: Retained<ProtocolObject<dyn MTLDevice>>,
        capabilities_query: &CapabilitiesQuery,
    ) -> Self {
        let private_caps = capabilities_query.private_capabilities();
        let private_texture_format_caps = capabilities_query.private_texture_format_capabilities();
        log::debug!("{private_caps:#?}");
        log::debug!("{private_texture_format_caps:#?}");

        Self {
            disabilities: PrivateDisabilities::new(&device),
            private_caps,
            private_texture_format_caps,
            device,
            settings: Settings::default(),
            presentation_timer: time::PresentationTimer::new(),
        }
    }

    fn expose(device: Retained<ProtocolObject<dyn MTLDevice>>) -> crate::ExposedAdapter<Api> {
        let name = device.name().to_string();
        let capabilities_query = CapabilitiesQuery::new(&device);
        let shared = AdapterShared::new(device, &capabilities_query);
        let features = capabilities_query.features();
        let capabilities = capabilities_query.capabilities();
        crate::ExposedAdapter {
            info: wgt::AdapterInfo {
                name,
                vendor: 0,
                device: 0,
                device_type: shared.private_caps.device_type(),
                device_pci_bus_id: String::new(),
                driver: String::new(),
                driver_info: String::new(),
                backend: wgt::Backend::Metal,
                // These are hardcoded based on typical values for Metal devices
                //
                // See <https://github.com/gpuweb/gpuweb/blob/main/proposals/subgroups.md#adapter-info>
                // for more information.
                subgroup_min_size: 4,
                subgroup_max_size: 64,
                transient_saves_memory: shared.private_caps.supports_memoryless_storage,
            },
            features,
            capabilities,
            adapter: Adapter::new(Arc::new(shared)),
        }
    }
}

pub struct Adapter {
    shared: Arc<AdapterShared>,
}

pub struct Queue {
    shared: Arc<QueueShared>,
    timestamp_period: f32,
}

unsafe impl Send for Queue {}
unsafe impl Sync for Queue {}

impl Queue {
    pub unsafe fn queue_from_raw(
        raw: Retained<ProtocolObject<dyn MTLCommandQueue>>,
        timestamp_period: f32,
    ) -> Self {
        Self {
            shared: Arc::new(QueueShared {
                raw,
                command_buffer_created_not_submitted: atomic::AtomicUsize::new(0),
            }),
            timestamp_period,
        }
    }

    pub fn as_raw(&self) -> &ProtocolObject<dyn MTLCommandQueue> {
        &self.shared.raw
    }
}

#[derive(Debug)]
pub struct QueueShared {
    raw: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    // Tracks command buffers created via `CommandEncoder::begin_encoding` that
    // have not yet been submitted or discarded. Used to proactively fail
    // before hitting Metal's `maxCommandBufferCount`.
    //
    // (In a few places we call `.commandBuffer{,WithUnretainedReferences}` directly
    // to create command buffers for internal purposes. In those cases we always
    // commit the buffer immediately, so we don't adjust the counter for them.)
    command_buffer_created_not_submitted: atomic::AtomicUsize,
}

pub struct Device {
    shared: Arc<AdapterShared>,
    features: wgt::Features,
    counters: Arc<wgt::HalCounters>,
}

pub struct Surface {
    render_layer: Mutex<Retained<CAMetalLayer>>,
    swapchain_format: RwLock<Option<wgt::TextureFormat>>,
    extent: RwLock<wgt::Extent3d>,
}

unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}

#[derive(Debug)]
pub struct SurfaceTexture {
    texture: Texture,
    // Useful for UI-intensive applications that are sensitive to
    // window resizing.
    drawable: Retained<ProtocolObject<dyn MTLDrawable>>,
    present_with_transaction: bool,
}

impl crate::DynSurfaceTexture for SurfaceTexture {}

impl core::borrow::Borrow<Texture> for SurfaceTexture {
    fn borrow(&self) -> &Texture {
        &self.texture
    }
}

impl core::borrow::Borrow<dyn crate::DynTexture> for SurfaceTexture {
    fn borrow(&self) -> &dyn crate::DynTexture {
        &self.texture
    }
}

unsafe impl Send for SurfaceTexture {}
unsafe impl Sync for SurfaceTexture {}

impl crate::Queue for Queue {
    type A = Api;

    unsafe fn submit(
        &self,
        command_buffers: &[&CommandBuffer],
        _surface_textures: &[&SurfaceTexture],
        (signal_fence, signal_value): (&mut Fence, crate::FenceValue),
    ) -> Result<(), crate::DeviceError> {
        autoreleasepool(|_| {
            let extra_command_buffer = {
                let completed_value = Arc::clone(&signal_fence.completed_value);
                let block = block2::RcBlock::new(move |_cmd_buf| {
                    completed_value.store(signal_value, atomic::Ordering::Release);
                });

                let raw = match command_buffers.last() {
                    Some(&cmd_buf) => cmd_buf.raw.clone(),
                    None => {
                        // We do not bother adjusting `command_buffer_created_not_submitted`
                        // because we immediately commit this buffer.
                        self.shared
                            .raw
                            .commandBufferWithUnretainedReferences()
                            .unwrap()
                    }
                };
                raw.setLabel(Some(ns_string!("(wgpu internal) Signal")));
                unsafe { raw.addCompletedHandler(block2::RcBlock::as_ptr(&block)) };

                signal_fence.maintain();
                signal_fence
                    .pending_command_buffers
                    .push((signal_value, raw.clone()));

                if let Some(shared_event) = &signal_fence.shared_event {
                    raw.encodeSignalEvent_value(shared_event.as_ref(), signal_value);
                }
                // only return an extra one if it's extra
                match command_buffers.last() {
                    Some(_) => None,
                    None => Some(raw),
                }
            };

            for cmd_buffer in command_buffers {
                cmd_buffer.raw.commit();
                // One command buffer per `end_encoding` call moves from the
                // "created but not yet submitted" bucket into the submitted
                // set, so update the counter.
                let previous = self
                    .shared
                    .command_buffer_created_not_submitted
                    .fetch_sub(1, atomic::Ordering::AcqRel);
                debug_assert!(previous > 0);
            }

            if let Some(raw) = extra_command_buffer {
                raw.commit();
            }
        });
        Ok(())
    }
    unsafe fn present(
        &self,
        _surface: &Surface,
        texture: SurfaceTexture,
    ) -> Result<(), crate::SurfaceError> {
        autoreleasepool(|_| {
            // We do not bother adjusting `command_buffer_created_not_submitted`
            // because we immediately commit this buffer.
            let command_buffer = self.shared.raw.commandBuffer().unwrap();
            command_buffer.setLabel(Some(ns_string!("(wgpu internal) Present")));

            // https://developer.apple.com/documentation/quartzcore/cametallayer/1478157-presentswithtransaction?language=objc
            if !texture.present_with_transaction {
                command_buffer.presentDrawable(&texture.drawable);
            }

            command_buffer.commit();

            if texture.present_with_transaction {
                command_buffer.waitUntilScheduled();
                texture.drawable.present();
            }
        });
        Ok(())
    }

    unsafe fn get_timestamp_period(&self) -> f32 {
        self.timestamp_period
    }
}

#[derive(Debug)]
pub struct Buffer {
    raw: Retained<ProtocolObject<dyn MTLBuffer>>,
    size: wgt::BufferAddress,
}

unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}

impl crate::DynBuffer for Buffer {}

impl Buffer {
    fn as_raw(&self) -> NonNull<ProtocolObject<dyn MTLBuffer>> {
        unsafe { NonNull::new_unchecked(Retained::as_ptr(&self.raw) as *mut _) }
    }
}

impl crate::BufferBinding<'_, Buffer> {
    fn resolve_size(&self) -> wgt::BufferAddress {
        match self.size {
            Some(size) => size.get(),
            None => self.buffer.size - self.offset,
        }
    }
}

#[derive(Debug)]
pub struct Texture {
    raw: Retained<ProtocolObject<dyn MTLTexture>>,
    format: wgt::TextureFormat,
    raw_type: MTLTextureType,
    array_layers: u32,
    mip_levels: u32,
    copy_size: crate::CopyExtent,
}

impl Texture {
    pub fn raw_handle(&self) -> &ProtocolObject<dyn MTLTexture> {
        &self.raw
    }
}

impl crate::DynTexture for Texture {}

unsafe impl Send for Texture {}
unsafe impl Sync for Texture {}

#[derive(Debug)]
pub struct TextureView {
    raw: Retained<ProtocolObject<dyn MTLTexture>>,
    aspects: crate::FormatAspects,
}

impl crate::DynTextureView for TextureView {}

unsafe impl Send for TextureView {}
unsafe impl Sync for TextureView {}

impl TextureView {
    fn as_raw(&self) -> NonNull<ProtocolObject<dyn MTLTexture>> {
        unsafe { NonNull::new_unchecked(Retained::as_ptr(&self.raw) as *mut _) }
    }
}

#[derive(Debug)]
pub struct Sampler {
    raw: Retained<ProtocolObject<dyn MTLSamplerState>>,
}

impl crate::DynSampler for Sampler {}

unsafe impl Send for Sampler {}
unsafe impl Sync for Sampler {}

impl Sampler {
    fn as_raw(&self) -> NonNull<ProtocolObject<dyn MTLSamplerState>> {
        unsafe { NonNull::new_unchecked(Retained::as_ptr(&self.raw) as *mut _) }
    }
}

#[derive(Debug)]
pub struct BindGroupLayout {
    /// Sorted list of BGL entries.
    entries: Arc<[wgt::BindGroupLayoutEntry]>,
}

impl crate::DynBindGroupLayout for BindGroupLayout {}

#[derive(Clone, Debug, Default)]
struct ResourceData<T> {
    buffers: T,
    textures: T,
    samplers: T,
}

#[derive(Clone, Debug, Default)]
struct MultiStageData<T> {
    vs: T,
    fs: T,
    cs: T,
    ts: T,
    ms: T,
}

const NAGA_STAGES: MultiStageData<naga::ShaderStage> = MultiStageData {
    vs: naga::ShaderStage::Vertex,
    fs: naga::ShaderStage::Fragment,
    cs: naga::ShaderStage::Compute,
    ts: naga::ShaderStage::Task,
    ms: naga::ShaderStage::Mesh,
};

impl<T> ops::Index<naga::ShaderStage> for MultiStageData<T> {
    type Output = T;
    fn index(&self, stage: naga::ShaderStage) -> &T {
        match stage {
            naga::ShaderStage::Vertex => &self.vs,
            naga::ShaderStage::Fragment => &self.fs,
            naga::ShaderStage::Compute => &self.cs,
            naga::ShaderStage::Task => &self.ts,
            naga::ShaderStage::Mesh => &self.ms,
            naga::ShaderStage::RayGeneration
            | naga::ShaderStage::AnyHit
            | naga::ShaderStage::ClosestHit
            | naga::ShaderStage::Miss => unimplemented!(),
        }
    }
}

impl<T> MultiStageData<T> {
    fn map_ref<Y>(&self, fun: impl Fn(&T) -> Y) -> MultiStageData<Y> {
        MultiStageData {
            vs: fun(&self.vs),
            fs: fun(&self.fs),
            cs: fun(&self.cs),
            ts: fun(&self.ts),
            ms: fun(&self.ms),
        }
    }
    fn map<Y>(self, fun: impl Fn(T) -> Y) -> MultiStageData<Y> {
        MultiStageData {
            vs: fun(self.vs),
            fs: fun(self.fs),
            cs: fun(self.cs),
            ts: fun(self.ts),
            ms: fun(self.ms),
        }
    }
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        iter::once(&self.vs)
            .chain(iter::once(&self.fs))
            .chain(iter::once(&self.cs))
            .chain(iter::once(&self.ts))
            .chain(iter::once(&self.ms))
    }
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        iter::once(&mut self.vs)
            .chain(iter::once(&mut self.fs))
            .chain(iter::once(&mut self.cs))
            .chain(iter::once(&mut self.ts))
            .chain(iter::once(&mut self.ms))
    }
}

type MultiStageResourceCounters = MultiStageData<ResourceData<ResourceIndex>>;
type MultiStageResources = MultiStageData<naga::back::msl::EntryPointResources>;

#[derive(Debug)]
struct BindGroupLayoutInfo {
    base_resource_indices: MultiStageResourceCounters,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct ImmediateDataInfo {
    count: u32,
    buffer_index: ResourceIndex,
}

#[derive(Debug)]
pub struct PipelineLayout {
    bind_group_infos: [Option<BindGroupLayoutInfo>; crate::MAX_BIND_GROUPS],
    immediates_infos: MultiStageData<Option<ImmediateDataInfo>>,
    total_counters: MultiStageResourceCounters,
    total_immediates: u32,
    per_stage_map: MultiStageResources,
}

impl crate::DynPipelineLayout for PipelineLayout {}

#[derive(Debug)]
enum BufferLikeResource {
    Buffer {
        ptr: NonNull<ProtocolObject<dyn MTLBuffer>>,
        offset: wgt::BufferAddress,
        dynamic_index: Option<u32>,

        /// The buffer's size, if it is a [`Storage`] binding. Otherwise `None`.
        ///
        /// Buffers with the [`wgt::BufferBindingType::Storage`] binding type can
        /// hold WGSL runtime-sized arrays. When one does, we must pass its size to
        /// shader entry points to implement bounds checks and WGSL's `arrayLength`
        /// function. See `device::CompiledShader::sized_bindings` for details.
        ///
        /// [`Storage`]: wgt::BufferBindingType::Storage
        binding_size: Option<wgt::BufferSize>,

        binding_location: u32,
    },
    AccelerationStructure(NonNull<ProtocolObject<dyn MTLAccelerationStructure>>),
}

#[derive(Debug)]
struct UseResourceInfo {
    uses: MTLResourceUsage,
    stages: MTLRenderStages,
    visible_in_compute: bool,
}

impl Default for UseResourceInfo {
    fn default() -> Self {
        Self {
            uses: MTLResourceUsage::empty(),
            stages: MTLRenderStages::empty(),
            visible_in_compute: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct BindGroup {
    counters: MultiStageResourceCounters,
    buffers: Vec<BufferLikeResource>,
    samplers: Vec<NonNull<ProtocolObject<dyn MTLSamplerState>>>,
    textures: Vec<NonNull<ProtocolObject<dyn MTLTexture>>>,

    argument_buffers: Vec<Retained<ProtocolObject<dyn MTLBuffer>>>,
    resources_to_use: HashMap<NonNull<ProtocolObject<dyn MTLResource>>, UseResourceInfo>,
}

impl crate::DynBindGroup for BindGroup {}

unsafe impl Send for BindGroup {}
unsafe impl Sync for BindGroup {}

#[derive(Debug)]
pub enum ShaderModuleSource {
    Naga(crate::NagaShader),
    Passthrough(PassthroughShader),
}

#[derive(Debug)]
pub struct PassthroughShader {
    pub library: Retained<ProtocolObject<dyn MTLLibrary>>,
    pub num_workgroups: (u32, u32, u32),
}

unsafe impl Send for PassthroughShader {}
unsafe impl Sync for PassthroughShader {}

#[derive(Debug)]
pub struct ShaderModule {
    source: ShaderModuleSource,
    bounds_checks: wgt::ShaderRuntimeChecks,
}

impl crate::DynShaderModule for ShaderModule {}

#[derive(Debug)]
struct PipelineStageInfo {
    #[allow(dead_code)]
    library: Option<Retained<ProtocolObject<dyn MTLLibrary>>>,
    immediates: Option<ImmediateDataInfo>,

    /// The buffer argument table index at which we pass runtime-sized arrays' buffer sizes.
    ///
    /// See `device::CompiledShader::sized_bindings` for more details.
    sizes_slot: Option<naga::back::msl::Slot>,

    /// Bindings of all WGSL `storage` globals that contain runtime-sized arrays.
    ///
    /// See `device::CompiledShader::sized_bindings` for more details.
    sized_bindings: Vec<naga::ResourceBinding>,

    /// Info on all bound vertex buffers.
    vertex_buffer_mappings: Vec<naga::back::msl::VertexBufferMapping>,

    /// The workgroup size for compute, task or mesh stages
    raw_wg_size: MTLSize,

    /// The workgroup memory sizes for compute task or mesh stages
    work_group_memory_sizes: Vec<u32>,
}

// TODO(madsmtm): Derive this when a release with
// https://github.com/madsmtm/objc2/issues/804 is available (likely 0.4).
impl Default for PipelineStageInfo {
    fn default() -> Self {
        Self {
            library: Default::default(),
            immediates: Default::default(),
            sizes_slot: Default::default(),
            sized_bindings: Default::default(),
            vertex_buffer_mappings: Default::default(),
            raw_wg_size: MTLSize {
                width: 0,
                height: 0,
                depth: 0,
            },
            work_group_memory_sizes: Default::default(),
        }
    }
}

impl PipelineStageInfo {
    fn clear(&mut self) {
        self.immediates = None;
        self.sizes_slot = None;
        self.sized_bindings.clear();
        self.vertex_buffer_mappings.clear();
        self.library = None;
        self.work_group_memory_sizes.clear();
        self.raw_wg_size = MTLSize {
            width: 0,
            height: 0,
            depth: 0,
        };
    }

    fn assign_from(&mut self, other: &Self) {
        self.immediates = other.immediates;
        self.sizes_slot = other.sizes_slot;
        self.sized_bindings.clear();
        self.sized_bindings.extend_from_slice(&other.sized_bindings);
        self.vertex_buffer_mappings.clear();
        self.vertex_buffer_mappings
            .extend_from_slice(&other.vertex_buffer_mappings);
        self.library = Some(other.library.as_ref().unwrap().clone());
        self.raw_wg_size = other.raw_wg_size;
        self.work_group_memory_sizes.clear();
        self.work_group_memory_sizes
            .extend_from_slice(&other.work_group_memory_sizes);
    }
}

#[derive(Debug)]
pub struct RenderPipeline {
    raw: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    vs_info: Option<PipelineStageInfo>,
    fs_info: Option<PipelineStageInfo>,
    ts_info: Option<PipelineStageInfo>,
    ms_info: Option<PipelineStageInfo>,
    raw_primitive_type: MTLPrimitiveType,
    raw_triangle_fill_mode: MTLTriangleFillMode,
    raw_front_winding: MTLWinding,
    raw_cull_mode: MTLCullMode,
    raw_depth_clip_mode: Option<MTLDepthClipMode>,
    depth_stencil: Option<(
        Retained<ProtocolObject<dyn MTLDepthStencilState>>,
        wgt::DepthBiasState,
    )>,
}

unsafe impl Send for RenderPipeline {}
unsafe impl Sync for RenderPipeline {}

impl crate::DynRenderPipeline for RenderPipeline {}

#[derive(Debug)]
pub struct ComputePipeline {
    raw: Retained<ProtocolObject<dyn MTLComputePipelineState>>,
    cs_info: PipelineStageInfo,
}

unsafe impl Send for ComputePipeline {}
unsafe impl Sync for ComputePipeline {}

impl crate::DynComputePipeline for ComputePipeline {}

#[derive(Debug, Clone)]
pub struct QuerySet {
    raw_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    //Metal has a custom buffer for counters.
    counter_sample_buffer: Option<Retained<ProtocolObject<dyn MTLCounterSampleBuffer>>>,
    ty: wgt::QueryType,
}

impl crate::DynQuerySet for QuerySet {}

unsafe impl Send for QuerySet {}
unsafe impl Sync for QuerySet {}

#[derive(Debug)]
pub struct Fence {
    completed_value: Arc<atomic::AtomicU64>,
    /// The pending fence values have to be ascending.
    pending_command_buffers: Vec<(
        crate::FenceValue,
        Retained<ProtocolObject<dyn MTLCommandBuffer>>,
    )>,
    shared_event: Option<Retained<ProtocolObject<dyn MTLSharedEvent>>>,
}

impl crate::DynFence for Fence {}

unsafe impl Send for Fence {}
unsafe impl Sync for Fence {}

impl Fence {
    fn get_latest(&self) -> crate::FenceValue {
        let mut max_value = self.completed_value.load(atomic::Ordering::Acquire);
        for &(value, ref cmd_buf) in self.pending_command_buffers.iter() {
            if cmd_buf.status() == MTLCommandBufferStatus::Completed {
                max_value = value;
            }
        }
        max_value
    }

    fn maintain(&mut self) {
        let latest = self.get_latest();
        self.pending_command_buffers
            .retain(|&(value, _)| value > latest);
    }

    pub fn raw_shared_event(&self) -> Option<&ProtocolObject<dyn MTLSharedEvent>> {
        self.shared_event.as_deref()
    }
}

struct IndexState {
    buffer_ptr: NonNull<ProtocolObject<dyn MTLBuffer>>,
    offset: wgt::BufferAddress,
    stride: wgt::BufferAddress,
    raw_type: MTLIndexType,
}

#[derive(Default)]
struct Temp {
    binding_sizes: Vec<u32>,
}

struct CommandState {
    blit: Option<Retained<ProtocolObject<dyn MTLBlitCommandEncoder>>>,
    acceleration_structure_builder:
        Option<Retained<ProtocolObject<dyn MTLAccelerationStructureCommandEncoder>>>,
    render: Option<Retained<ProtocolObject<dyn MTLRenderCommandEncoder>>>,
    compute: Option<Retained<ProtocolObject<dyn MTLComputeCommandEncoder>>>,
    raw_primitive_type: MTLPrimitiveType,
    index: Option<IndexState>,
    stage_infos: MultiStageData<PipelineStageInfo>,

    /// Sizes of currently bound [`wgt::BufferBindingType::Storage`] buffers.
    ///
    /// Specifically:
    ///
    /// - The keys are [`ResourceBinding`] values (that is, the WGSL `@group`
    ///   and `@binding` attributes) for `var<storage>` global variables in the
    ///   current module that contain runtime-sized arrays.
    ///
    /// - The values are the actual sizes of the buffers currently bound to
    ///   provide those globals' contents, which are needed to implement bounds
    ///   checks and the WGSL `arrayLength` function.
    ///
    /// For each stage `S` in `stage_infos`, we consult this to find the sizes
    /// of the buffers listed in `stage_infos.S.sized_bindings`, which we must
    /// pass to the entry point.
    ///
    /// See `device::CompiledShader::sized_bindings` for more details.
    ///
    /// [`ResourceBinding`]: naga::ResourceBinding
    storage_buffer_length_map: FastHashMap<naga::ResourceBinding, wgt::BufferSize>,

    vertex_buffer_size_map: FastHashMap<u64, wgt::BufferSize>,

    immediates: Vec<u32>,

    /// Timer query that should be executed when the next pass starts.
    pending_timer_queries: Vec<(QuerySet, u32)>,
}

pub struct CommandEncoder {
    shared: Arc<AdapterShared>,
    queue_shared: Arc<QueueShared>,
    raw_cmd_buf: Option<Retained<ProtocolObject<dyn MTLCommandBuffer>>>,
    state: CommandState,
    temp: Temp,
    counters: Arc<wgt::HalCounters>,
}

impl fmt::Debug for CommandEncoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CommandEncoder")
            .field("raw_cmd_buf", &self.raw_cmd_buf)
            .finish()
    }
}

unsafe impl Send for CommandEncoder {}
unsafe impl Sync for CommandEncoder {}

#[derive(Debug)]
pub struct CommandBuffer {
    raw: Retained<ProtocolObject<dyn MTLCommandBuffer>>,
    queue_shared: Arc<QueueShared>,
}

impl crate::DynCommandBuffer for CommandBuffer {}

unsafe impl Send for CommandBuffer {}
unsafe impl Sync for CommandBuffer {}

#[derive(Debug)]
pub struct PipelineCache;

impl crate::DynPipelineCache for PipelineCache {}

#[derive(Debug)]
pub struct AccelerationStructure {
    raw: Retained<ProtocolObject<dyn MTLAccelerationStructure>>,
}

impl AccelerationStructure {
    fn as_raw(&self) -> NonNull<ProtocolObject<dyn MTLAccelerationStructure>> {
        unsafe { NonNull::new_unchecked(Retained::as_ptr(&self.raw) as *mut _) }
    }
}

impl crate::DynAccelerationStructure for AccelerationStructure {}
unsafe impl Send for AccelerationStructure {}
unsafe impl Sync for AccelerationStructure {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsType {
    Macos,
    Ios,
    Tvos,
    VisionOs,
}
