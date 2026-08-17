use alloc::{string::String, sync::Arc, vec::Vec};
use core::ptr;
use std::thread;

use parking_lot::Mutex;
use windows::{
    core::Interface as _,
    Win32::{
        Devices::DeviceAndDriverInstallation::{
            SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW,
            SetupDiGetDeviceRegistryPropertyW, DIGCF_PRESENT, GUID_DEVCLASS_DISPLAY, HDEVINFO,
            SPDRP_ADDRESS, SPDRP_BUSNUMBER, SPDRP_HARDWAREID, SP_DEVINFO_DATA,
        },
        Foundation::{GetLastError, ERROR_NO_MORE_ITEMS},
        Graphics::{Direct3D, Direct3D12, Dxgi},
        UI::WindowsAndMessaging,
    },
};

use super::D3D12Lib;
use crate::{
    auxil::{
        self,
        dxgi::{factory::DxgiAdapter, result::HResult},
    },
    dx12::{
        dcomp::DCompLib, device_creation::DeviceFactory, shader_compilation, FeatureLevel,
        ShaderModel, SurfaceTarget,
    },
};

impl Drop for super::Adapter {
    fn drop(&mut self) {
        // Debug tracking alive objects
        if !thread::panicking()
            && self
                .private_caps
                .instance_flags
                .contains(wgt::InstanceFlags::VALIDATION)
        {
            unsafe {
                self.report_live_objects();
            }
        }
    }
}

impl super::Adapter {
    pub unsafe fn report_live_objects(&self) {
        if let Ok(debug_device) = self.raw.cast::<Direct3D12::ID3D12DebugDevice>() {
            unsafe {
                debug_device.ReportLiveDeviceObjects(
                    Direct3D12::D3D12_RLDO_SUMMARY | Direct3D12::D3D12_RLDO_IGNORE_INTERNAL,
                )
            }
            .unwrap()
        }
    }

