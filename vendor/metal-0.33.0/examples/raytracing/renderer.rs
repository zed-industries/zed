use std::{
    collections::BTreeMap,
    ops::Index,
    sync::{Arc, Condvar, Mutex},
};

use core_graphics_types::{base::CGFloat, geometry::CGSize};
use glam::{Vec3, Vec4, Vec4Swizzles};
use metal::{foreign_types::ForeignType, *};
use rand::RngCore;

use crate::{camera::Camera, geometry::get_managed_buffer_storage_mode, scene::Scene};

#[repr(C)]
struct Uniforms {
    pub width: u32,
    pub height: u32,
    pub frame_index: u32,
    pub light_count: u32,
    pub camera: Camera,
}

pub const MAX_FRAMES_IN_FLIGHT: NSUInteger = 3;
pub const ALIGNED_UNIFORMS_SIZE: NSUInteger = (size_of::<Uniforms>() as NSUInteger + 255) & !255;
pub const UNIFORM_BUFFER_SIZE: NSUInteger = MAX_FRAMES_IN_FLIGHT * ALIGNED_UNIFORMS_SIZE;

#[derive(Clone)]
struct Semaphore {
    data: Arc<(Mutex<usize>, Condvar)>,
}

impl Semaphore {
    fn new(capacity: usize) -> Self {
        Self {
            data: Arc::new((Mutex::new(capacity), Condvar::new())),
        }
    }

    fn acquire(&self) {
        let mut value = self.data.0.lock().unwrap();
        while *value == 0 {
            value = self.data.1.wait(value).unwrap();
        }
        *value -= 1;
    }

    fn release(&self) {
        let mut value = self.data.0.lock().unwrap();
        *value += 1;
        self.data.1.notify_one();
    }
}

pub struct Renderer {
    pub device: Device,
    pub scene: Scene,
    pub uniform_buffer: Buffer,
    pub resource_buffer: Buffer,
    pub instance_acceleration_structure: AccelerationStructure,
    pub accumulation_targets: [Texture; 2],
    pub random_texture: Texture,
    pub frame_index: NSUInteger,
    pub uniform_buffer_index: NSUInteger,
    pub uniform_buffer_offset: NSUInteger,
    pub size: CGSize,
    semaphore: Semaphore,
    pub queue: CommandQueue,
    instance_buffer: Buffer,
    intersection_function_table: IntersectionFunctionTable,
    primitive_acceleration_structures: Vec<AccelerationStructure>,
    raytracing_pipeline: ComputePipelineState,
    copy_pipeline: RenderPipelineState,
}

