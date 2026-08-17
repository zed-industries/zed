use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    DeviceDescription, DeviceDescriptionBuilder,
};
use cpal::{FromSample, Sample};

#[allow(dead_code)]
#[derive(Clone)] // Clone, Send+Sync are required
struct MyHost;

#[derive(Clone)] // Clone, Send+Sync are required
struct MyDevice;

// Only Send+Sync is needed
struct MyStream {
    controls: Arc<StreamControls>,
    // option is needed since joining a thread takes ownership,
    // and we want to do that on drop (gives us &mut self, not self)
    handle: Option<std::thread::JoinHandle<()>>,
}

struct StreamControls {
    exit: AtomicBool,
    pause: AtomicBool,
}

impl HostTrait for MyHost {
    type Device = MyDevice;
    type Devices = std::iter::Once<MyDevice>;

    fn is_available() -> bool {
        true
    }

    fn devices(&self) -> Result<Self::Devices, cpal::DevicesError> {
        Ok(std::iter::once(MyDevice))
    }

    fn default_input_device(&self) -> Option<Self::Device> {
        None
    }

    fn default_output_device(&self) -> Option<Self::Device> {
        Some(MyDevice)
    }
}

impl DeviceTrait for MyDevice {
    type SupportedInputConfigs = std::iter::Empty<cpal::SupportedStreamConfigRange>;
    type SupportedOutputConfigs = std::iter::Once<cpal::SupportedStreamConfigRange>;
    type Stream = MyStream;

    fn name(&self) -> Result<String, cpal::DeviceNameError> {
        Ok(String::from("custom"))
    }

    fn description(&self) -> Result<DeviceDescription, cpal::DeviceNameError> {
        Ok(DeviceDescriptionBuilder::new("Custom Device".to_string()).build())
    }

    fn id(&self) -> Result<cpal::DeviceId, cpal::DeviceIdError> {
        Err(cpal::DeviceIdError::UnsupportedPlatform)
    }

    fn supported_input_configs(
        &self,
    ) -> Result<Self::SupportedInputConfigs, cpal::SupportedStreamConfigsError> {
        Ok(std::iter::empty())
    }

    fn supported_output_configs(
        &self,
    ) -> Result<Self::SupportedOutputConfigs, cpal::SupportedStreamConfigsError> {
        Ok(std::iter::once(cpal::SupportedStreamConfigRange::new(
            2,
            44100,
            44100,
            cpal::SupportedBufferSize::Unknown,
            cpal::SampleFormat::F32,
        )))
    }

    fn default_input_config(
        &self,
    ) -> Result<cpal::SupportedStreamConfig, cpal::DefaultStreamConfigError> {
        Err(cpal::DefaultStreamConfigError::StreamTypeNotSupported)
    }

    fn default_output_config(
        &self,
    ) -> Result<cpal::SupportedStreamConfig, cpal::DefaultStreamConfigError> {
        Ok(cpal::SupportedStreamConfig::new(
            2,
            44100,
            cpal::SupportedBufferSize::Unknown,
            cpal::SampleFormat::I16,
        ))
    }

    fn build_input_stream_raw<D, E>(
        &self,
        _: &cpal::StreamConfig,
        _: cpal::SampleFormat,
        _: D,
        _: E,
        _: Option<std::time::Duration>,
    ) -> Result<Self::Stream, cpal::BuildStreamError>
    where
        D: FnMut(&cpal::Data, &cpal::InputCallbackInfo) + Send + 'static,
        E: FnMut(cpal::StreamError) + Send + 'static,
    {
        Err(cpal::BuildStreamError::StreamConfigNotSupported)
    }

    // this is the meat of a custom device impl.
    // you're expected to repeatedly call `data_callback` and provide it with a buffer of samples,
    // as well as a stream timestamp.
    // a proper impl would also check the stream config and sample format, as well as handle errors
    fn build_output_stream_raw<D, E>(
        &self,
        _: &cpal::StreamConfig,
        _: cpal::SampleFormat,
        mut data_callback: D,
        _: E,
        _: Option<std::time::Duration>,
    ) -> Result<Self::Stream, cpal::BuildStreamError>
    where
        D: FnMut(&mut cpal::Data, &cpal::OutputCallbackInfo) + Send + 'static,
        E: FnMut(cpal::StreamError) + Send + 'static,
    {
        let controls = Arc::new(StreamControls {
            exit: AtomicBool::new(false),
            pause: AtomicBool::new(true), // streams are expected to start out paused by default
        });

        let thread_controls = controls.clone();
        let handle = std::thread::spawn(move || {
            let start = std::time::Instant::now();
            let mut buffer = [0.0_f32; 4096];
            while !thread_controls.exit.load(Ordering::Relaxed) {
                std::thread::sleep(std::time::Duration::from_secs_f32(
                    buffer.len() as f32 / 44100.0,
                ));
                // continue if paused
                if thread_controls.pause.load(Ordering::Relaxed) {
                    continue;
                }

                // data is cpal's way of having a type erased buffer.
                // you're expected to provide a raw pointer, the amount of samples, and the sample format of the buffer
                let mut data = unsafe {
                    cpal::Data::from_parts(
                        buffer.as_mut_ptr().cast(),
                        buffer.len(),
                        cpal::SampleFormat::F32,
                    )
                };

                let duration = std::time::Instant::now().duration_since(start);
                let secs = duration.as_nanos() / 1_000_000_000;
                let subsec_nanos = duration.as_nanos() - secs * 1_000_000_000;
                let stream_instant = cpal::StreamInstant::new(secs as _, subsec_nanos as _);
                let timestamp = cpal::OutputStreamTimestamp {
                    callback: stream_instant,
                    playback: stream_instant,
                };
                data_callback(&mut data, &cpal::OutputCallbackInfo::new(timestamp));

                let avg = buffer.iter().sum::<f32>() / buffer.len() as f32;
                println!("avg: {avg}");
            }
        });

        Ok(MyStream {
            controls,
            handle: Some(handle),
        })
    }
}