    pub fn raw_adapter(&self) -> &DxgiAdapter {
        &self.raw
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn expose(
        adapter: DxgiAdapter,
        library: &Arc<D3D12Lib>,
        device_factory: &Arc<DeviceFactory>,
        dcomp_lib: &Arc<DCompLib>,
        instance_flags: wgt::InstanceFlags,
        memory_budget_thresholds: wgt::MemoryBudgetThresholds,
        compiler_container: Arc<shader_compilation::CompilerContainer>,
        backend_options: wgt::Dx12BackendOptions,
        telemetry: Option<crate::Telemetry>,
    ) -> Option<crate::ExposedAdapter<super::Api>> {
        let desc = unsafe { adapter.GetDesc2() }.unwrap();
        let driver_version = unsafe { adapter.CheckInterfaceSupport(&Dxgi::IDXGIDevice::IID) };
        let driver_version = driver_version
            .map(|driver_version| {
                let driver_version = driver_version as u64;
                [
                    (driver_version >> 48) as u16,
                    (driver_version >> 32) as u16,
                    (driver_version >> 16) as u16,
                    driver_version as u16,
                ]
            })
            .map_err(|e| e.code());

        // Create the device so that we can get the capabilities.
        let res = {
            profiling::scope!("ID3D12Device::create_device");
            device_factory.create_device(library, &adapter, Direct3D::D3D_FEATURE_LEVEL_11_0)
        };
        if let Some(telemetry) = telemetry {
            if let Err(err) = res {
                (telemetry.d3d12_expose_adapter)(
                    &desc,
                    driver_version,
                    crate::D3D12ExposeAdapterResult::CreateDeviceError(err),
                );
            }
        }
        let device = res.ok()?;

        profiling::scope!("feature queries");

        // Detect the highest supported feature level.
        let d3d_feature_level = [
            Direct3D::D3D_FEATURE_LEVEL_12_2,
            Direct3D::D3D_FEATURE_LEVEL_12_1,
            Direct3D::D3D_FEATURE_LEVEL_12_0,
            Direct3D::D3D_FEATURE_LEVEL_11_1,
            Direct3D::D3D_FEATURE_LEVEL_11_0,
        ];
        let mut device_levels = Direct3D12::D3D12_FEATURE_DATA_FEATURE_LEVELS {
            NumFeatureLevels: d3d_feature_level.len() as u32,
            pFeatureLevelsRequested: d3d_feature_level.as_ptr().cast(),
            MaxSupportedFeatureLevel: Default::default(),
        };
        unsafe {
            device.CheckFeatureSupport(
                Direct3D12::D3D12_FEATURE_FEATURE_LEVELS,
                <*mut _>::cast(&mut device_levels),
                size_of_val(&device_levels) as u32,
            )
        }
        .unwrap();
        let max_feature_level = match device_levels.MaxSupportedFeatureLevel {
            Direct3D::D3D_FEATURE_LEVEL_11_0 => FeatureLevel::_11_0,
            Direct3D::D3D_FEATURE_LEVEL_11_1 => FeatureLevel::_11_1,
            Direct3D::D3D_FEATURE_LEVEL_12_0 => FeatureLevel::_12_0,
            Direct3D::D3D_FEATURE_LEVEL_12_1 => FeatureLevel::_12_1,
            Direct3D::D3D_FEATURE_LEVEL_12_2 => FeatureLevel::_12_2,
            fl => {
                if let Some(telemetry) = telemetry {
                    (telemetry.d3d12_expose_adapter)(
                        &desc,
                        driver_version,
                        crate::D3D12ExposeAdapterResult::UnknownFeatureLevel(fl.0),
                    );
                }
                return None;
            }
        };

        let device_name = auxil::dxgi::conv::map_adapter_name(desc.Description);

        let mut features_architecture = Direct3D12::D3D12_FEATURE_DATA_ARCHITECTURE::default();

        unsafe {
            device.CheckFeatureSupport(
                Direct3D12::D3D12_FEATURE_ARCHITECTURE,
                <*mut _>::cast(&mut features_architecture),
                size_of_val(&features_architecture) as u32,
            )
        }
        .unwrap();

        let mut features1 = Direct3D12::D3D12_FEATURE_DATA_D3D12_OPTIONS1::default();
        let hr = unsafe {
            device.CheckFeatureSupport(
                Direct3D12::D3D12_FEATURE_D3D12_OPTIONS1,
                <*mut _>::cast(&mut features1),
                size_of_val(&features1) as u32,
            )
        };

        let mut workarounds = super::Workarounds::default();

        let is_warp = device_name.contains("Microsoft Basic Render Driver");

        // WARP uses two different versioning schemes. Versions that ship with windows
        // use a version that starts with 10.x.x.x. Versions that ship from Nuget use 1.0.x.x.
        //
        // As far as we know, this is only an issue on the Nuget versions.
        if let Ok(driver_version) = driver_version {
            if is_warp && driver_version >= [1, 0, 13, 0] && driver_version[0] < 10 {
                workarounds.avoid_shader_debug_info = true;
            }
        }

        let driver_version_string = {
            let driver_version = driver_version.unwrap_or([0, 0, 0, 0]);
            format!(
                "{}.{}.{}.{}",
                driver_version[0], driver_version[1], driver_version[2], driver_version[3]
            )
        };

        let info = wgt::AdapterInfo {
            backend: wgt::Backend::Dx12,
            name: device_name,
            vendor: desc.VendorId,
            device: desc.DeviceId,
            device_type: if Dxgi::DXGI_ADAPTER_FLAG(desc.Flags as i32)
                .contains(Dxgi::DXGI_ADAPTER_FLAG_SOFTWARE)
            {
                wgt::DeviceType::Cpu
            } else if features_architecture.UMA.as_bool() {
                wgt::DeviceType::IntegratedGpu
            } else {
                wgt::DeviceType::DiscreteGpu
            },
            device_pci_bus_id: get_adapter_pci_info(desc.VendorId, desc.DeviceId),
            driver: driver_version_string,
            driver_info: String::new(),
            subgroup_min_size: features1.WaveLaneCountMin,
            subgroup_max_size: features1.WaveLaneCountMax,
            transient_saves_memory: false,
        };

        let mut options = Direct3D12::D3D12_FEATURE_DATA_D3D12_OPTIONS::default();
        unsafe {
            device.CheckFeatureSupport(
                Direct3D12::D3D12_FEATURE_D3D12_OPTIONS,
                <*mut _>::cast(&mut options),
                size_of_val(&options) as u32,
            )
        }
        .unwrap();

        /// Resource Binding Tiers: https://learn.microsoft.com/en-us/windows/win32/direct3d12/hardware-support#limits-dependant-on-hardware
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        enum ResourceBindingTier {
            T1,
            T2,
            T3,
        }
        let rbt = match options.ResourceBindingTier {
            Direct3D12::D3D12_RESOURCE_BINDING_TIER_1 => ResourceBindingTier::T1,
            Direct3D12::D3D12_RESOURCE_BINDING_TIER_2 => ResourceBindingTier::T2,
            tier if tier.0 >= Direct3D12::D3D12_RESOURCE_BINDING_TIER_3.0 => {
                ResourceBindingTier::T3
            }
            other => {
                log::debug!("Got zero or negative value for resource binding tier {other:?}");
                ResourceBindingTier::T1
            }
        };

        if rbt == ResourceBindingTier::T1 {
            if let Some(telemetry) = telemetry {
                (telemetry.d3d12_expose_adapter)(
                    &desc,
                    driver_version,
                    crate::D3D12ExposeAdapterResult::ResourceBindingTier2Requirement,
                );
            }
            // We require Tier 2 or higher for the ability to make samplers bindless in all cases.
            return None;
        }

        let _depth_bounds_test_supported = {
            let mut features2 = Direct3D12::D3D12_FEATURE_DATA_D3D12_OPTIONS2::default();
            unsafe {
                device.CheckFeatureSupport(
                    Direct3D12::D3D12_FEATURE_D3D12_OPTIONS2,
                    <*mut _>::cast(&mut features2),
                    size_of_val(&features2) as u32,
                )
            }
            .is_ok()
                && features2.DepthBoundsTestSupported.as_bool()
        };

        let (casting_fully_typed_format_supported, view_instancing) = {
            let mut features3 = Direct3D12::D3D12_FEATURE_DATA_D3D12_OPTIONS3::default();
            if unsafe {
                device.CheckFeatureSupport(
                    Direct3D12::D3D12_FEATURE_D3D12_OPTIONS3,
                    <*mut _>::cast(&mut features3),
                    size_of_val(&features3) as u32,
                )
            }
            .is_ok()
            {
                (
                    features3.CastingFullyTypedFormatSupported.as_bool(),
                    features3.ViewInstancingTier.0 >= Direct3D12::D3D12_VIEW_INSTANCING_TIER_1.0,
                )
            } else {
                (false, false)
            }
        };

        let heap_create_not_zeroed = {
            // For D3D12_HEAP_FLAG_CREATE_NOT_ZEROED we just need to
            // make sure that options7 can be queried. See also:
            // https://devblogs.microsoft.com/directx/coming-to-directx-12-more-control-over-memory-allocation/
            let mut features7 = Direct3D12::D3D12_FEATURE_DATA_D3D12_OPTIONS7::default();
            unsafe {
                device.CheckFeatureSupport(
                    Direct3D12::D3D12_FEATURE_D3D12_OPTIONS7,
                    <*mut _>::cast(&mut features7),
                    size_of_val(&features7) as u32,
                )
            }
            .is_ok()
        };

        let unrestricted_buffer_texture_copy_pitch_supported = {
            let mut features13 = Direct3D12::D3D12_FEATURE_DATA_D3D12_OPTIONS13::default();
            unsafe {
                device.CheckFeatureSupport(
                    Direct3D12::D3D12_FEATURE_D3D12_OPTIONS13,
                    <*mut _>::cast(&mut features13),
                    size_of_val(&features13) as u32,
                )
            }
            .is_ok()
                && features13
                    .UnrestrictedBufferTextureCopyPitchSupported
                    .as_bool()
        };

        let mut max_sampler_descriptor_heap_size =
            Direct3D12::D3D12_MAX_SHADER_VISIBLE_SAMPLER_HEAP_SIZE;
        {
            let mut features19 = Direct3D12::D3D12_FEATURE_DATA_D3D12_OPTIONS19::default();
            let res = unsafe {
                device.CheckFeatureSupport(
                    Direct3D12::D3D12_FEATURE_D3D12_OPTIONS19,
                    <*mut _>::cast(&mut features19),
                    size_of_val(&features19) as u32,
                )
            };

            // Sometimes on Windows 11 23H2, the function returns success, even though the runtime
            // does not know about `Options19`. This can cause this number to be 0 as the structure isn't written to.
            // This value is nonsense and creating zero-sized sampler heaps can cause drivers to explode.
            // As as we're guaranteed 2048 anyway, we make sure this value is not under 2048.
            //
            // https://github.com/gfx-rs/wgpu/issues/7053
            let is_ok = res.is_ok();
            let is_above_minimum = features19.MaxSamplerDescriptorHeapSize
                > Direct3D12::D3D12_MAX_SHADER_VISIBLE_SAMPLER_HEAP_SIZE;
            if is_ok && is_above_minimum {
                max_sampler_descriptor_heap_size = features19.MaxSamplerDescriptorHeapSize;
            }
        };

        let mut shader_models_after_5_1 = [
            Direct3D12::D3D_SHADER_MODEL_6_9,
            Direct3D12::D3D_SHADER_MODEL_6_8,
            Direct3D12::D3D_SHADER_MODEL_6_7,
            Direct3D12::D3D_SHADER_MODEL_6_6,
            Direct3D12::D3D_SHADER_MODEL_6_5,
            Direct3D12::D3D_SHADER_MODEL_6_4,
            Direct3D12::D3D_SHADER_MODEL_6_3,
            Direct3D12::D3D_SHADER_MODEL_6_2,
            Direct3D12::D3D_SHADER_MODEL_6_1,
            Direct3D12::D3D_SHADER_MODEL_6_0,
        ]
        .iter();
        let max_device_shader_model = loop {
            if let Some(&sm) = shader_models_after_5_1.next() {
                let mut sm = Direct3D12::D3D12_FEATURE_DATA_SHADER_MODEL {
                    HighestShaderModel: sm,
                };
                if unsafe {
                    device.CheckFeatureSupport(
                        Direct3D12::D3D12_FEATURE_SHADER_MODEL,
                        <*mut _>::cast(&mut sm),
                        size_of_val(&sm) as u32,
                    )
                }
                .is_ok()
                {
                    break match sm.HighestShaderModel {
                        Direct3D12::D3D_SHADER_MODEL_5_1 => ShaderModel::_5_1,
                        Direct3D12::D3D_SHADER_MODEL_6_0 => ShaderModel::_6_0,
                        Direct3D12::D3D_SHADER_MODEL_6_1 => ShaderModel::_6_1,
                        Direct3D12::D3D_SHADER_MODEL_6_2 => ShaderModel::_6_2,
                        Direct3D12::D3D_SHADER_MODEL_6_3 => ShaderModel::_6_3,
                        Direct3D12::D3D_SHADER_MODEL_6_4 => ShaderModel::_6_4,
                        Direct3D12::D3D_SHADER_MODEL_6_5 => ShaderModel::_6_5,
                        Direct3D12::D3D_SHADER_MODEL_6_6 => ShaderModel::_6_6,
                        Direct3D12::D3D_SHADER_MODEL_6_7 => ShaderModel::_6_7,
                        Direct3D12::D3D_SHADER_MODEL_6_8 => ShaderModel::_6_8,
                        Direct3D12::D3D_SHADER_MODEL_6_9 => ShaderModel::_6_9,
                        _ => unreachable!(),
                    };
                }
            } else {
                break ShaderModel::_5_1;
            }
        };

        let wgt_shader_model = backend_options
            .force_shader_model
            .get()
            .or(compiler_container.max_shader_model());

        let shader_model = if let Some(max_shader_model) = wgt_shader_model {
            let max_dxc_shader_model = match max_shader_model {
                wgt::DxcShaderModel::V6_0 => ShaderModel::_6_0,
                wgt::DxcShaderModel::V6_1 => ShaderModel::_6_1,
                wgt::DxcShaderModel::V6_2 => ShaderModel::_6_2,
                wgt::DxcShaderModel::V6_3 => ShaderModel::_6_3,
                wgt::DxcShaderModel::V6_4 => ShaderModel::_6_4,
                wgt::DxcShaderModel::V6_5 => ShaderModel::_6_5,
                wgt::DxcShaderModel::V6_6 => ShaderModel::_6_6,
                wgt::DxcShaderModel::V6_7 => ShaderModel::_6_7,
                wgt::DxcShaderModel::V6_8 => ShaderModel::_6_8,
                wgt::DxcShaderModel::V6_9 => ShaderModel::_6_9,
            };

            let shader_model = max_device_shader_model.min(max_dxc_shader_model);

            match shader_model {
                ShaderModel::_5_1 => {
                    if let Some(telemetry) = telemetry {
                        (telemetry.d3d12_expose_adapter)(
                            &desc,
                            driver_version,
                            crate::D3D12ExposeAdapterResult::ShaderModel6Requirement,
                        );
                    }
                    // don't expose this adapter if it doesn't support DXIL
                    return None;
                }
                ShaderModel::_6_0 => naga::back::hlsl::ShaderModel::V6_0,
                ShaderModel::_6_1 => naga::back::hlsl::ShaderModel::V6_1,
                ShaderModel::_6_2 => naga::back::hlsl::ShaderModel::V6_2,
                ShaderModel::_6_3 => naga::back::hlsl::ShaderModel::V6_3,
                ShaderModel::_6_4 => naga::back::hlsl::ShaderModel::V6_4,
                ShaderModel::_6_5 => naga::back::hlsl::ShaderModel::V6_5,
                ShaderModel::_6_6 => naga::back::hlsl::ShaderModel::V6_6,
                ShaderModel::_6_7 => naga::back::hlsl::ShaderModel::V6_7,
                ShaderModel::_6_8 => naga::back::hlsl::ShaderModel::V6_8,
                ShaderModel::_6_9 => naga::back::hlsl::ShaderModel::V6_9,
            }
        } else {
            naga::back::hlsl::ShaderModel::V5_1
        };
        let private_caps = super::PrivateCapabilities {
            instance_flags,
            workarounds,
            heterogeneous_resource_heaps: options.ResourceHeapTier
                != Direct3D12::D3D12_RESOURCE_HEAP_TIER_1,
            memory_architecture: if features_architecture.UMA.as_bool() {
                super::MemoryArchitecture::Unified {
                    cache_coherent: features_architecture.CacheCoherentUMA.as_bool(),
                }
            } else {
                super::MemoryArchitecture::NonUnified
            },
            heap_create_not_zeroed,
            casting_fully_typed_format_supported,
            // See https://github.com/gfx-rs/wgpu/issues/3552
            suballocation_supported: !info.name.contains("Iris(R) Xe"),
            shader_model,
            max_sampler_descriptor_heap_size,
            unrestricted_buffer_texture_copy_pitch_supported,
        };

        // these should always be available on d3d12
        let mut features = wgt::Features::empty()
            | wgt::Features::DEPTH_CLIP_CONTROL
            | wgt::Features::DEPTH32FLOAT_STENCIL8
            | wgt::Features::INDIRECT_FIRST_INSTANCE
            | wgt::Features::MAPPABLE_PRIMARY_BUFFERS
            | wgt::Features::MULTI_DRAW_INDIRECT_COUNT
            | wgt::Features::ADDRESS_MODE_CLAMP_TO_BORDER
            | wgt::Features::ADDRESS_MODE_CLAMP_TO_ZERO
            | wgt::Features::POLYGON_MODE_LINE
            | wgt::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES
            | wgt::Features::TIMESTAMP_QUERY
            | wgt::Features::TIMESTAMP_QUERY_INSIDE_ENCODERS
            | wgt::Features::TIMESTAMP_QUERY_INSIDE_PASSES
            | wgt::Features::TEXTURE_COMPRESSION_BC
            | wgt::Features::TEXTURE_COMPRESSION_BC_SLICED_3D
            | wgt::Features::CLEAR_TEXTURE
            | wgt::Features::TEXTURE_FORMAT_16BIT_NORM
            | wgt::Features::IMMEDIATES
            | wgt::Features::PRIMITIVE_INDEX
            | wgt::Features::RG11B10UFLOAT_RENDERABLE
            | wgt::Features::DUAL_SOURCE_BLENDING
            | wgt::Features::TEXTURE_FORMAT_NV12
            | wgt::Features::FLOAT32_FILTERABLE
            | wgt::Features::TEXTURE_ATOMIC
            | wgt::Features::PASSTHROUGH_SHADERS
            | wgt::Features::EXTERNAL_TEXTURE
            | wgt::Features::MEMORY_DECORATION_COHERENT;

        //TODO: in order to expose this, we need to run a compute shader
        // that extract the necessary statistics out of the D3D12 result.
        // Alternatively, we could allocate a buffer for the query set,
        // write the results there, and issue a bunch of copy commands.
        //| wgt::Features::PIPELINE_STATISTICS_QUERY

        if max_feature_level >= FeatureLevel::_11_1 {
            features |= wgt::Features::VERTEX_WRITABLE_STORAGE;
        }

        features.set(
            wgt::Features::CONSERVATIVE_RASTERIZATION,
            options.ConservativeRasterizationTier
                != Direct3D12::D3D12_CONSERVATIVE_RASTERIZATION_TIER_NOT_SUPPORTED,
        );

        features.set(
            wgt::Features::TEXTURE_BINDING_ARRAY
                | wgt::Features::STORAGE_RESOURCE_BINDING_ARRAY
                | wgt::Features::STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING
                | wgt::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING
                // See note below the table https://learn.microsoft.com/en-us/windows/win32/direct3d12/hardware-support
                | wgt::Features::PARTIALLY_BOUND_BINDING_ARRAY,
            shader_model >= naga::back::hlsl::ShaderModel::V5_1 && rbt >= ResourceBindingTier::T3,
        );

        let bgra8unorm_storage_supported = {
            let mut bgra8unorm_info = Direct3D12::D3D12_FEATURE_DATA_FORMAT_SUPPORT {
                Format: Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM,
                ..Default::default()
            };
            let hr = unsafe {
                device.CheckFeatureSupport(
                    Direct3D12::D3D12_FEATURE_FORMAT_SUPPORT,
                    <*mut _>::cast(&mut bgra8unorm_info),
                    size_of_val(&bgra8unorm_info) as u32,
                )
            };
            hr.is_ok()
                && bgra8unorm_info
                    .Support2
                    .contains(Direct3D12::D3D12_FORMAT_SUPPORT2_UAV_TYPED_STORE)
        };
        features.set(
            wgt::Features::BGRA8UNORM_STORAGE,
            bgra8unorm_storage_supported,
        );

        let p010_format_supported = {
            let mut p010_info = Direct3D12::D3D12_FEATURE_DATA_FORMAT_SUPPORT {
                Format: Dxgi::Common::DXGI_FORMAT_P010,
                ..Default::default()
            };
            let hr = unsafe {
                device.CheckFeatureSupport(
                    Direct3D12::D3D12_FEATURE_FORMAT_SUPPORT,
                    <*mut _>::cast(&mut p010_info),
                    size_of_val(&p010_info) as u32,
                )
            };
            if hr.is_ok() {
                let supports_texture2d = p010_info
                    .Support1
                    .contains(Direct3D12::D3D12_FORMAT_SUPPORT1_TEXTURE2D);
                let supports_shader_load = p010_info
                    .Support1
                    .contains(Direct3D12::D3D12_FORMAT_SUPPORT1_SHADER_LOAD);
                let supports_shader_sample = p010_info
                    .Support1
                    .contains(Direct3D12::D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE);
                supports_texture2d && supports_shader_load && supports_shader_sample
            } else {
                false
            }
        };
        features.set(wgt::Features::TEXTURE_FORMAT_P010, p010_format_supported);

        features.set(
            wgt::Features::SHADER_INT64,
            shader_model >= naga::back::hlsl::ShaderModel::V6_0
                && hr.is_ok()
                && features1.Int64ShaderOps.as_bool(),
        );

        let float16_supported = {
            let mut features4 = Direct3D12::D3D12_FEATURE_DATA_D3D12_OPTIONS4::default();
            let hr = unsafe {
                device.CheckFeatureSupport(
                    Direct3D12::D3D12_FEATURE_D3D12_OPTIONS4, // https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_feature#syntax
                    ptr::from_mut(&mut features4).cast(),
                    size_of::<Direct3D12::D3D12_FEATURE_DATA_D3D12_OPTIONS4>() as _,
                )
            };
            hr.is_ok() && features4.Native16BitShaderOpsSupported.as_bool()
        };

        features.set(
            wgt::Features::SHADER_F16,
            shader_model >= naga::back::hlsl::ShaderModel::V6_2 && float16_supported,
        );

        features.set(
            wgt::Features::SUBGROUP,
            shader_model >= naga::back::hlsl::ShaderModel::V6_0
                && hr.is_ok()
                && features1.WaveOps.as_bool(),
        );
        let mut features5 = Direct3D12::D3D12_FEATURE_DATA_D3D12_OPTIONS5::default();
        let has_features5 = unsafe {
            device.CheckFeatureSupport(
                Direct3D12::D3D12_FEATURE_D3D12_OPTIONS5,
                <*mut _>::cast(&mut features5),
                size_of_val(&features5) as u32,
            )
        }
        .is_ok();

        // Once ray tracing pipelines are supported they also will go here
        let supports_ray_tracing = features5.RaytracingTier.0
            >= Direct3D12::D3D12_RAYTRACING_TIER_1_1.0
            && shader_model >= naga::back::hlsl::ShaderModel::V6_5
            && has_features5;

        features.set(
            wgt::Features::EXPERIMENTAL_RAY_QUERY
                | wgt::Features::EXTENDED_ACCELERATION_STRUCTURE_VERTEX_FORMATS,
            supports_ray_tracing,
        );

        // Binding arrays of TLAS are supported on D3D12 when ray tracing is supported.
        //
        // This flag is used for shader-side `binding_array<acceleration_structure>` as well as
        // allowing `BindGroupLayoutEntry::count = Some(...)` for `BindingType::AccelerationStructure`.
        features.set(
            wgt::Features::ACCELERATION_STRUCTURE_BINDING_ARRAY,
            supports_ray_tracing,
        );

        // Check for Int64 atomic support on buffers. This is very convoluted, but is based on a conservative reading
        // of https://microsoft.github.io/DirectX-Specs/d3d/HLSL_SM_6_6_Int64_and_Float_Atomics.html#integer-64-bit-capabilities.
        let atomic_int64_buffers;
        let atomic_int64_textures;
        {
            let mut features9 = Direct3D12::D3D12_FEATURE_DATA_D3D12_OPTIONS9::default();
            let hr9 = unsafe {
                device.CheckFeatureSupport(
                    Direct3D12::D3D12_FEATURE_D3D12_OPTIONS9,
                    <*mut _>::cast(&mut features9),
                    size_of_val(&features9) as u32,
                )
            }
            .is_ok();

            let mut features11 = Direct3D12::D3D12_FEATURE_DATA_D3D12_OPTIONS11::default();
            let hr11 = unsafe {
                device.CheckFeatureSupport(
                    Direct3D12::D3D12_FEATURE_D3D12_OPTIONS11,
                    <*mut _>::cast(&mut features11),
                    size_of_val(&features11) as u32,
                )
            }
            .is_ok();

            atomic_int64_buffers = hr9 && hr11 && hr.is_ok()
                // Int64 atomics show up in SM6.6.
                && shader_model >= naga::back::hlsl::ShaderModel::V6_6
                // They require Int64 to be available in the shader at all.
                && features1.Int64ShaderOps.as_bool()
                // As our RWByteAddressBuffers can exist on both descriptor heaps and
                // as root descriptors, we need to ensure that both cases are supported.
                // base SM6.6 only guarantees Int64 atomics on resources in root descriptors.
                && features11.AtomicInt64OnDescriptorHeapResourceSupported.as_bool()
                // Our Int64 atomic caps currently require groupshared. This
                // prevents Intel or Qcomm from using Int64 currently.
                // https://github.com/gfx-rs/wgpu/issues/8666
                && features9.AtomicInt64OnGroupSharedSupported.as_bool();

            atomic_int64_textures = hr9 && hr11 && hr.is_ok()
                // Int64 atomics show up in SM6.6.
                && shader_model >= naga::back::hlsl::ShaderModel::V6_6
                // They require Int64 to be available in the shader at all.
                && features1.Int64ShaderOps.as_bool()
                // Textures are typed resources, so we need this flag.
                && features9.AtomicInt64OnTypedResourceSupported.as_bool()
                // As textures can only exist in descriptor heaps, we require this.
                // However, all architectures that support atomics on typed resources
                // support this as well, so this is somewhat redundant.
                && features11.AtomicInt64OnDescriptorHeapResourceSupported.as_bool();
        };
        features.set(
            wgt::Features::SHADER_INT64_ATOMIC_ALL_OPS | wgt::Features::SHADER_INT64_ATOMIC_MIN_MAX,
            atomic_int64_buffers,
        );
        features.set(wgt::Features::TEXTURE_INT64_ATOMIC, atomic_int64_textures);
        let mesh_shader_supported = {
            let mut features7 = Direct3D12::D3D12_FEATURE_DATA_D3D12_OPTIONS7::default();
            unsafe {
                device.CheckFeatureSupport(
                    Direct3D12::D3D12_FEATURE_D3D12_OPTIONS7,
                    <*mut _>::cast(&mut features7),
                    size_of_val(&features7) as u32,
                )
            }
            .is_ok()
                && features7.MeshShaderTier != Direct3D12::D3D12_MESH_SHADER_TIER_NOT_SUPPORTED
        };
        features.set(
            wgt::Features::EXPERIMENTAL_MESH_SHADER,
            mesh_shader_supported,
        );
        let shader_barycentrics_supported = {
            let mut features3 = Direct3D12::D3D12_FEATURE_DATA_D3D12_OPTIONS3::default();
            unsafe {
                device.CheckFeatureSupport(
                    Direct3D12::D3D12_FEATURE_D3D12_OPTIONS3,
                    <*mut _>::cast(&mut features3),
                    size_of_val(&features3) as u32,
                )
            }
            .is_ok()
                && features3.BarycentricsSupported.as_bool()
                && shader_model >= naga::back::hlsl::ShaderModel::V6_1
        };
        features.set(
            wgt::Features::SHADER_BARYCENTRICS,
            shader_barycentrics_supported,
        );

        features.set(
            wgt::Features::MULTIVIEW,
            view_instancing && shader_model >= naga::back::hlsl::ShaderModel::V6_1,
        );
        features.set(
            wgt::Features::SELECTIVE_MULTIVIEW,
            view_instancing && shader_model >= naga::back::hlsl::ShaderModel::V6_1,
        );

        features.set(
            wgt::Features::EXPERIMENTAL_MESH_SHADER_MULTIVIEW,
            mesh_shader_supported
                && view_instancing
                && shader_model >= naga::back::hlsl::ShaderModel::V6_1,
        );

        // TODO: Determine if IPresentationManager is supported
        let presentation_timer = auxil::dxgi::time::PresentationTimer::new_dxgi();

        let downlevel = wgt::DownlevelCapabilities::default();

        // Limits that must share D3D12's root signature size of
        // D3D12_MAX_ROOT_COST 64 DWORDS (256 bytes).
        //
        // Root constants and root tables use 1 DWORD.
        // Root descriptors use 2 DWORDs.
        // Source: https://learn.microsoft.com/en-us/windows/win32/direct3d12/root-signature-limits#memory-limits-and-costs
        //
        // Per pipeline layout:
        // - RootElement::Constant, (immediates) 32 root constants
        //     (bounded by maxImmediateSize) = 32 x 4 bytes = 128 bytes
        // - RootElement::SamplerHeap, a root table = 4 bytes
        // - RootElement::SpecialConstantBuffer, 3 root constants = 3 x 4 bytes = 12 bytes
        // - RootElement::DynamicOffsetsBuffer, a root constant per dynamic storage buffer
        //     (bounded by maxDynamicStorageBuffersPerPipelineLayout) = 4 x 4 bytes = 16 bytes
        // - RootElement::DynamicUniformBuffer, a root descriptor per dynamic uniform buffer
        //     (bounded by maxDynamicUniformBuffersPerPipelineLayout) = 8 x 8 bytes = 64 bytes
        // Per bind group:
        // - RootElement::Table, a root table
        //     (bounded by maxBindGroups) = 8 x 4 bytes = 32 bytes
        //
        // Source: logic in `create_pipeline_layout`
        //
        // Total: 128 + 4 + 12 + 16 + 64 + 32 = 256 bytes
        //
        let max_immediate_size = 128;
        let max_bind_groups = 8;
        let max_dynamic_uniform_buffers_per_pipeline_layout = 8;
        let max_dynamic_storage_buffers_per_pipeline_layout = 4;

        // "Maximum number of descriptors in a Constant Buffer View (CBV), Shader Resource View (SRV), or Unordered Access View(UAV) heap used for rendering"
        let full_heap_count = match rbt {
            ResourceBindingTier::T1 | ResourceBindingTier::T2 => 1_000_000,
            // 1_000_000+
            ResourceBindingTier::T3 => {
                // Theoretically vram limited, but in practice 2^20 is the limit
                1 << 20
            }
        };

        // "Maximum number of Constant Buffer Views in all descriptor tables per shader stage"
        let max_uniform_buffers_per_shader_stage = match rbt {
            ResourceBindingTier::T1 | ResourceBindingTier::T2 => 14,
            _ => full_heap_count,
        };

        // "Maximum number of Shader Resource Views in all descriptor tables per shader stage"
        let mut max_srv_per_shader_stage = match rbt {
            ResourceBindingTier::T1 => 128,
            _ => full_heap_count,
        };

        // We use an extra SRV for all samplers in a bind group.
        // See comment in `create_pipeline_layout`.
        max_srv_per_shader_stage -= max_bind_groups;

        // If we also support acceleration structures these are shared so we must halve it.
        // It's unlikely that this affects anything because most devices that support ray tracing
        // probably have a higher binding tier than one.
        let mut max_sampled_textures_per_shader_stage = if supports_ray_tracing {
            max_srv_per_shader_stage / 2
        } else {
            max_srv_per_shader_stage
        };
        let mut max_acceleration_structures_per_shader_stage = if supports_ray_tracing {
            max_srv_per_shader_stage / 2
        } else {
            0
        };

        // "Maximum number of Unordered Access Views in all descriptor tables across all stages"
        let max_uav_across_all_stages = match rbt {
            ResourceBindingTier::T1 => match max_feature_level {
                FeatureLevel::_11_0 => 8,
                _ => 64,
            },
            ResourceBindingTier::T2 => 64,
            ResourceBindingTier::T3 => full_heap_count,
        };
        const MAX_SHADER_STAGES_PER_PIPELINE: u32 = 2;
        // We must share the UAV limit across both storage resource limits.
        let max_uav_per_shader_stage = max_uav_across_all_stages / MAX_SHADER_STAGES_PER_PIPELINE;
        let max_storage_textures_per_shader_stage = max_uav_per_shader_stage / 2;
        let mut max_storage_buffers_per_shader_stage = max_uav_per_shader_stage / 2;

        // WebGPU storage buffers count as 1 SRV if they are read-only
        // or as 1 UAV if they are read-write. See comment in
        // `create_pipeline_layout`. Make sure we don't exceed
        // the maximum number of SRVs for the relevant limits.
        auxil::cap_limits_to_be_under_the_sum_limit(
            [
                &mut max_sampled_textures_per_shader_stage,
                &mut max_acceleration_structures_per_shader_stage,
                &mut max_storage_buffers_per_shader_stage,
            ],
            max_srv_per_shader_stage,
        );

        // "Maximum number of Samplers in all descriptor tables per shader stage"
        let max_samplers_per_shader_stage = match rbt {
            ResourceBindingTier::T1 => 16,
            _ => 2048,
        };

        // See https://microsoft.github.io/DirectX-Specs/d3d/ViewInstancing.html#maximum-viewinstancecount
        let max_multiview_view_count = if view_instancing { 4 } else { 0 };

        if let Some(telemetry) = telemetry {
            (telemetry.d3d12_expose_adapter)(
                &desc,
                driver_version,
                crate::D3D12ExposeAdapterResult::Success(
                    max_feature_level,
                    max_device_shader_model,
                ),
            );
        }

        Some(crate::ExposedAdapter {
            adapter: super::Adapter {
                raw: adapter,
                device,
                library: Arc::clone(library),
                dcomp_lib: Arc::clone(dcomp_lib),
                private_caps,
                presentation_timer,
                memory_budget_thresholds,
                compiler_container,
                options: backend_options,
            },
            info,
            features,
            capabilities: crate::Capabilities {
                limits: auxil::adjust_raw_limits(wgt::Limits {
                    //
                    // WebGPU LIMITS:
                    // Based on https://gpuweb.github.io/gpuweb/correspondence/#limits
                    //
                    // 16384
                    max_texture_dimension_1d: Direct3D12::D3D12_REQ_TEXTURE1D_U_DIMENSION,
                    // 16384
                    max_texture_dimension_2d: Direct3D12::D3D12_REQ_TEXTURE2D_U_OR_V_DIMENSION
                        .min(Direct3D12::D3D12_REQ_TEXTURECUBE_DIMENSION),
                    // 2048
                    max_texture_dimension_3d: Direct3D12::D3D12_REQ_TEXTURE3D_U_V_OR_W_DIMENSION,
                    // 2048
                    max_texture_array_layers: Direct3D12::D3D12_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION,
                    // No real limit.
                    max_bindings_per_bind_group: u32::MAX,
                    max_sampled_textures_per_shader_stage,
                    max_samplers_per_shader_stage,
                    max_storage_textures_per_shader_stage,
                    max_storage_buffers_per_shader_stage,
                    max_uniform_buffers_per_shader_stage,
                    // See `InputSlot` param docs: https://learn.microsoft.com/en-ca/windows/win32/api/d3d12/ns-d3d12-d3d12_input_element_desc
                    max_vertex_buffers: 16,
                    // Dx12 does not expose a maximum buffer size in the API.
                    // This limit is chosen to avoid potential issues with drivers should they internally
                    // store buffer sizes using 32 bit ints (a situation we have already encountered with vulkan).
                    max_buffer_size: i32::MAX as u64,
                    max_storage_buffer_binding_size: auxil::MAX_I32_BINDING_SIZE as u64,
                    // 65536
                    max_uniform_buffer_binding_size:
                        Direct3D12::D3D12_REQ_CONSTANT_BUFFER_ELEMENT_COUNT as u64 * 16,
                    // 254
                    min_uniform_buffer_offset_alignment:
                        Direct3D12::D3D12_CONSTANT_BUFFER_DATA_PLACEMENT_ALIGNMENT,
                    // 16
                    min_storage_buffer_offset_alignment:
                        Direct3D12::D3D12_RAW_UAV_SRV_BYTE_ALIGNMENT,
                    // 32
                    max_vertex_attributes: Direct3D12::D3D12_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT,
                    // 2048
                    max_vertex_buffer_array_stride: Direct3D12::D3D12_SO_BUFFER_MAX_STRIDE_IN_BYTES,
                    // 31
                    max_inter_stage_shader_variables: Direct3D12::D3D12_VS_OUTPUT_REGISTER_COUNT
                        .min(Direct3D12::D3D12_PS_INPUT_REGISTER_COUNT)
                        - 1, // - 1 for position
                    max_immediate_size,
                    max_bind_groups,
                    max_dynamic_uniform_buffers_per_pipeline_layout,
                    max_dynamic_storage_buffers_per_pipeline_layout,
                    // 8
                    max_color_attachments: Direct3D12::D3D12_SIMULTANEOUS_RENDER_TARGET_COUNT,
                    // 128 (No documented limit)
                    max_color_attachment_bytes_per_sample:
                        Direct3D12::D3D12_SIMULTANEOUS_RENDER_TARGET_COUNT
                            * wgt::TextureFormat::MAX_TARGET_PIXEL_BYTE_COST,
                    // From: https://microsoft.github.io/DirectX-Specs/d3d/archive/D3D11_3_FunctionalSpec.htm#18.6.6%20Inter-Thread%20Data%20Sharing
                    max_compute_workgroup_storage_size: 32768,
                    // 1024
                    max_compute_invocations_per_workgroup:
                        Direct3D12::D3D12_CS_THREAD_GROUP_MAX_THREADS_PER_GROUP,
                    // 1024
                    max_compute_workgroup_size_x: Direct3D12::D3D12_CS_THREAD_GROUP_MAX_X,
                    // 1024
                    max_compute_workgroup_size_y: Direct3D12::D3D12_CS_THREAD_GROUP_MAX_Y,
                    // 64
                    max_compute_workgroup_size_z: Direct3D12::D3D12_CS_THREAD_GROUP_MAX_Z,
                    // 65535
                    max_compute_workgroups_per_dimension:
                        Direct3D12::D3D12_CS_DISPATCH_MAX_THREAD_GROUPS_PER_DIMENSION,
                    //
                    // NATIVE (Non-WebGPU) LIMITS:
                    //
                    max_non_sampler_bindings: 1_000_000,

                    max_binding_array_elements_per_shader_stage: full_heap_count,
                    max_binding_array_sampler_elements_per_shader_stage:
                        Direct3D12::D3D12_MAX_SHADER_VISIBLE_SAMPLER_HEAP_SIZE,

                    // Source: https://microsoft.github.io/DirectX-Specs/d3d/MeshShader.html#dispatchmesh-api
                    max_task_mesh_workgroup_total_count: if mesh_shader_supported {
                        2u32.pow(22)
                    } else {
                        0
                    },
                    // Technically it says "64k" but I highly doubt they want 65536 for compute and exactly 64,000 for task workgroups
                    max_task_mesh_workgroups_per_dimension: if mesh_shader_supported {
                        Direct3D12::D3D12_CS_DISPATCH_MAX_THREAD_GROUPS_PER_DIMENSION
                    } else {
                        0
                    },
                    // Assume this inherits from compute shaders
                    max_task_invocations_per_workgroup: if mesh_shader_supported {
                        Direct3D12::D3D12_CS_4_X_THREAD_GROUP_MAX_THREADS_PER_GROUP
                    } else {
                        0
                    },
                    max_task_invocations_per_dimension: if mesh_shader_supported {
                        Direct3D12::D3D12_CS_THREAD_GROUP_MAX_Z
                    } else {
                        0
                    },
                    // Source: https://microsoft.github.io/DirectX-Specs/d3d/MeshShader.html#amplification-shader-and-mesh-shader
                    max_mesh_invocations_per_workgroup: if mesh_shader_supported { 128 } else { 0 },
                    max_mesh_invocations_per_dimension: if mesh_shader_supported { 128 } else { 0 },

                    max_task_payload_size: if mesh_shader_supported { 16384 } else { 0 },
                    max_mesh_output_vertices: if mesh_shader_supported { 256 } else { 0 },
                    max_mesh_output_primitives: if mesh_shader_supported { 256 } else { 0 },
                    // Source: https://microsoft.github.io/DirectX-Specs/d3d/MeshShader.html#sv_rendertargetarrayindex-limitations-based-on-queryable-capability
                    max_mesh_output_layers: if mesh_shader_supported { 8 } else { 0 },
                    max_mesh_multiview_view_count: if mesh_shader_supported {
                        max_multiview_view_count
                    } else {
                        0
                    },

                    max_blas_primitive_count: if supports_ray_tracing {
                        1 << 29 // 2^29
                    } else {
                        0
                    },
                    max_blas_geometry_count: if supports_ray_tracing {
                        1 << 24 // 2^24
                    } else {
                        0
                    },
                    max_tlas_instance_count: if supports_ray_tracing {
                        1 << 24 // 2^24
                    } else {
                        0
                    },
                    max_acceleration_structures_per_shader_stage,
                    max_binding_array_acceleration_structure_elements_per_shader_stage:
                        max_acceleration_structures_per_shader_stage,
                    max_multiview_view_count,
                }),
                alignments: crate::Alignments {
                    buffer_copy_offset: wgt::BufferSize::new(
                        Direct3D12::D3D12_TEXTURE_DATA_PLACEMENT_ALIGNMENT as u64,
                    )
                    .unwrap(),
                    buffer_copy_pitch: wgt::BufferSize::new(
                        Direct3D12::D3D12_TEXTURE_DATA_PITCH_ALIGNMENT as u64,
                    )
                    .unwrap(),
                    // Direct3D correctly bounds-checks all array accesses:
                    // https://microsoft.github.io/DirectX-Specs/d3d/archive/D3D11_3_FunctionalSpec.htm#18.6.8.2%20Device%20Memory%20Reads
                    uniform_bounds_check_alignment: wgt::BufferSize::new(1).unwrap(),
                    raw_tlas_instance_size: size_of::<Direct3D12::D3D12_RAYTRACING_INSTANCE_DESC>(),
                    ray_tracing_scratch_buffer_alignment:
                        Direct3D12::D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BYTE_ALIGNMENT,
                },
                downlevel,
                cooperative_matrix_properties: Vec::new(),
            },
        })
    }
}

impl crate::Adapter for super::Adapter {
    type A = super::Api;

