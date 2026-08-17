extern crate wgpu_hal as hal;

use hal::{
    Adapter as _, CommandEncoder as _, Device as _, Instance as _, Queue as _, Surface as _,
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use glam::{Affine3A, Mat4, Vec3};
use std::{
    borrow::{Borrow, Cow},
    iter, ptr,
    time::Instant,
};
use wgpu_types::Dx12BackendOptions;
use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Window, WindowButtons},
};

const DESIRED_MAX_LATENCY: u32 = 2;

/// [D3D12_RAYTRACING_INSTANCE_DESC](https://microsoft.github.io/DirectX-Specs/d3d/Raytracing.html#d3d12_raytracing_instance_desc)
/// [VkAccelerationStructureInstanceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureInstanceKHR.html)
#[derive(Clone)]
#[repr(C)]
struct AccelerationStructureInstance {
    transform: [f32; 12],
    custom_index_and_mask: u32,
    shader_binding_table_record_offset_and_flags: u32,
    acceleration_structure_reference: u64,
}

impl std::fmt::Debug for AccelerationStructureInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instance")
            .field("transform", &self.transform)
            .field("custom_data()", &self.custom_index())
            .field("mask()", &self.mask())
            .field(
                "shader_binding_table_record_offset()",
                &self.shader_binding_table_record_offset(),
            )
            .field("flags()", &self.flags())
            .field(
                "acceleration_structure_reference",
                &self.acceleration_structure_reference,
            )
            .finish()
    }
}

#[allow(dead_code)]
impl AccelerationStructureInstance {
    const LOW_24_MASK: u32 = 0x00ff_ffff;
    const MAX_U24: u32 = (1u32 << 24u32) - 1u32;

    #[inline]
    fn affine_to_rows(mat: &Affine3A) -> [f32; 12] {
        let row_0 = mat.matrix3.row(0);
        let row_1 = mat.matrix3.row(1);
        let row_2 = mat.matrix3.row(2);
        let translation = mat.translation;
        [
            row_0.x,
            row_0.y,
            row_0.z,
            translation.x,
            row_1.x,
            row_1.y,
            row_1.z,
            translation.y,
            row_2.x,
            row_2.y,
            row_2.z,
            translation.z,
        ]
    }

    #[inline]
    fn rows_to_affine(rows: &[f32; 12]) -> Affine3A {
        Affine3A::from_cols_array(&[
            rows[0], rows[3], rows[6], rows[9], rows[1], rows[4], rows[7], rows[10], rows[2],
            rows[5], rows[8], rows[11],
        ])
    }

    pub fn transform_as_affine(&self) -> Affine3A {
        Self::rows_to_affine(&self.transform)
    }
    pub fn set_transform(&mut self, transform: &Affine3A) {
        self.transform = Self::affine_to_rows(transform);
    }

    pub fn custom_index(&self) -> u32 {
        self.custom_index_and_mask & Self::LOW_24_MASK
    }

    pub fn mask(&self) -> u8 {
        (self.custom_index_and_mask >> 24) as u8
    }

    pub fn shader_binding_table_record_offset(&self) -> u32 {
        self.shader_binding_table_record_offset_and_flags & Self::LOW_24_MASK
    }

    pub fn flags(&self) -> u8 {
        (self.shader_binding_table_record_offset_and_flags >> 24) as u8
    }

    pub fn set_custom_index(&mut self, custom_index: u32) {
        debug_assert!(
            custom_index <= Self::MAX_U24,
            "custom_index uses more than 24 bits! {custom_index} > {}",
            Self::MAX_U24
        );
        self.custom_index_and_mask =
            (custom_index & Self::LOW_24_MASK) | (self.custom_index_and_mask & !Self::LOW_24_MASK)
    }

    pub fn set_mask(&mut self, mask: u8) {
        self.custom_index_and_mask =
            (self.custom_index_and_mask & Self::LOW_24_MASK) | (u32::from(mask) << 24)
    }

    pub fn set_shader_binding_table_record_offset(
        &mut self,
        shader_binding_table_record_offset: u32,
    ) {
        debug_assert!(
            shader_binding_table_record_offset <= Self::MAX_U24,
            "shader_binding_table_record_offset uses more than 24 bits! {} > {}",
            shader_binding_table_record_offset,
            Self::MAX_U24
        );
        self.shader_binding_table_record_offset_and_flags = (shader_binding_table_record_offset
            & Self::LOW_24_MASK)
            | (self.shader_binding_table_record_offset_and_flags & !Self::LOW_24_MASK)
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.shader_binding_table_record_offset_and_flags =
            (self.shader_binding_table_record_offset_and_flags & Self::LOW_24_MASK)
                | (u32::from(flags) << 24)
    }

