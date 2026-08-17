//! A basic input + output stream example, copying the mic input stream to the default output stream

extern crate coreaudio;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use coreaudio::audio_unit::audio_format::LinearPcmFlags;
use coreaudio::audio_unit::macos_helpers::{
    audio_unit_from_device_id, get_default_device_id, get_device_name, RateListener,
};
use coreaudio::audio_unit::render_callback::{self, data};
use coreaudio::audio_unit::{Element, SampleFormat, Scope, StreamFormat};
use coreaudio::sys::*;

const SAMPLE_RATE: f64 = 44100.0;

type S = f32;
const SAMPLE_FORMAT: SampleFormat = SampleFormat::F32;
// type S = i32; const SAMPLE_FORMAT: SampleFormat = SampleFormat::I32;
// type S = i16; const SAMPLE_FORMAT: SampleFormat = SampleFormat::I16;
// type S = i8; const SAMPLE_FORMAT: SampleFormat = SampleFormat::I8;

fn main() -> Result<(), coreaudio::Error> {
    let input_device_id = get_default_device_id(true).unwrap();
    let output_device_id = get_default_device_id(false).unwrap();
    println!(
        "Input device: {}",
        get_device_name(input_device_id).unwrap()
    );
    println!(
        "Output device: {}",
        get_device_name(output_device_id).unwrap()
    );
    let mut input_audio_unit = audio_unit_from_device_id(input_device_id, true)?;
    let mut output_audio_unit = audio_unit_from_device_id(output_device_id, false)?;

    let format_flag = match SAMPLE_FORMAT {
        SampleFormat::F32 => LinearPcmFlags::IS_FLOAT | LinearPcmFlags::IS_PACKED,
        SampleFormat::I32 | SampleFormat::I16 | SampleFormat::I8 => {
            LinearPcmFlags::IS_SIGNED_INTEGER | LinearPcmFlags::IS_PACKED
        }
        _ => {
            unimplemented!("Please use one of the packed formats");
        }
    };

    let in_stream_format = StreamFormat {
        sample_rate: SAMPLE_RATE,
        sample_format: SAMPLE_FORMAT,
        flags: format_flag,
        channels: 2,
    };

    let out_stream_format = StreamFormat {
        sample_rate: SAMPLE_RATE,
        sample_format: SAMPLE_FORMAT,
        flags: format_flag,
        channels: 2,
    };

    println!("input={:#?}", &in_stream_format);
    println!("output={:#?}", &out_stream_format);
    println!("input_asbd={:#?}", &in_stream_format.to_asbd());
    println!("output_asbd={:#?}", &out_stream_format.to_asbd());

    let id = kAudioUnitProperty_StreamFormat;
    let asbd = in_stream_format.to_asbd();
    input_audio_unit.set_property(id, Scope::Output, Element::Input, Some(&asbd))?;

    let asbd = out_stream_format.to_asbd();
    output_audio_unit.set_property(id, Scope::Input, Element::Output, Some(&asbd))?;

    let buffer_left = Arc::new(Mutex::new(VecDeque::<S>::new()));
    let producer_left = buffer_left.clone();
    let consumer_left = buffer_left.clone();
    let buffer_right = Arc::new(Mutex::new(VecDeque::<S>::new()));
    let producer_right = buffer_right.clone();
    let consumer_right = buffer_right.clone();

    // Register a rate listener for playback
    let mut listener_pb = RateListener::new(output_device_id, None);
    listener_pb.register()?;

    // Register a rate listener for capture
    let mut listener_cap = RateListener::new(input_device_id, None);
    listener_cap.register()?;

    // seed roughly 1 second of data to create a delay in the feedback loop for easier testing
    for buffer in vec![buffer_left, buffer_right] {
        let mut buffer = buffer.lock().unwrap();
        for _ in 0..(out_stream_format.sample_rate as i32) {
            buffer.push_back(0 as S);
        }
    }

    type Args = render_callback::Args<data::Interleaved<S>>;

    input_audio_unit.set_input_callback(move |args| {
        let Args {
            num_frames, data, ..
        } = args;
        // Print the number of frames the callback requests.
        // Included to aid understanding, don't use println and other things
        // that may block for an unknown amount of time inside the callback
        // of a real application.
        println!("input cb {} frames", num_frames);
        let buffer_left = producer_left.lock().unwrap();
        let buffer_right = producer_right.lock().unwrap();
        let mut buffers = vec![buffer_left, buffer_right];
        for i in 0..num_frames {
            for channel in 0..2 {
                let value: S = data.buffer[2 * i + channel];
                buffers[channel].push_back(value);
            }
        }
        Ok(())
    })?;
    input_audio_unit.start()?;

    output_audio_unit.set_render_callback(move |args: Args| {
        let Args {
            num_frames, data, ..
        } = args;
        // Print the number of frames the callback requests.
        println!("output cb {} frames", num_frames);
        let buffer_left = consumer_left.lock().unwrap();
        let buffer_right = consumer_right.lock().unwrap();
        let mut buffers = vec![buffer_left, buffer_right];
        for i in 0..num_frames {
            // Default other channels to copy value from first channel as a fallback
            let zero: S = 0 as S;
            let f: S = *buffers[0].front().unwrap_or(&zero);
            for channel in 0..2 {
                let sample: S = buffers[channel].pop_front().unwrap_or(f);
                data.buffer[2 * i + channel] = sample;
            }
        }
        Ok(())
    })?;
    output_audio_unit.start()?;
    for _ in 0..1000 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        if listener_cap.get_nbr_values() > 0 {
            println!("capture rate change: {:?}", listener_cap.drain_values());
        }
        if listener_pb.get_nbr_values() > 0 {
            println!("playback rate change: {:?}", listener_pb.drain_values());
        }
    }
    Ok(())
}
