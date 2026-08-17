//! An output stream example showing more advanced usage.
//! Tries to use hog mode to get exclusive access to the device.

extern crate coreaudio;

use coreaudio::audio_unit::audio_format::LinearPcmFlags;
use coreaudio::audio_unit::macos_helpers::{
    audio_unit_from_device_id, find_matching_physical_format, get_default_device_id,
    get_hogging_pid, get_supported_physical_stream_formats, set_device_physical_stream_format,
    set_device_sample_rate, toggle_hog_mode, AliveListener, RateListener,
};
use coreaudio::audio_unit::render_callback::{self, data};
use coreaudio::audio_unit::{Element, SampleFormat, Scope, StreamFormat};
use coreaudio::sys::kAudioUnitProperty_StreamFormat;
use std::f64::consts::PI;
use std::process;

const SAMPLE_FORMAT: SampleFormat = SampleFormat::F32;
// type S = i32; const SAMPLE_FORMAT: SampleFormat = SampleFormat::I32;
// type S = i16; const SAMPLE_FORMAT: SampleFormat = SampleFormat::I16;
// type S = i8; const SAMPLE_FORMAT: SampleFormat = SampleFormat::I8;

const SAMPLE_RATE: f64 = 44100.0;

const INTERLEAVED: bool = true;

struct SineWaveGenerator {
    time: f64,
    /// generated frequency in Hz
    freq: f64,
    /// magnitude of generated signal
    volume: f64,
}

impl SineWaveGenerator {
    fn new(freq: f64, volume: f64) -> Self {
        SineWaveGenerator {
            time: 0.,
            freq,
            volume,
        }
    }
}

impl Iterator for SineWaveGenerator {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        self.time += 1. / SAMPLE_RATE;
        let output = ((self.freq * self.time * PI * 2.).sin() * self.volume) as f32;
        Some(output)
    }
}