    pub fn new(
        transform: &Affine3A,
        custom_index: u32,
        mask: u8,
        shader_binding_table_record_offset: u32,
        flags: u8,
        acceleration_structure_reference: u64,
    ) -> Self {
        debug_assert!(
            custom_index <= Self::MAX_U24,
            "custom_index uses more than 24 bits! {custom_index} > {}",
            Self::MAX_U24
        );
        debug_assert!(
            shader_binding_table_record_offset <= Self::MAX_U24,
            "shader_binding_table_record_offset uses more than 24 bits! {} > {}",
            shader_binding_table_record_offset,
            Self::MAX_U24
        );
        AccelerationStructureInstance {
            transform: Self::affine_to_rows(transform),
            custom_index_and_mask: (custom_index & Self::MAX_U24) | (u32::from(mask) << 24),
            shader_binding_table_record_offset_and_flags: (shader_binding_table_record_offset
                & Self::MAX_U24)
                | (u32::from(flags) << 24),
            acceleration_structure_reference,
        }
    }
}

struct ExecutionContext<A: hal::Api> {
    encoder: A::CommandEncoder,
    fence: A::Fence,
    fence_value: hal::FenceValue,
    used_views: Vec<A::TextureView>,
    used_cmd_bufs: Vec<A::CommandBuffer>,
    frames_recorded: usize,
}

impl<A: hal::Api> ExecutionContext<A> {
    unsafe fn wait_and_clear(&mut self, device: &A::Device) {
        device.wait(&self.fence, self.fence_value, None).unwrap();
        self.encoder.reset_all(self.used_cmd_bufs.drain(..));
        for view in self.used_views.drain(..) {
            device.destroy_texture_view(view);
        }
        self.frames_recorded = 0;
    }
}

#[allow(dead_code)]
struct Example<A: hal::Api> {
    instance: A::Instance,
    adapter: A::Adapter,
    surface: A::Surface,
    surface_format: wgpu_types::TextureFormat,
    device: A::Device,
    queue: A::Queue,

    contexts: Vec<ExecutionContext<A>>,
    context_index: usize,
    extent: [u32; 2],
    start: Instant,
    pipeline: A::ComputePipeline,
    bind_group: A::BindGroup,
    bgl: A::BindGroupLayout,
    shader_module: A::ShaderModule,
    texture_view: A::TextureView,
    uniform_buffer: A::Buffer,
    pipeline_layout: A::PipelineLayout,
    vertices_buffer: A::Buffer,
    indices_buffer: Option<A::Buffer>,
    texture: A::Texture,
    instances: [AccelerationStructureInstance; 3],
    instances_buffer: A::Buffer,
    blas: A::AccelerationStructure,
    tlas: A::AccelerationStructure,
    scratch_buffer: A::Buffer,
    time: f32,
}

impl<A: hal::Api> Example<A> {
    fn init(window: &winit::window::Window) -> Result<Self, Box<dyn std::error::Error>> {
        let mut index_buffer = false;

        for arg in std::env::args() {
            if arg == "index_buffer" {
                index_buffer = true;
            }
        }

        if index_buffer {
            log::info!("using index buffer")
        }

        // The Instance can be initialized with the DisplayHandle from the EventLoop as well
        let raw_display_handle = window.display_handle()?;

        let instance_desc = hal::InstanceDescriptor {
            name: "example",
            flags: wgpu_types::InstanceFlags::default(),
            memory_budget_thresholds: wgpu_types::MemoryBudgetThresholds::default(),
            backend_options: wgpu_types::BackendOptions {
                dx12: Dx12BackendOptions {
                    shader_compiler: wgpu_types::Dx12Compiler::default_dynamic_dxc(),
                    ..Default::default()
                },
                ..Default::default()
            },
            telemetry: None,
            display: Some(raw_display_handle),
        };
        let instance = unsafe { A::Instance::init(&instance_desc)? };
        let surface = {
            let raw_window_handle = window.window_handle()?.as_raw();

            unsafe {
                instance
                    .create_surface(raw_display_handle.as_raw(), raw_window_handle)
                    .unwrap()
            }
        };

        let (adapter, features) = unsafe {
            let mut adapters = instance.enumerate_adapters(Some(&surface));
            if adapters.is_empty() {
                panic!("No adapters found");
            }
            let exposed = adapters.swap_remove(0);
            dbg!(exposed.features);
            (exposed.adapter, exposed.features)
        };
        let surface_caps = unsafe { adapter.surface_capabilities(&surface) }
            .expect("Surface doesn't support presentation");
        log::info!("Surface caps: {surface_caps:#?}");

        let hal::OpenDevice { device, queue } = unsafe {
            adapter
                .open(
                    features,
                    &wgpu_types::Limits::default(),
                    &wgpu_types::MemoryHints::Performance,
                )
                .unwrap()
        };

        let window_size: (u32, u32) = window.inner_size().into();
        dbg!(&surface_caps.formats);
        let surface_format = if surface_caps
            .formats
            .contains(&wgpu_types::TextureFormat::Rgba8Unorm)
        {
            wgpu_types::TextureFormat::Rgba8Unorm
        } else {
            *surface_caps.formats.first().unwrap()
        };
        let surface_config = hal::SurfaceConfiguration {
            maximum_frame_latency: DESIRED_MAX_LATENCY
                .max(*surface_caps.maximum_frame_latency.start())
                .min(*surface_caps.maximum_frame_latency.end()),
            present_mode: wgpu_types::PresentMode::Fifo,
            composite_alpha_mode: wgpu_types::CompositeAlphaMode::Opaque,
            format: surface_format,
            extent: wgpu_types::Extent3d {
                width: window_size.0,
                height: window_size.1,
                depth_or_array_layers: 1,
            },
            usage: wgpu_types::TextureUses::COLOR_TARGET | wgpu_types::TextureUses::COPY_DST,
            view_formats: vec![surface_format],
        };
        unsafe {
            surface.configure(&device, &surface_config).unwrap();
        };

        #[allow(dead_code)]
        struct Uniforms {
            view_inverse: glam::Mat4,
            proj_inverse: glam::Mat4,
        }

        let bgl_desc = hal::BindGroupLayoutDescriptor {
            label: None,
            flags: hal::BindGroupLayoutFlags::empty(),
            entries: &[
                wgpu_types::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu_types::ShaderStages::COMPUTE,
                    ty: wgpu_types::BindingType::Buffer {
                        ty: wgpu_types::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu_types::BufferSize::new(size_of::<Uniforms>() as _),
                    },
                    count: None,
                },
                wgpu_types::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu_types::ShaderStages::COMPUTE,
                    ty: wgpu_types::BindingType::StorageTexture {
                        access: wgpu_types::StorageTextureAccess::WriteOnly,
                        format: wgpu_types::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu_types::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu_types::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu_types::ShaderStages::COMPUTE,
                    ty: wgpu_types::BindingType::AccelerationStructure {
                        vertex_return: false,
                    },
                    count: None,
                },
            ],
        };

