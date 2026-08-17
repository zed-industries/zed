use metal::*;

#[repr(C)]
struct Vertex {
    xyz: [f32; 3],
}

type Ray = mps::MPSRayOriginMinDistanceDirectionMaxDistance;
type Intersection = mps::MPSIntersectionDistancePrimitiveIndexCoordinates;

// Original example taken from https://sergeyreznik.github.io/metal-ray-tracer/part-1/index.html
fn main() {
    let device = Device::system_default().expect("No device found");

    let library_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/mps/shaders.metallib");
    let library = device
        .new_library_with_file(library_path)
        .expect("Failed to load shader library");

    let generate_rays_pipeline = create_pipeline("generateRays", &library, &device);

    let queue = device.new_command_queue();
    let command_buffer = queue.new_command_buffer();

    // Simple vertex/index buffer data

    let vertices = [
        Vertex {
            xyz: [0.25, 0.25, 0.0],
        },
        Vertex {
            xyz: [0.75, 0.25, 0.0],
        },
        Vertex {
            xyz: [0.50, 0.75, 0.0],
        },
    ];

    let indices = [0, 1, 2];

    // Vertex data should be stored in private or managed buffers on discrete GPU systems (AMD, NVIDIA).
    // Private buffers are stored entirely in GPU memory and cannot be accessed by the CPU. Managed
    // buffers maintain a copy in CPU memory and a copy in GPU memory.
    let buffer_opts = MTLResourceOptions::StorageModeManaged;

    let vertex_buffer = device.new_buffer_with_data(
        vertices.as_ptr().cast(),
        size_of_val(&vertices) as u64,
        buffer_opts,
    );

    let index_buffer = device.new_buffer_with_data(
        indices.as_ptr().cast(),
        size_of_val(&indices) as u64,
        buffer_opts,
    );

    // Build an acceleration structure using our vertex and index buffers containing the single triangle.
    let acceleration_structure = mps::TriangleAccelerationStructure::from_device(&device)
        .expect("Failed to create acceleration structure");

    acceleration_structure.set_vertex_buffer(Some(&vertex_buffer));
    acceleration_structure.set_vertex_stride(size_of::<Vertex>() as u64);
    acceleration_structure.set_index_buffer(Some(&index_buffer));
    acceleration_structure.set_index_type(mps::MPSDataType::UInt32);
    acceleration_structure.set_triangle_count(1);
    acceleration_structure.set_usage(mps::MPSAccelerationStructureUsage::None);
    acceleration_structure.rebuild();

    let ray_intersector =
        mps::RayIntersector::from_device(&device).expect("Failed to create ray intersector");

    ray_intersector.set_ray_stride(size_of::<Ray>() as u64);
    ray_intersector.set_ray_data_type(mps::MPSRayDataType::OriginMinDistanceDirectionMaxDistance);
    ray_intersector.set_intersection_stride(size_of::<Intersection>() as u64);
    ray_intersector.set_intersection_data_type(
        mps::MPSIntersectionDataType::DistancePrimitiveIndexCoordinates,
    );

    // Create a buffer to hold generated rays and intersection results
    let ray_count = 1024;
    let ray_buffer = device.new_buffer(
        (size_of::<Ray>() * ray_count) as u64,
        MTLResourceOptions::StorageModePrivate,
    );

    let intersection_buffer = device.new_buffer(
        (size_of::<Intersection>() * ray_count) as u64,
        MTLResourceOptions::StorageModePrivate,
    );

    // Run the compute shader to generate rays
    let encoder = command_buffer.new_compute_command_encoder();
    encoder.set_buffer(0, Some(&ray_buffer), 0);
    encoder.set_compute_pipeline_state(&generate_rays_pipeline);
    encoder.dispatch_thread_groups(
        MTLSize {
            width: 4,
            height: 4,
            depth: 1,
        },
        MTLSize {
            width: 8,
            height: 8,
            depth: 1,
        },
    );
    encoder.end_encoding();

    // Intersect rays with triangles inside acceleration structure
    ray_intersector.encode_intersection_to_command_buffer(
        &command_buffer,
        mps::MPSIntersectionType::Nearest,
        &ray_buffer,
        0,
        &intersection_buffer,
        0,
        ray_count as u64,
        &acceleration_structure,
    );

    command_buffer.commit();
    command_buffer.wait_until_completed();

    println!("Done");
}

fn create_pipeline(func: &str, library: &LibraryRef, device: &DeviceRef) -> ComputePipelineState {
    // Create compute pipelines will will execute code on the GPU
    let compute_descriptor = ComputePipelineDescriptor::new();

    // Set to YES to allow compiler to make certain optimizations
    compute_descriptor.set_thread_group_size_is_multiple_of_thread_execution_width(true);

    let function = library.get_function(func, None).unwrap();
    compute_descriptor.set_compute_function(Some(&function));

    let pipeline = device
        .new_compute_pipeline_state(&compute_descriptor)
        .unwrap();

    pipeline
}
