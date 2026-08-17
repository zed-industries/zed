use std::path::PathBuf;

use metal::*;
use objc::rc::autoreleasepool;

const NUM_SAMPLES: u64 = 2;

fn main() {
    let num_elements = std::env::args()
        .nth(1)
        .map(|s| s.parse::<u32>().unwrap())
        .unwrap_or(64 * 64);

    autoreleasepool(|| {
        let device = Device::system_default().expect("No device found");
        let mut cpu_start = 0;
        let mut gpu_start = 0;
        device.sample_timestamps(&mut cpu_start, &mut gpu_start);

        let counter_sample_buffer = create_counter_sample_buffer(&device);
        let destination_buffer = device.new_buffer(
            (size_of::<u64>() * NUM_SAMPLES as usize) as u64,
            MTLResourceOptions::StorageModeShared,
        );

        let counter_sampling_point = MTLCounterSamplingPoint::AtStageBoundary;
        assert!(device.supports_counter_sampling(counter_sampling_point));

        let command_queue = device.new_command_queue();
        let command_buffer = command_queue.new_command_buffer();

        let compute_pass_descriptor = ComputePassDescriptor::new();
        handle_compute_pass_sample_buffer_attachment(
            compute_pass_descriptor,
            &counter_sample_buffer,
        );
        let encoder =
            command_buffer.compute_command_encoder_with_descriptor(compute_pass_descriptor);

        let pipeline_state = create_pipeline_state(&device);
        encoder.set_compute_pipeline_state(&pipeline_state);

        let (buffer, sum) = create_input_and_output_buffers(&device, num_elements);
        encoder.set_buffer(0, Some(&buffer), 0);
        encoder.set_buffer(1, Some(&sum), 0);

        let num_threads = pipeline_state.thread_execution_width();

        let thread_group_count = MTLSize {
            width: ((num_elements as NSUInteger + num_threads) / num_threads),
            height: 1,
            depth: 1,
        };

        let thread_group_size = MTLSize {
            width: num_threads,
            height: 1,
            depth: 1,
        };

        encoder.dispatch_thread_groups(thread_group_count, thread_group_size);
        encoder.end_encoding();

        resolve_samples_into_buffer(command_buffer, &counter_sample_buffer, &destination_buffer);

        command_buffer.commit();
        command_buffer.wait_until_completed();
        let mut cpu_end = 0;
        let mut gpu_end = 0;
        device.sample_timestamps(&mut cpu_end, &mut gpu_end);

        let sum = unsafe { *sum.contents().cast::<u32>() };
        println!("Compute shader sum: {}", sum);

        assert_eq!(num_elements, sum);

        handle_timestamps(&destination_buffer, cpu_start, cpu_end, gpu_start, gpu_end);
    });
}

fn create_pipeline_state(device: &Device) -> ComputePipelineState {
    let library_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/compute/shaders.metallib");
    let library = device.new_library_with_file(library_path).unwrap();
    let kernel = library.get_function("sum", None).unwrap();

    let pipeline_state_descriptor = ComputePipelineDescriptor::new();
    pipeline_state_descriptor.set_compute_function(Some(&kernel));

    device
        .new_compute_pipeline_state_with_function(
            pipeline_state_descriptor.compute_function().unwrap(),
        )
        .unwrap()
}

fn handle_compute_pass_sample_buffer_attachment(
    compute_pass_descriptor: &ComputePassDescriptorRef,
    counter_sample_buffer: &CounterSampleBufferRef,
) {
    let sample_buffer_attachment_descriptor = compute_pass_descriptor
        .sample_buffer_attachments()
        .object_at(0)
        .unwrap();

    sample_buffer_attachment_descriptor.set_sample_buffer(counter_sample_buffer);
    sample_buffer_attachment_descriptor.set_start_of_encoder_sample_index(0);
    sample_buffer_attachment_descriptor.set_end_of_encoder_sample_index(1);
}

fn resolve_samples_into_buffer(
    command_buffer: &CommandBufferRef,
    counter_sample_buffer: &CounterSampleBufferRef,
    destination_buffer: &BufferRef,
) {
    let blit_encoder = command_buffer.new_blit_command_encoder();
    blit_encoder.resolve_counters(
        counter_sample_buffer,
        crate::NSRange::new(0u64, NUM_SAMPLES),
        destination_buffer,
        0u64,
    );
    blit_encoder.end_encoding();
}

fn handle_timestamps(
    resolved_sample_buffer: &BufferRef,
    cpu_start: u64,
    cpu_end: u64,
    gpu_start: u64,
    gpu_end: u64,
) {
    let samples = unsafe {
        std::slice::from_raw_parts(
            resolved_sample_buffer.contents().cast::<u64>(),
            NUM_SAMPLES as usize,
        )
    };
    let pass_start = samples[0];
    let pass_end = samples[1];

    let cpu_time_span = cpu_end - cpu_start;
    let gpu_time_span = gpu_end - gpu_start;

    let micros = microseconds_between_begin(pass_start, pass_end, gpu_time_span, cpu_time_span);
    println!("Compute pass duration: {} µs", micros);
}

fn create_counter_sample_buffer(device: &Device) -> CounterSampleBuffer {
    let counter_sample_buffer_desc = metal::CounterSampleBufferDescriptor::new();
    counter_sample_buffer_desc.set_storage_mode(metal::MTLStorageMode::Shared);
    counter_sample_buffer_desc.set_sample_count(NUM_SAMPLES);
    let counter_sets = device.counter_sets();

    let timestamp_counter = counter_sets.iter().find(|cs| cs.name() == "timestamp");

    counter_sample_buffer_desc
        .set_counter_set(timestamp_counter.expect("No timestamp counter found"));

    device
        .new_counter_sample_buffer_with_descriptor(&counter_sample_buffer_desc)
        .unwrap()
}

fn create_input_and_output_buffers(
    device: &Device,
    num_elements: u32,
) -> (metal::Buffer, metal::Buffer) {
    let data = vec![1u32; num_elements as usize];

    let buffer = device.new_buffer_with_data(
        data.as_ptr().cast(),
        size_of_val(data.as_slice()) as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache,
    );

    let sum = {
        let data = [0u32];
        device.new_buffer_with_data(
            data.as_ptr().cast(),
            size_of_val(&data) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        )
    };
    (buffer, sum)
}

/// <https://developer.apple.com/documentation/metal/gpu_counters_and_counter_sample_buffers/converting_gpu_timestamps_into_cpu_time>
fn microseconds_between_begin(begin: u64, end: u64, gpu_time_span: u64, cpu_time_span: u64) -> f64 {
    let time_span = (end as f64) - (begin as f64);
    let nanoseconds = time_span / (gpu_time_span as f64) * (cpu_time_span as f64);
    nanoseconds / 1000.0
}