        let bgl = unsafe { device.create_bind_group_layout(&bgl_desc).unwrap() };

        let naga_shader = {
            let shader_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("examples")
                .join("ray-traced-triangle")
                .join("shader.wgsl");
            let source = std::fs::read_to_string(shader_file).unwrap();
            let module = naga::front::wgsl::Frontend::new().parse(&source).unwrap();
            let info = naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                naga::valid::Capabilities::RAY_QUERY,
            )
            .validate(&module)
            .unwrap();
            hal::NagaShader {
                module: Cow::Owned(module),
                info,
                debug_source: None,
            }
        };
        let shader_desc = hal::ShaderModuleDescriptor {
            label: None,
            runtime_checks: wgpu_types::ShaderRuntimeChecks::checked(),
        };
        let shader_module = unsafe {
            device
                .create_shader_module(&shader_desc, hal::ShaderInput::Naga(naga_shader))
                .unwrap()
        };

        let pipeline_layout_desc = hal::PipelineLayoutDescriptor {
            label: None,
            flags: hal::PipelineLayoutFlags::empty(),
            bind_group_layouts: &[Some(&bgl)],
            immediate_size: 0,
        };
        let pipeline_layout = unsafe {
            device
                .create_pipeline_layout(&pipeline_layout_desc)
                .unwrap()
        };

        let pipeline = unsafe {
            device.create_compute_pipeline(&hal::ComputePipelineDescriptor {
                label: Some("pipeline"),
                layout: &pipeline_layout,
                stage: hal::ProgrammableStage {
                    module: &shader_module,
                    entry_point: "main",
                    constants: &Default::default(),
                    zero_initialize_workgroup_memory: true,
                },
                cache: None,
            })
        }
        .unwrap();

        let vertices: [f32; 9] = [1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 0.0, -1.0, 0.0];

        let vertices_size_in_bytes = vertices.len() * 4;

        let indices: [u32; 3] = [0, 1, 2];

        let indices_size_in_bytes = indices.len() * 4;

        let vertices_buffer = unsafe {
            let vertices_buffer = device
                .create_buffer(&hal::BufferDescriptor {
                    label: Some("vertices buffer"),
                    size: vertices_size_in_bytes as u64,
                    usage: wgpu_types::BufferUses::MAP_WRITE
                        | wgpu_types::BufferUses::BOTTOM_LEVEL_ACCELERATION_STRUCTURE_INPUT,
                    memory_flags: hal::MemoryFlags::TRANSIENT | hal::MemoryFlags::PREFER_COHERENT,
                })
                .unwrap();

            let mapping = device
                .map_buffer(&vertices_buffer, 0..vertices_size_in_bytes as u64)
                .unwrap();
            ptr::copy_nonoverlapping(
                vertices.as_ptr() as *const u8,
                mapping.ptr.as_ptr(),
                vertices_size_in_bytes,
            );
            device.unmap_buffer(&vertices_buffer);
            assert!(mapping.is_coherent);

            vertices_buffer
        };

        let indices_buffer = if index_buffer {
            unsafe {
                let indices_buffer = device
                    .create_buffer(&hal::BufferDescriptor {
                        label: Some("indices buffer"),
                        size: indices_size_in_bytes as u64,
                        usage: wgpu_types::BufferUses::MAP_WRITE
                            | wgpu_types::BufferUses::BOTTOM_LEVEL_ACCELERATION_STRUCTURE_INPUT,
                        memory_flags: hal::MemoryFlags::TRANSIENT
                            | hal::MemoryFlags::PREFER_COHERENT,
                    })
                    .unwrap();

                let mapping = device
                    .map_buffer(&indices_buffer, 0..indices_size_in_bytes as u64)
                    .unwrap();
                ptr::copy_nonoverlapping(
                    indices.as_ptr() as *const u8,
                    mapping.ptr.as_ptr(),
                    indices_size_in_bytes,
                );
                device.unmap_buffer(&indices_buffer);
                assert!(mapping.is_coherent);

                Some((indices_buffer, indices.len()))
            }
        } else {
            None
        };

        let blas_triangles = vec![hal::AccelerationStructureTriangles {
            vertex_buffer: Some(&vertices_buffer),
            first_vertex: 0,
            vertex_format: wgpu_types::VertexFormat::Float32x3,
            // each vertex is 3 floats, and floats are stored raw in the array
            vertex_count: vertices.len() as u32 / 3,
            vertex_stride: 3 * 4,
            indices: indices_buffer.as_ref().map(|(buf, len)| {
                hal::AccelerationStructureTriangleIndices {
                    buffer: Some(buf),
                    format: wgpu_types::IndexFormat::Uint32,
                    offset: 0,
                    count: *len as u32,
                }
            }),

            transform: None,
            flags: hal::AccelerationStructureGeometryFlags::OPAQUE,
        }];
        let blas_entries = hal::AccelerationStructureEntries::Triangles(blas_triangles);

        let blas_sizes = unsafe {
            device.get_acceleration_structure_build_sizes(
                &hal::GetAccelerationStructureBuildSizesDescriptor {
                    entries: &blas_entries,
                    flags: hal::AccelerationStructureBuildFlags::PREFER_FAST_TRACE,
                },
            )
        };

        let blas = unsafe {
            device.create_acceleration_structure(&hal::AccelerationStructureDescriptor {
                label: Some("blas"),
                size: blas_sizes.acceleration_structure_size,
                format: hal::AccelerationStructureFormat::BottomLevel,
                allow_compaction: false,
            })
        }
        .unwrap();

        let instances = [
            AccelerationStructureInstance::new(
                &Affine3A::from_translation(Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }),
                0,
                0xff,
                0,
                0,
                unsafe { device.get_acceleration_structure_device_address(&blas) },
            ),
            AccelerationStructureInstance::new(
                &Affine3A::from_translation(Vec3 {
                    x: -1.0,
                    y: -1.0,
                    z: -2.0,
                }),
                0,
                0xff,
                0,
                0,
                unsafe { device.get_acceleration_structure_device_address(&blas) },
            ),
            AccelerationStructureInstance::new(
                &Affine3A::from_translation(Vec3 {
                    x: 1.0,
                    y: -1.0,
                    z: -2.0,
                }),
                0,
                0xff,
                0,
                0,
                unsafe { device.get_acceleration_structure_device_address(&blas) },
            ),
        ];

        let instances_buffer_size = instances.len() * size_of::<AccelerationStructureInstance>();

        let instances_buffer = unsafe {
            let instances_buffer = device
                .create_buffer(&hal::BufferDescriptor {
                    label: Some("instances_buffer"),
                    size: instances_buffer_size as u64,
                    usage: wgpu_types::BufferUses::MAP_WRITE
                        | wgpu_types::BufferUses::TOP_LEVEL_ACCELERATION_STRUCTURE_INPUT,
                    memory_flags: hal::MemoryFlags::TRANSIENT | hal::MemoryFlags::PREFER_COHERENT,
                })
                .unwrap();

            let mapping = device
                .map_buffer(&instances_buffer, 0..instances_buffer_size as u64)
                .unwrap();
            ptr::copy_nonoverlapping(
                instances.as_ptr() as *const u8,
                mapping.ptr.as_ptr(),
                instances_buffer_size,
            );
            device.unmap_buffer(&instances_buffer);
            assert!(mapping.is_coherent);

            instances_buffer
        };

        let mut tlas_entries =
            hal::AccelerationStructureEntries::Instances(hal::AccelerationStructureInstances {
                buffer: Some(&instances_buffer),
                count: 3,
                offset: 0,
            });

        let tlas_flags = hal::AccelerationStructureBuildFlags::PREFER_FAST_TRACE
            | hal::AccelerationStructureBuildFlags::ALLOW_UPDATE;

        let tlas_sizes = unsafe {
            device.get_acceleration_structure_build_sizes(
                &hal::GetAccelerationStructureBuildSizesDescriptor {
                    entries: &tlas_entries,
                    flags: tlas_flags,
                },
            )
        };

        let tlas = unsafe {
            device.create_acceleration_structure(&hal::AccelerationStructureDescriptor {
                label: Some("tlas"),
                size: tlas_sizes.acceleration_structure_size,
                format: hal::AccelerationStructureFormat::TopLevel,
                allow_compaction: false,
            })
        }
        .unwrap();

        let uniforms = {
            let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 2.5), Vec3::ZERO, Vec3::Y);
            let proj = Mat4::perspective_rh(59.0_f32.to_radians(), 1.0, 0.001, 1000.0);

            Uniforms {
                view_inverse: view.inverse(),
                proj_inverse: proj.inverse(),
            }
        };

        let uniforms_size = size_of::<Uniforms>();

        let uniform_buffer = unsafe {
            let uniform_buffer = device
                .create_buffer(&hal::BufferDescriptor {
                    label: Some("uniform buffer"),
                    size: uniforms_size as u64,
                    usage: wgpu_types::BufferUses::MAP_WRITE | wgpu_types::BufferUses::UNIFORM,
                    memory_flags: hal::MemoryFlags::PREFER_COHERENT,
                })
                .unwrap();

            let mapping = device
                .map_buffer(&uniform_buffer, 0..uniforms_size as u64)
                .unwrap();
            ptr::copy_nonoverlapping(
                &uniforms as *const Uniforms as *const u8,
                mapping.ptr.as_ptr(),
                uniforms_size,
            );
            device.unmap_buffer(&uniform_buffer);
            assert!(mapping.is_coherent);
            uniform_buffer
        };

        let texture_desc = hal::TextureDescriptor {
            label: None,
            size: wgpu_types::Extent3d {
                width: 512,
                height: 512,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu_types::TextureDimension::D2,
            format: wgpu_types::TextureFormat::Rgba8Unorm,
            usage: wgpu_types::TextureUses::STORAGE_READ_WRITE | wgpu_types::TextureUses::COPY_SRC,
            memory_flags: hal::MemoryFlags::empty(),
            view_formats: vec![wgpu_types::TextureFormat::Rgba8Unorm],
        };
        let texture = unsafe { device.create_texture(&texture_desc).unwrap() };

        let view_desc = hal::TextureViewDescriptor {
            label: None,
            format: texture_desc.format,
            dimension: wgpu_types::TextureViewDimension::D2,
            usage: wgpu_types::TextureUses::STORAGE_READ_WRITE | wgpu_types::TextureUses::COPY_SRC,
            range: wgpu_types::ImageSubresourceRange::default(),
        };
        let texture_view = unsafe { device.create_texture_view(&texture, &view_desc).unwrap() };

        let bind_group = {
            let buffer_binding = unsafe {
                // SAFETY: The size matches the buffer allocation.
                hal::BufferBinding::new_unchecked(
                    &uniform_buffer,
                    0,
                    wgpu_types::BufferSize::new_unchecked(uniforms_size as u64),
                )
            };
            let texture_binding = hal::TextureBinding {
                view: &texture_view,
                usage: wgpu_types::TextureUses::STORAGE_READ_WRITE,
            };
            let group_desc = hal::BindGroupDescriptor {
                label: Some("bind group"),
                layout: &bgl,
                buffers: &[buffer_binding],
                samplers: &[],
                textures: &[texture_binding],
                acceleration_structures: &[&tlas],
                external_textures: &[],
                entries: &[
                    hal::BindGroupEntry {
                        binding: 0,
                        resource_index: 0,
                        count: 1,
                    },
                    hal::BindGroupEntry {
                        binding: 1,
                        resource_index: 0,
                        count: 1,
                    },
                    hal::BindGroupEntry {
                        binding: 2,
                        resource_index: 0,
                        count: 1,
                    },
                ],
            };
            unsafe { device.create_bind_group(&group_desc).unwrap() }
        };

        let scratch_buffer = unsafe {
            device
                .create_buffer(&hal::BufferDescriptor {
                    label: Some("scratch buffer"),
                    size: blas_sizes
                        .build_scratch_size
                        .max(tlas_sizes.build_scratch_size),
                    usage: wgpu_types::BufferUses::ACCELERATION_STRUCTURE_SCRATCH,
                    memory_flags: hal::MemoryFlags::empty(),
                })
                .unwrap()
        };

        if let hal::AccelerationStructureEntries::Instances(ref mut i) = tlas_entries {
            assert!(
                instances.len() <= i.count as usize,
                "Tlas allocation to small"
            );
        }

        let cmd_encoder_desc = hal::CommandEncoderDescriptor {
            label: None,
            queue: &queue,
        };
        let mut cmd_encoder = unsafe { device.create_command_encoder(&cmd_encoder_desc).unwrap() };

        unsafe { cmd_encoder.begin_encoding(Some("init")).unwrap() };

        unsafe {
            cmd_encoder.place_acceleration_structure_barrier(hal::AccelerationStructureBarrier {
                usage: hal::StateTransition {
                    from: hal::AccelerationStructureUses::empty(),
                    to: hal::AccelerationStructureUses::BUILD_OUTPUT,
                },
            });

            cmd_encoder.build_acceleration_structures(
                1,
                [hal::BuildAccelerationStructureDescriptor {
                    mode: hal::AccelerationStructureBuildMode::Build,
                    flags: hal::AccelerationStructureBuildFlags::PREFER_FAST_TRACE,
                    destination_acceleration_structure: &blas,
                    scratch_buffer: &scratch_buffer,
                    entries: &blas_entries,
                    source_acceleration_structure: None,
                    scratch_buffer_offset: 0,
                }],
            );

            let scratch_buffer_barrier = hal::BufferBarrier {
                buffer: &scratch_buffer,
                usage: hal::StateTransition {
                    from: wgpu_types::BufferUses::BOTTOM_LEVEL_ACCELERATION_STRUCTURE_INPUT,
                    to: wgpu_types::BufferUses::TOP_LEVEL_ACCELERATION_STRUCTURE_INPUT,
                },
            };
            cmd_encoder.transition_buffers(iter::once(scratch_buffer_barrier));

            cmd_encoder.place_acceleration_structure_barrier(hal::AccelerationStructureBarrier {
                usage: hal::StateTransition {
                    from: hal::AccelerationStructureUses::BUILD_OUTPUT,
                    to: hal::AccelerationStructureUses::BUILD_INPUT,
                },
            });

            cmd_encoder.build_acceleration_structures(
                1,
                [hal::BuildAccelerationStructureDescriptor {
                    mode: hal::AccelerationStructureBuildMode::Build,
                    flags: tlas_flags,
                    destination_acceleration_structure: &tlas,
                    scratch_buffer: &scratch_buffer,
                    entries: &tlas_entries,
                    source_acceleration_structure: None,
                    scratch_buffer_offset: 0,
                }],
            );

            cmd_encoder.place_acceleration_structure_barrier(hal::AccelerationStructureBarrier {
                usage: hal::StateTransition {
                    from: hal::AccelerationStructureUses::BUILD_OUTPUT,
                    to: hal::AccelerationStructureUses::SHADER_INPUT,
                },
            });

            let texture_barrier = hal::TextureBarrier {
                texture: &texture,
                range: wgpu_types::ImageSubresourceRange::default(),
                usage: hal::StateTransition {
                    from: wgpu_types::TextureUses::UNINITIALIZED,
                    to: wgpu_types::TextureUses::STORAGE_READ_WRITE,
                },
            };

            cmd_encoder.transition_textures(iter::once(texture_barrier));
        }

        let init_fence_value = 1;
        let fence = unsafe {
            let mut fence = device.create_fence().unwrap();
            let init_cmd = cmd_encoder.end_encoding().unwrap();
            queue
                .submit(&[&init_cmd], &[], (&mut fence, init_fence_value))
                .unwrap();
            device.wait(&fence, init_fence_value, None).unwrap();
            cmd_encoder.reset_all(iter::once(init_cmd));
            fence
        };

        Ok(Self {
            instance,
            adapter,
            surface,
            surface_format: surface_config.format,
            device,
            queue,
            pipeline,
            contexts: vec![ExecutionContext {
                encoder: cmd_encoder,
                fence,
                fence_value: init_fence_value + 1,
                used_views: Vec::new(),
                used_cmd_bufs: Vec::new(),
                frames_recorded: 0,
            }],
            context_index: 0,
            extent: [window_size.0, window_size.1],
            start: Instant::now(),
            pipeline_layout,
            bind_group,
            texture,
            instances,
            instances_buffer,
            blas,
            tlas,
            scratch_buffer,
            time: 0.0,
            indices_buffer: indices_buffer.map(|(buf, _)| buf),
            vertices_buffer,
            uniform_buffer,
            texture_view,
            bgl,
            shader_module,
        })
    }

    fn update(&mut self, _event: winit::event::WindowEvent) {}

    fn render(&mut self) {
        let ctx = &mut self.contexts[self.context_index];

        let surface_tex = unsafe {
            self.surface
                .acquire_texture(None, &ctx.fence)
                .unwrap()
                .texture
        };

        let target_barrier0 = hal::TextureBarrier {
            texture: surface_tex.borrow(),
            range: wgpu_types::ImageSubresourceRange::default(),
            usage: hal::StateTransition {
                from: wgpu_types::TextureUses::UNINITIALIZED,
                to: wgpu_types::TextureUses::COPY_DST,
            },
        };

        let instances_buffer_size =
            self.instances.len() * size_of::<AccelerationStructureInstance>();

        let tlas_flags = hal::AccelerationStructureBuildFlags::PREFER_FAST_TRACE
            | hal::AccelerationStructureBuildFlags::ALLOW_UPDATE;

        self.time += 1.0 / 60.0;

        self.instances[0].set_transform(&Affine3A::from_rotation_y(self.time));

        unsafe {
            let mapping = self
                .device
                .map_buffer(&self.instances_buffer, 0..instances_buffer_size as u64)
                .unwrap();
            ptr::copy_nonoverlapping(
                self.instances.as_ptr() as *const u8,
                mapping.ptr.as_ptr(),
                instances_buffer_size,
            );
            self.device.unmap_buffer(&self.instances_buffer);
            assert!(mapping.is_coherent);
        }

        unsafe {
            ctx.encoder.begin_encoding(Some("frame")).unwrap();

            let instances = hal::AccelerationStructureInstances {
                buffer: Some(&self.instances_buffer),
                count: self.instances.len() as u32,
                offset: 0,
            };

            ctx.encoder
                .place_acceleration_structure_barrier(hal::AccelerationStructureBarrier {
                    usage: hal::StateTransition {
                        from: hal::AccelerationStructureUses::SHADER_INPUT,
                        to: hal::AccelerationStructureUses::BUILD_INPUT,
                    },
                });

            ctx.encoder.build_acceleration_structures(
                1,
                [hal::BuildAccelerationStructureDescriptor {
                    mode: hal::AccelerationStructureBuildMode::Update,
                    flags: tlas_flags,
                    destination_acceleration_structure: &self.tlas,
                    scratch_buffer: &self.scratch_buffer,
                    entries: &hal::AccelerationStructureEntries::Instances(instances),
                    source_acceleration_structure: Some(&self.tlas),
                    scratch_buffer_offset: 0,
                }],
            );

            ctx.encoder
                .place_acceleration_structure_barrier(hal::AccelerationStructureBarrier {
                    usage: hal::StateTransition {
                        from: hal::AccelerationStructureUses::BUILD_OUTPUT,
                        to: hal::AccelerationStructureUses::SHADER_INPUT,
                    },
                });

            let scratch_buffer_barrier = hal::BufferBarrier {
                buffer: &self.scratch_buffer,
                usage: hal::StateTransition {
                    from: wgpu_types::BufferUses::BOTTOM_LEVEL_ACCELERATION_STRUCTURE_INPUT,
                    to: wgpu_types::BufferUses::TOP_LEVEL_ACCELERATION_STRUCTURE_INPUT,
                },
            };
            ctx.encoder
                .transition_buffers(iter::once(scratch_buffer_barrier));

            ctx.encoder.transition_textures(iter::once(target_barrier0));
        }

        let surface_view_desc = hal::TextureViewDescriptor {
            label: None,
            format: self.surface_format,
            dimension: wgpu_types::TextureViewDimension::D2,
            usage: wgpu_types::TextureUses::COPY_DST,
            range: wgpu_types::ImageSubresourceRange::default(),
        };
        let surface_tex_view = unsafe {
            self.device
                .create_texture_view(surface_tex.borrow(), &surface_view_desc)
                .unwrap()
        };
        unsafe {
            ctx.encoder.begin_compute_pass(&hal::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            ctx.encoder.set_compute_pipeline(&self.pipeline);
            ctx.encoder
                .set_bind_group(&self.pipeline_layout, 0, &self.bind_group, &[]);
            ctx.encoder.dispatch([512 / 8, 512 / 8, 1]);
        }

        ctx.frames_recorded += 1;

        let target_barrier1 = hal::TextureBarrier {
            texture: surface_tex.borrow(),
            range: wgpu_types::ImageSubresourceRange::default(),
            usage: hal::StateTransition {
                from: wgpu_types::TextureUses::COPY_DST,
                to: wgpu_types::TextureUses::PRESENT,
            },
        };
        let target_barrier2 = hal::TextureBarrier {
            texture: &self.texture,
            range: wgpu_types::ImageSubresourceRange::default(),
            usage: hal::StateTransition {
                from: wgpu_types::TextureUses::STORAGE_READ_WRITE,
                to: wgpu_types::TextureUses::COPY_SRC,
            },
        };
        let target_barrier3 = hal::TextureBarrier {
            texture: &self.texture,
            range: wgpu_types::ImageSubresourceRange::default(),
            usage: hal::StateTransition {
                from: wgpu_types::TextureUses::COPY_SRC,
                to: wgpu_types::TextureUses::STORAGE_READ_WRITE,
            },
        };
        unsafe {
            ctx.encoder.end_compute_pass();
            ctx.encoder.transition_textures(iter::once(target_barrier2));
            ctx.encoder.copy_texture_to_texture(
                &self.texture,
                wgpu_types::TextureUses::COPY_SRC,
                surface_tex.borrow(),
                std::iter::once(hal::TextureCopy {
                    src_base: hal::TextureCopyBase {
                        mip_level: 0,
                        array_layer: 0,
                        origin: wgpu_types::Origin3d::ZERO,
                        aspect: hal::FormatAspects::COLOR,
                    },
                    dst_base: hal::TextureCopyBase {
                        mip_level: 0,
                        array_layer: 0,
                        origin: wgpu_types::Origin3d::ZERO,
                        aspect: hal::FormatAspects::COLOR,
                    },
                    size: hal::CopyExtent {
                        width: 512,
                        height: 512,
                        depth: 1,
                    },
                }),
            );
            ctx.encoder.transition_textures(iter::once(target_barrier1));
            ctx.encoder.transition_textures(iter::once(target_barrier3));
        }

        unsafe {
            let cmd_buf = ctx.encoder.end_encoding().unwrap();
            self.queue
                .submit(
                    &[&cmd_buf],
                    &[&surface_tex],
                    (&mut ctx.fence, ctx.fence_value),
                )
                .unwrap();
            self.queue.present(&self.surface, surface_tex).unwrap();
            ctx.used_cmd_bufs.push(cmd_buf);
            ctx.used_views.push(surface_tex_view);
        };

        log::info!("Context switch from {}", self.context_index);
        let old_fence_value = ctx.fence_value;
        if self.contexts.len() == 1 {
            let hal_desc = hal::CommandEncoderDescriptor {
                label: None,
                queue: &self.queue,
            };
            self.contexts.push(unsafe {
                ExecutionContext {
                    encoder: self.device.create_command_encoder(&hal_desc).unwrap(),
                    fence: self.device.create_fence().unwrap(),
                    fence_value: 0,
                    used_views: Vec::new(),
                    used_cmd_bufs: Vec::new(),
                    frames_recorded: 0,
                }
            });
        }
        self.context_index = (self.context_index + 1) % self.contexts.len();
        let next = &mut self.contexts[self.context_index];
        unsafe {
            next.wait_and_clear(&self.device);
        }
        next.fence_value = old_fence_value + 1;
    }

    fn exit(mut self) {
        unsafe {
            {
                let ctx = &mut self.contexts[self.context_index];
                self.queue
                    .submit(&[], &[], (&mut ctx.fence, ctx.fence_value))
                    .unwrap();
            }

            for mut ctx in self.contexts {
                ctx.wait_and_clear(&self.device);
                drop(ctx.encoder);
                self.device.destroy_fence(ctx.fence);
            }

            self.device.destroy_bind_group(self.bind_group);
            self.device.destroy_buffer(self.scratch_buffer);
            self.device.destroy_buffer(self.instances_buffer);
            if let Some(buffer) = self.indices_buffer {
                self.device.destroy_buffer(buffer);
            }
            self.device.destroy_buffer(self.vertices_buffer);
            self.device.destroy_buffer(self.uniform_buffer);
            self.device.destroy_acceleration_structure(self.tlas);
            self.device.destroy_acceleration_structure(self.blas);
            self.device.destroy_texture_view(self.texture_view);
            self.device.destroy_texture(self.texture);
            self.device.destroy_compute_pipeline(self.pipeline);
            self.device.destroy_pipeline_layout(self.pipeline_layout);
            self.device.destroy_bind_group_layout(self.bgl);
            self.device.destroy_shader_module(self.shader_module);

            self.surface.unconfigure(&self.device);
            drop(self.queue);
            drop(self.device);
            drop(self.surface);
            drop(self.adapter);
        }
    }
}

