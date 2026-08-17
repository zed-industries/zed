use std::{
    fs::File,
    io::Write,
    process::Command,
    sync::mpsc::{sync_channel, SyncSender},
};

use objc_foundation::INSData;
use objc_id::Id;
use screencapturekit_sys::{
    cm_sample_buffer_ref::CMSampleBufferRef, content_filter::UnsafeContentFilter,
    content_filter::UnsafeInitParams, sc_stream_frame_info::SCFrameStatus,
    shareable_content::UnsafeSCShareableContent, stream::UnsafeSCStream,
    stream_configuration::UnsafeStreamConfiguration, stream_error_handler::UnsafeSCStreamError,
    stream_output_handler::UnsafeSCStreamOutput,
};

struct StoreImageHandler {
    tx: SyncSender<Id<CMSampleBufferRef>>,
}

struct ErrorHandler;

impl UnsafeSCStreamError for ErrorHandler {
    fn handle_error(&self) {
        eprintln!("ERROR!");
    }
}

impl UnsafeSCStreamOutput for StoreImageHandler {
    fn did_output_sample_buffer(&self, sample: Id<CMSampleBufferRef>, _of_type: u8) {
        sample.get_frame_info();
        if let SCFrameStatus::Complete = sample.get_frame_info().status() {
            self.tx.send(sample).ok();
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
    let filter = UnsafeContentFilter::init(UnsafeInitParams::Display(display));
    let (tx, rx) = sync_channel(2);

    let config = UnsafeStreamConfiguration {
        width,
        height,
        ..Default::default()
    };

    let stream = UnsafeSCStream::init(filter, config.into(), ErrorHandler);
    stream.add_stream_output(StoreImageHandler { tx }, 0);
    stream.start_capture().ok();

    let sample_buf = rx.recv().unwrap();
    stream.stop_capture().ok();
    let jpeg = sample_buf.get_image_buffer().unwrap().get_jpeg_data();

    let mut buffer = File::create("picture.jpg").unwrap();

    buffer.write_all(jpeg.bytes()).unwrap();
    Command::new("open")
        .args(["picture.jpg"])
        .output()
        .expect("failedto execute process");
}