fn main() -> Result<(), coreaudio::Error> {
    let frequency_hz_l = 1000.;
    let frequency_hz_r = 1200.;
    let volume = 0.95;
    let mut samples_l = SineWaveGenerator::new(frequency_hz_l, volume);
    let mut samples_r = SineWaveGenerator::new(frequency_hz_r, volume);

    // Construct an Output audio unit that delivers audio to the default output device.
    let audio_unit_id = get_default_device_id(false).unwrap();
    let mut audio_unit = audio_unit_from_device_id(audio_unit_id, false)?;

    let pid = get_hogging_pid(audio_unit_id)?;
    if pid != -1 {
        println!("Device is owned by another process with pid {}!", pid);
    } else {
        println!("Device is free, trying to get exclusive access..");
        let new_pid = toggle_hog_mode(audio_unit_id)?;
        let process_id = process::id();
        if new_pid == process_id as i32 {
            println!("We have exclusive access.");
        } else {
            println!(
                "Could not get exclusive access. Process pid: {}, new pid value: {}",
                process_id, new_pid
            );
        }
    }

    let mut format_flag = match SAMPLE_FORMAT {
        SampleFormat::F32 => LinearPcmFlags::IS_FLOAT | LinearPcmFlags::IS_PACKED,
        SampleFormat::I32 | SampleFormat::I16 | SampleFormat::I8 => {
            LinearPcmFlags::IS_SIGNED_INTEGER | LinearPcmFlags::IS_PACKED
        }
        _ => {
            unimplemented!("Please use one of the packed formats");
        }
    };

    if !INTERLEAVED {
        format_flag = format_flag | LinearPcmFlags::IS_NON_INTERLEAVED;
    }

    let stream_format = StreamFormat {
        sample_rate: SAMPLE_RATE,
        sample_format: SAMPLE_FORMAT,
        flags: format_flag,
        // you can change this to 1
        channels: 2,
    };

    println!("stream format={:#?}", &stream_format);
    println!("asbd={:#?}", &stream_format.to_asbd());

    // Lets print all supported formats, disabled for now since it often crashes.
    println!("All supported formats");
    let formats = get_supported_physical_stream_formats(audio_unit_id)?;
    for fmt in formats {
        println!("{:?}", &fmt);
    }

    // set the sample rate. This isn't actually needed since the sample rate
    // will anyway be changed when setting the sample format later.
    // Keeping it here as an example.
    //println!("set device sample rate");
    //set_device_sample_rate(audio_unit_id, SAMPLE_RATE)?;

    println!("setting hardware (physical) format");
    let hw_stream_format = StreamFormat {
        sample_rate: SAMPLE_RATE,
        sample_format: SampleFormat::I16,
        flags: LinearPcmFlags::empty(),
        channels: 2,
    };

    let hw_asbd = find_matching_physical_format(audio_unit_id, hw_stream_format)
        .ok_or(coreaudio::Error::UnsupportedStreamFormat)?;

    println!("asbd: {:?}", hw_asbd);

    // Note that using a StreamFormat here is convenient, but it only supports a few sample formats.
    // Setting the format to for example 24 bit integers requires using an ASBD.
    set_device_physical_stream_format(audio_unit_id, hw_asbd)?;

    println!("write audio unit StreamFormat property");
    let id = kAudioUnitProperty_StreamFormat;
    let asbd = stream_format.to_asbd();
    audio_unit.set_property(id, Scope::Input, Element::Output, Some(&asbd))?;

    // For this example, our sine wave expects `f32` data.
    assert!(SampleFormat::F32 == stream_format.sample_format);

    // Register rate and alive listeners
    let mut rate_listener = RateListener::new(audio_unit_id, None);
    rate_listener.register()?;
    let mut alive_listener = AliveListener::new(audio_unit_id);
    alive_listener.register()?;

    if INTERLEAVED {
        println!("Register interleaved callback");
        type Args = render_callback::Args<data::Interleaved<f32>>;
        audio_unit.set_render_callback(move |args| {
            let Args {
                num_frames, data, ..
            } = args;
            // Print the number of frames the callback requests.
            // Included to aid understanding, don't use println and other things
            // that may block for an unknown amount of time inside the callback
            // of a real application.
            println!("frames: {}", num_frames);
            for i in 0..num_frames {
                let sample_l = samples_l.next().unwrap();
                let sample_r = samples_r.next().unwrap();
                data.buffer[2 * i] = sample_l;
                data.buffer[2 * i + 1] = sample_r;
            }
            Ok(())
        })?;
    } else {
        println!("Register non-interleaved callback");
        type Args = render_callback::Args<data::NonInterleaved<f32>>;
        audio_unit.set_render_callback(move |args| {
            let Args {
                num_frames,
                mut data,
                ..
            } = args;
            for i in 0..num_frames {
                let sample_l = samples_l.next().unwrap();
                let sample_r = samples_r.next().unwrap();
                let mut channels = data.channels_mut();
                let left = channels.next().unwrap();
                left[i] = sample_l;
                let right = channels.next().unwrap();
                right[i] = sample_r;
            }
            Ok(())
        })?;
    }
    audio_unit.start()?;

    for _ in 0..100 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        // print all sample change events
        println!("rate events: {:?}", rate_listener.copy_values());
        println!("alive state: {}", alive_listener.is_alive());
    }

    // Release exclusive access, not really needed as the process exits anyway after this.
    let owner_pid = get_hogging_pid(audio_unit_id)?;
    let process_id = process::id();
    if owner_pid == process_id as i32 {
        println!("Releasing exclusive access");
        let new_pid = toggle_hog_mode(audio_unit_id)?;
        if new_pid == -1 {
            println!("Exclusive access released.");
        } else {
            println!(
                "Could not release exclusive access. Process pid: {}, new pid value: {}",
                process_id, new_pid
            );
        }
    }
    Ok(())
}