    unsafe fn open(
        &self,
        features: wgt::Features,
        limits: &wgt::Limits,
        memory_hints: &wgt::MemoryHints,
    ) -> Result<crate::OpenDevice<super::Api>, crate::DeviceError> {
        let queue: Direct3D12::ID3D12CommandQueue = {
            profiling::scope!("ID3D12Device::CreateCommandQueue");
            unsafe {
                self.device
                    .CreateCommandQueue(&Direct3D12::D3D12_COMMAND_QUEUE_DESC {
                        Type: Direct3D12::D3D12_COMMAND_LIST_TYPE_DIRECT,
                        Priority: Direct3D12::D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0,
                        Flags: Direct3D12::D3D12_COMMAND_QUEUE_FLAG_NONE,
                        NodeMask: 0,
                    })
            }
            .into_device_result("Queue creation")?
        };

        let device = super::Device::new(
            self.raw.clone(),
            self.device.clone(),
            queue.clone(),
            features,
            limits,
            memory_hints,
            self.private_caps,
            &self.library,
            &self.dcomp_lib,
            self.memory_budget_thresholds,
            self.compiler_container.clone(),
            self.options.clone(),
        )?;
        Ok(crate::OpenDevice {
            device,
            queue: super::Queue {
                raw: queue,
                temp_lists: Mutex::new(Vec::new()),
            },
        })
    }

