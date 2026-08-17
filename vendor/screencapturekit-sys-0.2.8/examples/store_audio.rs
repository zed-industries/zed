use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Deref;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use objc_id::Id;

use screencapturekit_sys::os_types::base::BOOL;
use screencapturekit_sys::{
    cm_sample_buffer_ref::CMSampleBufferRef, content_filter::UnsafeContentFilter,
    content_filter::UnsafeInitParams, shareable_content::UnsafeSCShareableContent,
    stream::UnsafeSCStream, stream_configuration::UnsafeStreamConfiguration,
    stream_error_handler::UnsafeSCStreamError, stream_output_handler::UnsafeSCStreamOutput,
};

struct StoreAudioHandler {}

struct ErrorHandler;

impl UnsafeSCStreamError for ErrorHandler {
    fn handle_error(&self) {
        eprintln!("ERROR!");
    }
}

impl UnsafeSCStreamOutput for StoreAudioHandler {
    fn did_output_sample_buffer(&self, sample: Id<CMSampleBufferRef>, _of_type: u8) {
        println!("Got sample buffer");

        // Get audio format information
        let format_description = sample.get_format_description().expect("format description");
        println!("format_description={:?}", format_description);
        let audio = format_description
            .audio_format_description_get_stream_basic_description()
            .expect("Get AudioStreamBasicDescription");
        println!("audio info: {:?}", audio);
        println!("  format={:?}", audio.get_format_name().unwrap());
        println!("  flags={:?}", audio.get_flag_names());

        // Get audio buffers
        let audio_buffers = sample.get_av_audio_buffer_list();
        println!("audio buffer list: number={:?}", audio_buffers.len());
        for (i, buffer) in audio_buffers.into_iter().enumerate() {
            println!(
                "  {}: channels={}, size={}",
                i,
                buffer.number_channels,
                buffer.data.len()
            );

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true) // Use append mode
                .open(format!("{}.raw", i))
                .expect("failed to open file");

            if let Err(e) = file.write_all(buffer.data.deref()) {
                eprintln!("failed to write to file: {:?}", e);
            }
        }

        println!("status={:?}", sample.get_frame_info().status());
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

    let config = UnsafeStreamConfiguration {
        width,
        height,
        captures_audio: BOOL::from(true),
        excludes_current_process_audio: BOOL::from(true),
        ..Default::default()
    };

    for i in 0..8 {
        let filename = format!("{}.raw", i);
        if PathBuf::from(filename.clone()).exists() {
            fs::remove_file(PathBuf::from(filename.to_string())).unwrap();
        }
    }

    let stream = UnsafeSCStream::init(filter, config.into(), ErrorHandler);
    stream.add_stream_output(StoreAudioHandler {}, 1);
    stream.start_capture().ok();

    sleep(Duration::from_secs(5));

    stream.stop_capture().ok();

    // In order to play the audio, you need to convert the raw PCM file into a WAV format.
    //
    // Here's an example of how one can do this:
    // sox -t raw -r 48000 -e floating-point -b 32 -c 1 --endian little /tmp/audio-0.raw /tmp/output-0.wav
}