cfg_if::cfg_if! {
    // Apple + Metal
    if #[cfg(all(target_vendor = "apple", feature = "metal"))] {
        type Api = hal::api::Metal;
    }
    // Wasm + Vulkan
    else if #[cfg(all(not(target_arch = "wasm32"), feature = "vulkan"))] {
        type Api = hal::api::Vulkan;
    }
    // Windows + DX12
    else if #[cfg(all(windows, feature = "dx12"))] {
        type Api = hal::api::Dx12;
    }
    // Anything + GLES
    else if #[cfg(feature = "gles")] {
        type Api = hal::api::Gles;
    }
    // Fallback
    else {
        type Api = hal::api::Noop;
    }
}

struct App {
    example: Option<Example<Api>>,
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("hal-ray-traced-triangle")
                    .with_inner_size(winit::dpi::PhysicalSize {
                        width: 512,
                        height: 512,
                    })
                    .with_resizable(false)
                    .with_enabled_buttons(WindowButtons::CLOSE),
            )
            .unwrap();
        let example = Example::<Api>::init(&window).expect("Selected backend is not supported");
        self.window = Some(window);
        self.example = Some(example);
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.example.take().unwrap().exit();
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        event_loop.set_control_flow(ControlFlow::Poll);

        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::KeyboardInput { event, .. }
                if event.physical_key
                    == winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape) =>
            {
                event_loop.exit();
            }
            winit::event::WindowEvent::RedrawRequested => {
                let ex = self.example.as_mut().unwrap();
                ex.render();
            }
            _ => {
                self.example.as_mut().unwrap().update(event);
            }
        }
    }
}

fn main() {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let mut app = App {
        example: None,
        window: None,
    };
    event_loop.run_app(&mut app).unwrap();
}