    unsafe fn texture_format_capabilities(
        &self,
        format: wgt::TextureFormat,
    ) -> crate::TextureFormatCapabilities {
        use crate::TextureFormatCapabilities as Tfc;

        let raw_format = match auxil::dxgi::conv::map_texture_format_failable(format) {
            Some(f) => f,
            None => return Tfc::empty(),
        };
        let srv_uav_format = if format.is_combined_depth_stencil_format() {
            auxil::dxgi::conv::map_texture_format_for_srv_uav(
                format,
                // use the depth aspect here as opposed to stencil since it has more capabilities
                crate::FormatAspects::DEPTH,
            )
        } else {
            auxil::dxgi::conv::map_texture_format_for_srv_uav(
                format,
                crate::FormatAspects::from(format),
            )
        }
        .unwrap();

        let mut data = Direct3D12::D3D12_FEATURE_DATA_FORMAT_SUPPORT {
            Format: raw_format,
            ..Default::default()
        };
        unsafe {
            self.device.CheckFeatureSupport(
                Direct3D12::D3D12_FEATURE_FORMAT_SUPPORT,
                <*mut _>::cast(&mut data),
                size_of_val(&data) as u32,
            )
        }
        .unwrap();

        // Because we use a different format for SRV and UAV views of depth textures, we need to check
        // the features that use SRV/UAVs using the no-depth format.
        let mut data_srv_uav = Direct3D12::D3D12_FEATURE_DATA_FORMAT_SUPPORT {
            Format: srv_uav_format,
            Support1: Direct3D12::D3D12_FORMAT_SUPPORT1_NONE,
            Support2: Direct3D12::D3D12_FORMAT_SUPPORT2_NONE,
        };
        if raw_format != srv_uav_format {
            // Only-recheck if we're using a different format
            unsafe {
                self.device.CheckFeatureSupport(
                    Direct3D12::D3D12_FEATURE_FORMAT_SUPPORT,
                    ptr::addr_of_mut!(data_srv_uav).cast(),
                    size_of::<Direct3D12::D3D12_FEATURE_DATA_FORMAT_SUPPORT>() as u32,
                )
            }
            .unwrap();
        } else {
            // Same format, just copy over.
            data_srv_uav = data;
        }

        let mut caps = Tfc::COPY_SRC | Tfc::COPY_DST;
        // Cannot use the contains() helper, and windows-rs doesn't provide a .intersect() helper
        let is_texture = (data.Support1
            & (Direct3D12::D3D12_FORMAT_SUPPORT1_TEXTURE1D
                | Direct3D12::D3D12_FORMAT_SUPPORT1_TEXTURE2D
                | Direct3D12::D3D12_FORMAT_SUPPORT1_TEXTURE3D
                | Direct3D12::D3D12_FORMAT_SUPPORT1_TEXTURECUBE))
            .0
            != 0;
        // SRVs use srv_uav_format
        caps.set(
            Tfc::SAMPLED,
            is_texture
                && data_srv_uav
                    .Support1
                    .contains(Direct3D12::D3D12_FORMAT_SUPPORT1_SHADER_LOAD),
        );
        caps.set(
            Tfc::SAMPLED_LINEAR,
            data_srv_uav
                .Support1
                .contains(Direct3D12::D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE),
        );
        caps.set(
            Tfc::COLOR_ATTACHMENT,
            data.Support1
                .contains(Direct3D12::D3D12_FORMAT_SUPPORT1_RENDER_TARGET),
        );
        caps.set(
            Tfc::COLOR_ATTACHMENT_BLEND,
            data.Support1
                .contains(Direct3D12::D3D12_FORMAT_SUPPORT1_BLENDABLE),
        );
        caps.set(
            Tfc::DEPTH_STENCIL_ATTACHMENT,
            data.Support1
                .contains(Direct3D12::D3D12_FORMAT_SUPPORT1_DEPTH_STENCIL),
        );
        // UAVs use srv_uav_format
        caps.set(
            Tfc::STORAGE_READ_ONLY,
            data_srv_uav
                .Support2
                .contains(Direct3D12::D3D12_FORMAT_SUPPORT2_UAV_TYPED_LOAD),
        );
        caps.set(
            Tfc::STORAGE_ATOMIC,
            data_srv_uav
                .Support2
                .contains(Direct3D12::D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_UNSIGNED_MIN_OR_MAX),
        );
        caps.set(
            Tfc::STORAGE_WRITE_ONLY,
            data_srv_uav
                .Support2
                .contains(Direct3D12::D3D12_FORMAT_SUPPORT2_UAV_TYPED_STORE),
        );
        caps.set(
            Tfc::STORAGE_READ_WRITE,
            caps.contains(Tfc::STORAGE_READ_ONLY | Tfc::STORAGE_WRITE_ONLY),
        );

        // We load via UAV/SRV so use srv_uav_format
        let no_msaa_load = caps.contains(Tfc::SAMPLED)
            && !data_srv_uav
                .Support1
                .contains(Direct3D12::D3D12_FORMAT_SUPPORT1_MULTISAMPLE_LOAD);

        let no_msaa_target = (data.Support1
            & (Direct3D12::D3D12_FORMAT_SUPPORT1_RENDER_TARGET
                | Direct3D12::D3D12_FORMAT_SUPPORT1_DEPTH_STENCIL))
            .0
            != 0
            && !data
                .Support1
                .contains(Direct3D12::D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RENDERTARGET);

        caps.set(
            Tfc::MULTISAMPLE_RESOLVE,
            data.Support1
                .contains(Direct3D12::D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RESOLVE),
        );

        let mut ms_levels = Direct3D12::D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
            Format: raw_format,
            SampleCount: 0,
            Flags: Direct3D12::D3D12_MULTISAMPLE_QUALITY_LEVELS_FLAG_NONE,
            NumQualityLevels: 0,
        };