impl Renderer {
    pub fn new(device: Device) -> Self {
        let scene = Scene::new(device.clone());

        let library_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/raytracing/shaders.metallib");
        let library = device.new_library_with_file(library_path).unwrap();
        let queue = device.new_command_queue();

        let buffer_data = [0u8; UNIFORM_BUFFER_SIZE as usize];
        let uniform_buffer = device.new_buffer_with_data(
            buffer_data.as_ptr().cast(),
            UNIFORM_BUFFER_SIZE,
            get_managed_buffer_storage_mode(),
        );
        uniform_buffer.set_label("uniform buffer");
        let resources_stride = {
            let mut max = 0;
            for geometry in &scene.geometries {
                let s = geometry.get_resources().len();
                if s > max {
                    max = s;
                }
            }
            max
        };
        let mut resource_buffer_data = vec![0u64; resources_stride * scene.geometries.len()];
        for geometry_index in 0..scene.geometries.len() {
            let geometry = scene.geometries[geometry_index].as_ref();
            let resource_buffer_begin_index = resources_stride * geometry_index;
            let resources = geometry.get_resources();

            for (argument_index, resource) in resources.iter().enumerate() {
                let resource_buffer_index = resource_buffer_begin_index + argument_index;
                let resource = resource.clone();
                resource_buffer_data[resource_buffer_index] =
                    if resource.conforms_to_protocol::<MTLBuffer>().unwrap() {
                        let buffer = unsafe { Buffer::from_ptr(resource.into_ptr().cast()) };
                        buffer.gpu_address()
                    } else if resource.conforms_to_protocol::<MTLTexture>().unwrap() {
                        let texture = unsafe { Texture::from_ptr(resource.into_ptr().cast()) };
                        texture.gpu_resource_id()._impl
                    } else {
                        panic!("Unexpected resource!")
                    }
            }
        }
        let resource_buffer = device.new_buffer_with_data(
            resource_buffer_data.as_ptr().cast(),
            size_of_val(resource_buffer_data.as_slice()) as NSUInteger,
            get_managed_buffer_storage_mode(),
        );
        resource_buffer.set_label("resource buffer");
        resource_buffer.did_modify_range(NSRange::new(0, resource_buffer.length()));

        let mut primitive_acceleration_structures = Vec::new();
        for i in 0..scene.geometries.len() {
            let mesh = scene.geometries[i].as_ref();
            let geometry_descriptor = mesh.get_geometry_descriptor();
            geometry_descriptor.set_intersection_function_table_offset(i as NSUInteger);
            let geometry_descriptors = Array::from_owned_slice(&[geometry_descriptor]);
            let accel_descriptor = PrimitiveAccelerationStructureDescriptor::descriptor();
            accel_descriptor.set_geometry_descriptors(geometry_descriptors);
            let accel_descriptor: AccelerationStructureDescriptor = From::from(accel_descriptor);
            primitive_acceleration_structures.push(
                Self::new_acceleration_structure_with_descriptor(
                    &device,
                    &queue,
                    &accel_descriptor,
                ),
            );
        }

        let mut instance_descriptors = vec![
            MTLAccelerationStructureInstanceDescriptor::default();
            scene.geometry_instances.len()
        ];
        for (instance_index, instance) in scene.geometry_instances.iter().enumerate() {
            let geometry_index = instance.index_in_scene;
            instance_descriptors[instance_index].acceleration_structure_index =
                geometry_index as u32;
            instance_descriptors[instance_index].options =
                if instance.geometry.get_intersection_function_name().is_none() {
                    MTLAccelerationStructureInstanceOptions::Opaque
                } else {
                    MTLAccelerationStructureInstanceOptions::None
                };
            instance_descriptors[instance_index].intersection_function_table_offset = 0;
            instance_descriptors[instance_index].mask = instance.mask;
            for column in 0..4 {
                for row in 0..3 {
                    instance_descriptors[instance_index].transformation_matrix[column][row] =
                        *instance.transform.col(column).index(row);
                }
            }
        }
        let instance_buffer = device.new_buffer_with_data(
            instance_descriptors.as_ptr().cast(),
            size_of_val(instance_descriptors.as_slice()) as NSUInteger,
            get_managed_buffer_storage_mode(),
        );
        instance_buffer.set_label("instance buffer");
        instance_buffer.did_modify_range(NSRange::new(0, instance_buffer.length()));

        let accel_descriptor = InstanceAccelerationStructureDescriptor::descriptor();
        accel_descriptor.set_instanced_acceleration_structures(Array::from_owned_slice(
            &primitive_acceleration_structures,
        ));
        accel_descriptor.set_instance_count(scene.geometry_instances.len() as NSUInteger);
        accel_descriptor.set_instance_descriptor_buffer(&instance_buffer);
        let accel_descriptor: AccelerationStructureDescriptor = From::from(accel_descriptor);
        let instance_acceleration_structure =
            Self::new_acceleration_structure_with_descriptor(&device, &queue, &accel_descriptor);

        let mut intersection_functions = BTreeMap::<String, Function>::new();
        for geometry in &scene.geometries {
            if let Some(name) = geometry.get_intersection_function_name() {
                if !intersection_functions.contains_key(name) {
                    let intersection_function = Self::new_specialised_function_with_name(
                        &library,
                        resources_stride as u32,
                        name,
                    );
                    intersection_functions.insert(name.to_string(), intersection_function);
                }
            }
        }
        let raytracing_function = Self::new_specialised_function_with_name(
            &library,
            resources_stride as u32,
            "raytracingKernel",
        );
        let intersection_function_array: Vec<&FunctionRef> = intersection_functions
            .values()
            .map(|f| -> &FunctionRef { f })
            .collect();
        let raytracing_pipeline = Self::new_compute_pipeline_state_with_function(
            &device,
            &raytracing_function,
            &intersection_function_array,
        );
        let intersection_function_table_descriptor = IntersectionFunctionTableDescriptor::new();
        intersection_function_table_descriptor
            .set_function_count(scene.geometries.len() as NSUInteger);
        let intersection_function_table = raytracing_pipeline
            .new_intersection_function_table_with_descriptor(
                &intersection_function_table_descriptor,
            );
        for geometry_index in 0..scene.geometries.len() {
            let geometry = scene.geometries[geometry_index].as_ref();
            if let Some(intersection_function_name) = geometry.get_intersection_function_name() {
                let intersection_function = &intersection_functions[intersection_function_name];
                let handle = raytracing_pipeline
                    .function_handle_with_function(intersection_function)
                    .unwrap();
                intersection_function_table.set_function(handle, geometry_index as NSUInteger);
            }
        }
        let render_descriptor = RenderPipelineDescriptor::new();
        render_descriptor
            .set_vertex_function(Some(&library.get_function("copyVertex", None).unwrap()));
        render_descriptor
            .set_fragment_function(Some(&library.get_function("copyFragment", None).unwrap()));
        render_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap()
            .set_pixel_format(MTLPixelFormat::RGBA16Float);
        let copy_pipeline = device
            .new_render_pipeline_state(&render_descriptor)
            .unwrap();

        let texture_descriptor = Self::create_target_descriptor(1024, 1024);
        let accumulation_targets = [
            device.new_texture(&texture_descriptor),
            device.new_texture(&texture_descriptor),
        ];
        let random_texture = device.new_texture(&texture_descriptor);

        Self {
            device,
            scene,
            uniform_buffer,
            resource_buffer,
            instance_acceleration_structure,
            accumulation_targets,
            random_texture,
            frame_index: 0,
            uniform_buffer_index: 0,
            uniform_buffer_offset: 0,
            size: CGSize::new(1024 as CGFloat, 1024 as CGFloat),
            semaphore: Semaphore::new((MAX_FRAMES_IN_FLIGHT - 2) as usize),
            instance_buffer,
            queue,
            intersection_function_table,
            primitive_acceleration_structures,
            raytracing_pipeline,
            copy_pipeline,
        }
    }

