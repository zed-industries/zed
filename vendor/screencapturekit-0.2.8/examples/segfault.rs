use std::{path::PathBuf, thread, time};

use screencapturekit::{
    cm_sample_buffer::CMSampleBuffer,
    sc_content_filter::{InitParams, SCContentFilter},
    sc_error_handler::StreamErrorHandler,
    sc_output_handler::{SCStreamOutputType, StreamOutput},
    sc_shareable_content::SCShareableContent,
    sc_stream::SCStream,
    sc_stream_configuration::SCStreamConfiguration,
};
use screencapturekit_sys::os_types::geometry::{CGPoint, CGRect, CGSize};

struct ErrorHandler;
impl StreamErrorHandler for ErrorHandler {
    fn on_error(&self) {
        println!("Error!");
    }
}

pub struct Capturer {}

impl Capturer {
    pub fn new() -> Self {
        println!("Capturer initialized");
        Capturer {}
    }
}

impl StreamErrorHandler for Capturer {
    fn on_error(&self) {
        eprintln!("ERROR!");
    }
}

impl StreamOutput for Capturer {
    fn did_output_sample_buffer(&self, sample: CMSampleBuffer, of_type: SCStreamOutputType) {
        println!("New frame recvd");
    }
}
fn main() {
    println!("Starting");

    let content = SCShareableContent::current();
    let displays = content.displays;

    let display = displays.first().unwrap_or_else(|| {
        panic!("Main display not found");
    });
    let display = display.to_owned();

    let width = display.width;
    let height = display.height;

    let params = InitParams::Display(display);
    let filter = SCContentFilter::new(params);

    let stream_config = SCStreamConfiguration {
        width,
        height,
        ..Default::default()
    };

    let mut stream = SCStream::new(filter, stream_config, ErrorHandler);
    let capturer = Capturer::new();
    stream.add_output(capturer, SCStreamOutputType::Screen);

    stream.start_capture();

    let ten_millis = time::Duration::from_millis(10000);

    thread::sleep(ten_millis);

    stream.stop_capture();

    println!("Ended");
}