        let mut set_sample_count = |sc: u32, tfc: Tfc| {
            ms_levels.SampleCount = sc;

            if unsafe {
                self.device.CheckFeatureSupport(
                    Direct3D12::D3D12_FEATURE_MULTISAMPLE_QUALITY_LEVELS,
                    <*mut _>::cast(&mut ms_levels),
                    size_of_val(&ms_levels) as u32,
                )
            }
            .is_ok()
                && ms_levels.NumQualityLevels != 0
            {
                caps.set(tfc, !no_msaa_load && !no_msaa_target);
            }
        };

        set_sample_count(2, Tfc::MULTISAMPLE_X2);
        set_sample_count(4, Tfc::MULTISAMPLE_X4);
        set_sample_count(8, Tfc::MULTISAMPLE_X8);
        set_sample_count(16, Tfc::MULTISAMPLE_X16);

        caps
    }

    unsafe fn surface_capabilities(
        &self,
        surface: &super::Surface,
    ) -> Option<crate::SurfaceCapabilities> {
        let current_extent = {
            match surface.target {
                SurfaceTarget::WndHandle(wnd_handle)
                | SurfaceTarget::VisualFromWndHandle {
                    handle: wnd_handle, ..
                } => {
                    let mut rect = Default::default();
                    if unsafe { WindowsAndMessaging::GetClientRect(wnd_handle, &mut rect) }.is_ok()
                    {
                        Some(wgt::Extent3d {
                            width: (rect.right - rect.left) as u32,
                            height: (rect.bottom - rect.top) as u32,
                            depth_or_array_layers: 1,
                        })
                    } else {
                        log::warn!("Unable to get the window client rect");
                        None
                    }
                }
                SurfaceTarget::Visual(_)
                | SurfaceTarget::SurfaceHandle(_)
                | SurfaceTarget::SwapChainPanel(_) => None,
            }
        };

        let mut present_modes = vec![wgt::PresentMode::Mailbox, wgt::PresentMode::Fifo];
        if surface.supports_allow_tearing {
            present_modes.push(wgt::PresentMode::Immediate);
        }

        Some(crate::SurfaceCapabilities {
            formats: vec![
                wgt::TextureFormat::Bgra8UnormSrgb,
                wgt::TextureFormat::Bgra8Unorm,
                wgt::TextureFormat::Rgba8UnormSrgb,
                wgt::TextureFormat::Rgba8Unorm,
                wgt::TextureFormat::Rgb10a2Unorm,
                wgt::TextureFormat::Rgba16Float,
            ],
            // See https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgidevice1-setmaximumframelatency
            maximum_frame_latency: 1..=16,
            current_extent,
            usage: wgt::TextureUses::COLOR_TARGET
                | wgt::TextureUses::COPY_SRC
                | wgt::TextureUses::COPY_DST,
            present_modes,
            composite_alpha_modes: match surface.target {
                SurfaceTarget::WndHandle(_) => vec![wgt::CompositeAlphaMode::Opaque],
                SurfaceTarget::Visual(_)
                | SurfaceTarget::VisualFromWndHandle { .. }
                | SurfaceTarget::SurfaceHandle(_)
                | SurfaceTarget::SwapChainPanel(_) => vec![
                    wgt::CompositeAlphaMode::Auto,
                    wgt::CompositeAlphaMode::Inherit,
                    wgt::CompositeAlphaMode::Opaque,
                    wgt::CompositeAlphaMode::PostMultiplied,
                    wgt::CompositeAlphaMode::PreMultiplied,
                ],
            },
        })
    }