    fn create_target_descriptor(width: NSUInteger, height: NSUInteger) -> TextureDescriptor {
        let texture_descriptor = TextureDescriptor::new();
        texture_descriptor.set_pixel_format(MTLPixelFormat::RGBA32Float);
        texture_descriptor.set_texture_type(MTLTextureType::D2);
        texture_descriptor.set_width(width);
        texture_descriptor.set_height(height);
        texture_descriptor.set_storage_mode(MTLStorageMode::Private);
        texture_descriptor.set_usage(MTLTextureUsage::ShaderRead | MTLTextureUsage::ShaderWrite);
        texture_descriptor
    }

    pub fn window_resized(&mut self, size: CGSize) {
        self.size = size;
        let texture_descriptor =
            Self::create_target_descriptor(size.width as NSUInteger, size.height as NSUInteger);
        self.accumulation_targets[0] = self.device.new_texture(&texture_descriptor);
        self.accumulation_targets[1] = self.device.new_texture(&texture_descriptor);
        texture_descriptor.set_pixel_format(MTLPixelFormat::R32Uint);
        texture_descriptor.set_usage(MTLTextureUsage::ShaderRead);
        texture_descriptor.set_storage_mode(MTLStorageMode::Managed);
        self.random_texture = self.device.new_texture(&texture_descriptor);
        let mut rng = rand::rng();
        let mut random_values = vec![0u32; (size.width * size.height) as usize];
        for v in &mut random_values {
            *v = rng.next_u32();
        }
        self.random_texture.replace_region(
            MTLRegion::new_2d(0, 0, size.width as NSUInteger, size.height as NSUInteger),
            0,
            random_values.as_ptr().cast(),
            size_of::<u32>() as NSUInteger * size.width as NSUInteger,
        );
        self.frame_index = 0;
    }

