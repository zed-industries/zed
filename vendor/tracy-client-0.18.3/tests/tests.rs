use std::time::Duration;

use tracy_client::*;

#[global_allocator]
static GLOBAL: ProfiledAllocator<std::alloc::System> =
    ProfiledAllocator::new(std::alloc::System, 100);

fn basic_zone() {
    let client = Client::start();
    let span = client.span(span_location!("basic_zone"), 100);
    span.emit_value(42);
    span.emit_text("some text");
    for i in 322..420 {
        span.emit_value(i);
    }
}

fn alloc_zone() {
    let client = Client::start();
    let span = client.span_alloc(Some("alloc_zone"), "alloc_zone", file!(), line!(), 100);
    span.emit_value(42);
    span.emit_color(0x00FF0000);
    span.emit_text("some text");
}

fn finish_frameset() {
    let client = Client::start();
    for _ in 0..10 {
        client.frame_mark();
    }
    frame_mark();
}

fn finish_secondary_frameset() {
    let client = Client::start();
    for _ in 0..5 {
        client.secondary_frame_mark(frame_name!("secondary frame"));
    }
    secondary_frame_mark!("secondary frame macro");
}

fn non_continuous_frameset() {
    const NON_CONTINUOUS: FrameName = frame_name!("non continuous");
    let client = Client::start();
    let _ = client.non_continuous_frame(NON_CONTINUOUS);
    let _ = non_continuous_frame!("non continuous macro");
}

fn plot_something() {
    static TEMPERATURE: PlotName = plot_name!("temperature");
    let client = Client::start();
    for i in 0..10 {
        client.plot(TEMPERATURE, f64::from(i));
    }

    plot!("temperature", 42.0);
}

fn allocations() {
    let mut strings = Vec::new();
    for i in 0..100 {
        strings.push(format!("{i:?}"));
    }
}

fn fib(i: u16) -> u64 {
    let span = span!();
    span.emit_text(&format!("fib({i})"));
    let result = match i {
        0 => 0,
        1 => 1,
        _ => fib(i - 1) + fib(i - 2),
    };
    span.emit_value(result);
    result
}

fn message() {
    let client = Client::start();
    client.message("test message", 100);
    client.message("test message without stack", 0);
}

fn tls_confusion() {
    let client = Client::start();
    let t1 = std::thread::spawn(move || {
        drop(client);
    });
    let _ = t1.join();
    let _ = Client::start();
}

fn set_thread_name() {
    let _client = Client::start();
    set_thread_name!("test thread");
}

fn nameless_span() {
    let client = Client::start();
    let _ = span!();
    let _ = client.span_alloc(None, "nameless_span", file!(), line!(), 0);
    set_thread_name!("test thread");
}

fn gpu() {
    let client = Client::start();

    let gpu_context = client
        .new_gpu_context(Some("MyContext"), GpuContextType::Vulkan, 1_000, 1.0)
        .unwrap();

    // cmd_buf.write_timestamp(...); to start a span
    let span_loc = span_location!("MyGpuSpan1");
    let mut span1 = gpu_context.span(span_loc).unwrap();

    // cmd_buf.write_timestamp(...); to end a span
    span1.end_zone();

    // cmd_buf.write_timestamp(...); to start a second span
    let mut span2 = gpu_context
        .span_alloc("MyGpuSpan2", "Blah::Blah2", "myfile.rs", 14)
        .unwrap();

    // cmd_buf.write_timestamp(...); to end a second span
    span2.end_zone();

    // Some time later, when the timestamps are back
    span1.upload_timestamp_start(100_000);
    span1.upload_timestamp_end(110_000);
    span2.upload_timestamp_start(120_000);
    span2.upload_timestamp_end(130_000);
}

fn main() {
    #[cfg(not(loom))]
    {
        basic_zone();
        alloc_zone();
        finish_frameset();
        finish_secondary_frameset();
        non_continuous_frameset();
        plot_something();
        message();
        allocations();
        tls_confusion();
        nameless_span();
        let thread = std::thread::spawn(|| {
            let _client = Client::start();
            fib(25);
        });
        thread.join().unwrap();
        set_thread_name();
        gpu();
        // Sleep to give time to the client to send the data to the profiler.
        std::thread::sleep(Duration::from_secs(5));
    }
}