impl StreamTrait for MyStream {
    fn play(&self) -> Result<(), cpal::PlayStreamError> {
        self.controls.pause.store(false, Ordering::Relaxed);
        Ok(())
    }

    fn pause(&self) -> Result<(), cpal::PauseStreamError> {
        self.controls.pause.store(true, Ordering::Relaxed);
        Ok(())
    }
}

// streams are expected to stop when dropped
impl Drop for MyStream {
    fn drop(&mut self) {
        self.controls.exit.store(true, Ordering::Relaxed);
        let _ = self.handle.take().unwrap().join();
    }
}

#[cfg(feature = "custom")]
fn main() {
    let custom_host = cpal::platform::CustomHost::from_host(MyHost);
    // alternatively, use cpal::platform::CustomDevice and skip enumerating devices
    let host = cpal::Host::from(custom_host); // this host can be passed to rodio or any other crate that uses cpal

    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();

    let stream = make_stream(&device, &config.into()).unwrap();
    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(4000));
}

#[cfg(not(feature = "custom"))]
fn main() {
    panic!("please run with -F custom to try this example")
}

// rest of this example is mostly based off of synth_tones.rs

pub enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
}

pub struct Oscillator {
    pub sample_rate: f32,
    pub waveform: Waveform,
    pub current_sample_index: f32,
    pub frequency_hz: f32,
}

impl Oscillator {
    fn advance_sample(&mut self) {
        self.current_sample_index = (self.current_sample_index + 1.0) % self.sample_rate;
    }

    fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    fn calculate_sine_output_from_freq(&self, freq: f32) -> f32 {
        let two_pi = 2.0 * std::f32::consts::PI;
        (self.current_sample_index * freq * two_pi / self.sample_rate).sin()
    }

    fn is_multiple_of_freq_above_nyquist(&self, multiple: f32) -> bool {
        self.frequency_hz * multiple > self.sample_rate / 2.0
    }

    fn sine_wave(&mut self) -> f32 {
        self.advance_sample();
        self.calculate_sine_output_from_freq(self.frequency_hz)
    }

    fn generative_waveform(&mut self, harmonic_index_increment: i32, gain_exponent: f32) -> f32 {
        self.advance_sample();
        let mut output = 0.0;
        let mut i = 1;
        while !self.is_multiple_of_freq_above_nyquist(i as f32) {
            let gain = 1.0 / (i as f32).powf(gain_exponent);
            output += gain * self.calculate_sine_output_from_freq(self.frequency_hz * i as f32);
            i += harmonic_index_increment;
        }
        output
    }

    fn square_wave(&mut self) -> f32 {
        self.generative_waveform(2, 1.0)
    }

    fn saw_wave(&mut self) -> f32 {
        self.generative_waveform(1, 1.0)
    }

    fn triangle_wave(&mut self) -> f32 {
        self.generative_waveform(2, 2.0)
    }

    fn tick(&mut self) -> f32 {
        match self.waveform {
            Waveform::Sine => self.sine_wave(),
            Waveform::Square => self.square_wave(),
            Waveform::Saw => self.saw_wave(),
            Waveform::Triangle => self.triangle_wave(),
        }
    }
}

pub fn make_stream(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
) -> Result<cpal::Stream, anyhow::Error> {
    let num_channels = config.channels as usize;
    let mut oscillator = Oscillator {
        waveform: Waveform::Sine,
        sample_rate: config.sample_rate as f32,
        current_sample_index: 0.0,
        frequency_hz: 440.0,
    };
    let err_fn = |err| eprintln!("Error building output sound stream: {err}");

    let time_at_start = std::time::Instant::now();
    println!("Time at start: {time_at_start:?}");

    let stream = device.build_output_stream(
        config,
        move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // for 0-1s play sine, 1-2s play square, 2-3s play saw, 3-4s play triangle_wave
            let time_since_start = std::time::Instant::now()
                .duration_since(time_at_start)
                .as_secs_f32();
            if time_since_start < 1.0 {
                oscillator.set_waveform(Waveform::Sine);
            } else if time_since_start < 2.0 {
                oscillator.set_waveform(Waveform::Triangle);
            } else if time_since_start < 3.0 {
                oscillator.set_waveform(Waveform::Square);
            } else if time_since_start < 4.0 {
                oscillator.set_waveform(Waveform::Saw);
            } else {
                oscillator.set_waveform(Waveform::Sine);
            }
            process_frame(output, &mut oscillator, num_channels)
        },
        err_fn,
        None,
    )?;

    Ok(stream)
}

fn process_frame<SampleType>(
    output: &mut [SampleType],
    oscillator: &mut Oscillator,
    num_channels: usize,
) where
    SampleType: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(num_channels) {
        let value: SampleType = SampleType::from_sample(oscillator.tick());

        // copy the same value to all channels
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