    fn update_uniforms(&mut self) {
        self.uniform_buffer_offset = ALIGNED_UNIFORMS_SIZE * self.uniform_buffer_index;

        let uniforms = unsafe {
            &mut *self
                .uniform_buffer
                .contents()
                .add(self.uniform_buffer_offset as usize)
                .cast::<Uniforms>()
        };

        let position = self.scene.camera.position;
        let target = self.scene.camera.forward;
        let up = self.scene.camera.up;

        let forward = Vec3::normalize(target.xyz() - position.xyz());
        let right = Vec3::normalize(Vec3::cross(forward, up.xyz()));
        let up = Vec3::normalize(Vec3::cross(right, forward));

        uniforms.camera.position = position;
        uniforms.camera.forward = Vec4::from((forward, 0.0));
        uniforms.camera.right = Vec4::from((right, 0.0));
        uniforms.camera.up = Vec4::from((up, 0.0));

        let field_of_view = 45.0 * (std::f32::consts::PI / 180.0);
        let aspect_ratio = self.size.width as f32 / self.size.height as f32;
        let image_plane_height = f32::tan(field_of_view / 2.0);
        let image_plane_width = aspect_ratio * image_plane_height;

        uniforms.camera.right *= image_plane_width;
        uniforms.camera.up *= image_plane_height;

        uniforms.width = self.size.width as u32;
        uniforms.height = self.size.height as u32;

        uniforms.frame_index = self.frame_index as u32;
        self.frame_index += 1;

        uniforms.light_count = self.scene.lights.len() as u32;

        self.uniform_buffer.did_modify_range(NSRange {
            location: self.uniform_buffer_offset,
            length: ALIGNED_UNIFORMS_SIZE,
        });

        self.uniform_buffer_index = (self.uniform_buffer_index + 1) % MAX_FRAMES_IN_FLIGHT;
    }

