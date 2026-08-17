use objc_id::Id;
use once_cell::sync::Lazy;
use screencapturekit_sys::{
    cm_sample_buffer_ref::CMSampleBufferRef,
    content_filter::{UnsafeContentFilter, UnsafeInitParams::Display},
    sc_stream_frame_info::SCFrameStatus,
    shareable_content::UnsafeSCShareableContent,
    stream::UnsafeSCStream,
    stream_configuration::UnsafeStreamConfiguration,
    stream_error_handler::UnsafeSCStreamError,
    stream_output_handler::UnsafeSCStreamOutput,
};
use std::{
    sync::atomic::{AtomicI64, Ordering},
    thread,
    time::Duration,
};

#[repr(C)]
struct TestHandler {}
impl UnsafeSCStreamError for TestHandler {
    fn handle_error(&self) {
        eprintln!("ERROR!");
    }
}
static PREV_TIMESTAMP: Lazy<AtomicI64> = Lazy::new(|| AtomicI64::new(0));

impl UnsafeSCStreamOutput for TestHandler {
    fn did_output_sample_buffer(&self, sample: Id<CMSampleBufferRef>, _of_type: u8) {
        if let SCFrameStatus::Complete = sample.get_frame_info().status() {
            let timescale_ms = 1000000;
            let prev_timestamp = PREV_TIMESTAMP.load(Ordering::Relaxed);
            let new_timestamp = sample.get_presentation_timestamp().value / timescale_ms;
            let frame_ms = new_timestamp - prev_timestamp;
            println!("{} MS for frame", frame_ms);
            PREV_TIMESTAMP.store(new_timestamp, Ordering::Relaxed);
        }
    }
}
fn main() {
    let display = UnsafeSCShareableContent::get()
        .unwrap()
        .displays()
        .into_iter()
        .next()
        .unwrap();
    let width = display.get_width();
    let height = display.get_height();
    println!("{width}, {height}");
    let width = display.get_width();
    let height = display.get_height();
    let filter = UnsafeContentFilter::init(Display(display));

    let config = UnsafeStreamConfiguration {
        width,
        height,
        ..Default::default()
    };

    let stream = UnsafeSCStream::init(filter, config.into(), TestHandler {});
    stream.add_stream_output(TestHandler {}, 0);
    stream.start_capture().ok();

    thread::sleep(Duration::from_millis(10_000));
}