    unsafe fn get_presentation_timestamp(&self) -> wgt::PresentationTimestamp {
        wgt::PresentationTimestamp(self.presentation_timer.get_timestamp_ns())
    }

    fn get_ordered_buffer_usages(&self) -> wgt::BufferUses {
        wgt::BufferUses::INCLUSIVE | wgt::BufferUses::MAP_WRITE
    }

    // Don't put barriers between inclusive uses
    // DX12 implicitly orders renderpasses on the same resources.
    fn get_ordered_texture_usages(&self) -> wgt::TextureUses {
        wgt::TextureUses::INCLUSIVE
            | wgt::TextureUses::COLOR_TARGET
            | wgt::TextureUses::DEPTH_STENCIL_WRITE
    }
}

fn get_adapter_pci_info(vendor_id: u32, device_id: u32) -> String {
    // SAFETY: SetupDiGetClassDevsW is called with valid parameters
    let device_info_set = unsafe {
        match SetupDiGetClassDevsW(Some(&GUID_DEVCLASS_DISPLAY), None, None, DIGCF_PRESENT) {
            Ok(set) => set,
            Err(_) => return String::new(),
        }
    };

    struct DeviceInfoSetGuard(HDEVINFO);
    impl Drop for DeviceInfoSetGuard {
        fn drop(&mut self) {
            // SAFETY: device_info_set is a valid HDEVINFO and is only dropped once via this guard
            unsafe {
                let _ = SetupDiDestroyDeviceInfoList(self.0);
            }
        }
    }
    let _guard = DeviceInfoSetGuard(device_info_set);

    let mut device_index = 0u32;
    loop {
        let mut device_info_data = SP_DEVINFO_DATA {
            cbSize: size_of::<SP_DEVINFO_DATA>() as u32,
            ..Default::default()
        };

        // SAFETY: device_info_set is a valid HDEVINFO, device_index starts at 0 and
        // device_info_data is properly initialized above
        unsafe {
            if SetupDiEnumDeviceInfo(device_info_set, device_index, &mut device_info_data).is_err()
            {
                if GetLastError() == ERROR_NO_MORE_ITEMS {
                    break;
                }
                device_index += 1;
                continue;
            }
        }

        let mut hardware_id_size = 0u32;
        // SAFETY: device_info_set and device_info_data are valid
        unsafe {
            let _ = SetupDiGetDeviceRegistryPropertyW(
                device_info_set,
                &device_info_data,
                SPDRP_HARDWAREID,
                None,
                None,
                Some(&mut hardware_id_size),
            );
        }

        if hardware_id_size == 0 {
            device_index += 1;
            continue;
        }

        let mut hardware_id_buffer = vec![0u8; hardware_id_size as usize];
        // SAFETY: device_info_set and device_info_data are valid
        unsafe {
            if SetupDiGetDeviceRegistryPropertyW(
                device_info_set,
                &device_info_data,
                SPDRP_HARDWAREID,
                None,
                Some(&mut hardware_id_buffer),
                Some(&mut hardware_id_size),
            )
            .is_err()
            {
                device_index += 1;
                continue;
            }
        }

        let hardware_id_u16: Vec<u16> = hardware_id_buffer
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();
        let hardware_ids: Vec<String> = hardware_id_u16
            .split(|&c| c == 0)
            .filter(|s| !s.is_empty())
            .map(|s| String::from_utf16_lossy(s).to_uppercase())
            .collect();

        // https://learn.microsoft.com/en-us/windows-hardware/drivers/install/identifiers-for-pci-devices
        let expected_id = format!("PCI\\VEN_{vendor_id:04X}&DEV_{device_id:04X}");
        if !hardware_ids.iter().any(|id| id.contains(&expected_id)) {
            device_index += 1;
            continue;
        }

        let mut bus_buffer = [0u8; 4];
        let mut data_size = bus_buffer.len() as u32;
        // SAFETY: device_info_set and device_info_data are valid
        let bus_number = unsafe {
            if SetupDiGetDeviceRegistryPropertyW(
                device_info_set,
                &device_info_data,
                SPDRP_BUSNUMBER,
                None,
                Some(&mut bus_buffer),
                Some(&mut data_size),
            )
            .is_err()
            {
                device_index += 1;
                continue;
            }
            u32::from_le_bytes(bus_buffer)
        };

        let mut addr_buffer = [0u8; 4];
        let mut addr_size = addr_buffer.len() as u32;
        // SAFETY: device_info_set and device_info_data are valid
        unsafe {
            if SetupDiGetDeviceRegistryPropertyW(
                device_info_set,
                &device_info_data,
                SPDRP_ADDRESS,
                None,
                Some(&mut addr_buffer),
                Some(&mut addr_size),
            )
            .is_err()
            {
                device_index += 1;
                continue;
            }
        }
        let address = u32::from_le_bytes(addr_buffer);

        // https://learn.microsoft.com/en-us/windows-hardware/drivers/kernel/obtaining-device-configuration-information-at-irql---dispatch-level
        let device = (address >> 16) & 0x0000FFFF;
        let function = address & 0x0000FFFF;

        // domain:bus:device.function
        return format!("{:04x}:{:02x}:{:02x}.{:x}", 0, bus_number, device, function);
    }

    String::new()
}