    pub fn draw(&mut self, layer: &MetalLayer) {
        self.semaphore.acquire();
        self.update_uniforms();
        let command_buffer = self.queue.new_command_buffer();
        let sem = self.semaphore.clone();
        let block = block::ConcreteBlock::new(move |_| {
            sem.release();
        })
        .copy();
        command_buffer.add_completed_handler(&block);
        let width = self.size.width as NSUInteger;
        let height = self.size.height as NSUInteger;
        let threads_per_thread_group = MTLSize::new(8, 8, 1);
        let thread_groups = MTLSize::new(
            width.div_ceil(threads_per_thread_group.width),
            height.div_ceil(threads_per_thread_group.height),
            1,
        );
        let compute_encoder = command_buffer.new_compute_command_encoder();
        compute_encoder.set_buffer(0, Some(&self.uniform_buffer), self.uniform_buffer_offset);
        compute_encoder.set_buffer(2, Some(&self.instance_buffer), 0);
        compute_encoder.set_buffer(3, Some(&self.scene.lights_buffer), 0);
        compute_encoder.set_acceleration_structure(4, Some(&self.instance_acceleration_structure));
        compute_encoder.set_intersection_function_table(5, Some(&self.intersection_function_table));
        compute_encoder.set_texture(0, Some(&self.random_texture));
        compute_encoder.set_texture(1, Some(&self.accumulation_targets[0]));
        compute_encoder.set_texture(2, Some(&self.accumulation_targets[1]));
        for geometry in &self.scene.geometries {
            for resource in geometry.get_resources() {
                compute_encoder.use_resource(&resource, MTLResourceUsage::Read);
            }
        }
        for primitive_acceleration_structure in &self.primitive_acceleration_structures {
            let resource: Resource = From::from(primitive_acceleration_structure.clone());
            compute_encoder.use_resource(&resource, MTLResourceUsage::Read);
        }
        compute_encoder.set_compute_pipeline_state(&self.raytracing_pipeline);
        compute_encoder.dispatch_thread_groups(thread_groups, threads_per_thread_group);
        compute_encoder.end_encoding();
        (self.accumulation_targets[0], self.accumulation_targets[1]) = (
            self.accumulation_targets[1].clone(),
            self.accumulation_targets[0].clone(),
        );
        if let Some(drawable) = layer.next_drawable() {
            let render_pass_descriptor = RenderPassDescriptor::new();
            let colour_attachment = render_pass_descriptor
                .color_attachments()
                .object_at(0)
                .unwrap();
            colour_attachment.set_texture(Some(drawable.texture()));
            colour_attachment.set_load_action(MTLLoadAction::Clear);
            colour_attachment.set_clear_color(MTLClearColor::new(0.0, 0.0, 0.0, 1.0));
            let render_encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);
            render_encoder.set_render_pipeline_state(&self.copy_pipeline);
            render_encoder.set_fragment_texture(0, Some(&self.accumulation_targets[0]));
            render_encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 6);
            render_encoder.end_encoding();
            command_buffer.present_drawable(drawable);
        }
        command_buffer.commit();
    }

    fn new_acceleration_structure_with_descriptor(
        device: &Device,
        queue: &CommandQueue,
        descriptor: &AccelerationStructureDescriptorRef,
    ) -> AccelerationStructure {
        let accel_sizes = device.acceleration_structure_sizes_with_descriptor(descriptor);
        let acceleration_structure =
            device.new_acceleration_structure_with_size(accel_sizes.acceleration_structure_size);
        let scratch_buffer = device.new_buffer(
            accel_sizes.build_scratch_buffer_size,
            MTLResourceOptions::StorageModePrivate,
        );
        let command_buffer = queue.new_command_buffer();
        let command_encoder = command_buffer.new_acceleration_structure_command_encoder();
        let compacted_size_buffer = device.new_buffer(
            size_of::<u32>() as NSUInteger,
            MTLResourceOptions::StorageModeShared,
        );
        command_encoder.build_acceleration_structure(
            &acceleration_structure,
            descriptor,
            &scratch_buffer,
            0,
        );
        command_encoder.write_compacted_acceleration_structure_size(
            &acceleration_structure,
            &compacted_size_buffer,
            0,
        );
        command_encoder.end_encoding();
        command_buffer.commit();
        command_buffer.wait_until_completed();
        let compacted_size = unsafe { *compacted_size_buffer.contents().cast::<u32>() };
        let compacted_acceleration_structure =
            device.new_acceleration_structure_with_size(compacted_size as NSUInteger);
        let command_buffer = queue.new_command_buffer();
        let command_encoder = command_buffer.new_acceleration_structure_command_encoder();
        command_encoder.copy_and_compact_acceleration_structure(
            &acceleration_structure,
            &compacted_acceleration_structure,
        );
        command_encoder.end_encoding();
        command_buffer.commit();
        compacted_acceleration_structure
    }

    fn new_specialised_function_with_name(
        library: &Library,
        resources_stride: u32,
        name: &str,
    ) -> Function {
        let constants = FunctionConstantValues::new();
        let resources_stride = resources_stride * size_of::<u64>() as u32;
        constants.set_constant_value_at_index(
            (&raw const resources_stride).cast(),
            MTLDataType::UInt,
            0,
        );
        let v = true;
        constants.set_constant_value_at_index((&raw const v).cast(), MTLDataType::Bool, 1);
        constants.set_constant_value_at_index((&raw const v).cast(), MTLDataType::Bool, 2);
        library.get_function(name, Some(constants)).unwrap()
    }

    fn new_compute_pipeline_state_with_function(
        device: &Device,
        function: &Function,
        linked_functions: &[&FunctionRef],
    ) -> ComputePipelineState {
        let linked_functions = {
            let lf = LinkedFunctions::new();
            lf.set_functions(linked_functions);
            lf
        };
        let descriptor = ComputePipelineDescriptor::new();
        descriptor.set_compute_function(Some(function));
        descriptor.set_linked_functions(linked_functions.as_ref());
        descriptor.set_thread_group_size_is_multiple_of_thread_execution_width(true);
        device.new_compute_pipeline_state(&descriptor).unwrap()
    }
}
