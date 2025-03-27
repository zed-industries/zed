use anyhow::{anyhow, Context as _, Result};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait as _};
use cpal::{StreamConfig, SupportedStreamConfig};
use futures::{Stream, StreamExt as _};
use gpui::{
    BackgroundExecutor, ScreenCaptureFrame, ScreenCaptureSource, ScreenCaptureStream, Task,
};
use libwebrtc::native::apm;
use livekit::track;

use livekit::webrtc::{
    audio_frame::AudioFrame,
    audio_source::{native::NativeAudioSource, AudioSourceOptions, RtcAudioSource},
    audio_stream::native::NativeAudioStream,
    video_frame::{VideoBuffer, VideoFrame, VideoRotation},
    video_source::{native::NativeVideoSource, RtcVideoSource, VideoResolution},
    video_stream::native::NativeVideoStream,
};
use parking_lot::Mutex;
use std::slice;
use std::time::Duration;
use std::{borrow::Cow, collections::VecDeque, sync::Arc, thread};
use util::{maybe, ResultExt as _};
use webrtc_sys::audio_mixer::ffi::AudioFrameInfo;

use crate::RemoteAudioTrack;

use super::LocalVideoTrack;

pub enum AudioStream {
    Input {
        _thread_handle: std::sync::mpsc::Sender<()>,
        _transmit_task: Task<()>,
    },
    Output {
        _task: Task<()>,
    },
}

pub(crate) async fn capture_local_video_track(
    capture_source: &dyn ScreenCaptureSource,
    cx: &mut gpui::AsyncApp,
) -> Result<(crate::LocalVideoTrack, Box<dyn ScreenCaptureStream>)> {
    let resolution = capture_source.resolution()?;
    let track_source = gpui_tokio::Tokio::spawn(cx, async move {
        NativeVideoSource::new(VideoResolution {
            width: resolution.width.0 as u32,
            height: resolution.height.0 as u32,
        })
    })?
    .await?;

    let capture_stream = capture_source
        .stream({
            let track_source = track_source.clone();
            Box::new(move |frame| {
                if let Some(buffer) = video_frame_buffer_to_webrtc(frame) {
                    track_source.capture_frame(&VideoFrame {
                        rotation: VideoRotation::VideoRotation0,
                        timestamp_us: 0,
                        buffer,
                    });
                }
            })
        })
        .await??;

    Ok((
        LocalVideoTrack(track::LocalVideoTrack::create_video_track(
            "screen share",
            RtcVideoSource::Native(track_source),
        )),
        capture_stream,
    ))
}

fn start_capture(
    apm: Arc<Mutex<apm::AudioProcessingModule>>,
    device: cpal::Device,
    config: SupportedStreamConfig,
    source: NativeAudioSource,
    background_executor: &BackgroundExecutor,
) -> (Task<()>, std::sync::mpsc::Sender<()>) {
    let (frame_tx, mut frame_rx) = futures::channel::mpsc::unbounded();
    let (thread_handle, thread_kill_rx) = std::sync::mpsc::channel::<()>();
    thread::spawn(move || {
        maybe!({
            if let Some(name) = device.name().ok() {
                log::info!("Using microphone: {}", name)
            } else {
                log::info!("Using microphone: <unknown>");
            }

            let sample_rate = config.sample_rate().0;
            let channels = config.channels() as u32;

            let ten_ms_buffer_size = (channels * sample_rate / 100) as usize;
            let mut buf: Vec<i16> = Vec::with_capacity(ten_ms_buffer_size);

            let stream = device
                .build_input_stream_raw(
                    &config.config(),
                    cpal::SampleFormat::I16,
                    move |data, _: &_| {
                        let mut data = data.as_slice::<i16>().unwrap();
                        while data.len() > 0 {
                            let remainder = (buf.capacity() - buf.len()).min(data.len());
                            buf.extend_from_slice(&data[..remainder]);
                            data = &data[remainder..];

                            if buf.capacity() == buf.len() {
                                frame_tx
                                    .unbounded_send(AudioFrame {
                                        data: Cow::Owned(std::mem::replace(
                                            &mut buf,
                                            Vec::with_capacity(ten_ms_buffer_size),
                                        )),
                                        sample_rate,
                                        num_channels: channels,
                                        samples_per_channel: buf.len() as u32 / channels,
                                    })
                                    .ok();
                            }
                        }
                    },
                    |err| log::error!("error capturing audio track: {:?}", err),
                    Some(Duration::from_millis(10)),
                )
                .context("failed to build input stream")?;

            stream.play()?;
            // Keep the thread alive and holding onto the `stream`
            thread_kill_rx.recv().ok();
            anyhow::Ok(Some(()))
        })
        .log_err();
    });

    let background_executor = background_executor.clone();
    let transmit_task = background_executor.spawn({
        let source = source.clone();
        async move {
            while let Some(mut frame) = frame_rx.next().await {
                apm.lock()
                    .process_stream(
                        frame.data.to_mut(),
                        frame.sample_rate as i32,
                        frame.num_channels as i32,
                    )
                    .log_err();
                source.capture_frame(&frame).await.log_err();
            }
        }
    });

    return (transmit_task, thread_handle);
}

pub(crate) fn capture_local_audio_track(
    apm: Arc<Mutex<apm::AudioProcessingModule>>,
    background_executor: &BackgroundExecutor,
) -> Result<(crate::LocalAudioTrack, AudioStream)> {
    let sample_rate;
    let channels;

    let (device, config) = default_device(true)?;
    sample_rate = config.sample_rate().0;
    channels = config.channels() as u32;
    let source = NativeAudioSource::new(
        // n.b. this struct's options are always ignored, noise cancellation is provided by apm.
        AudioSourceOptions::default(),
        sample_rate,
        channels,
        10,
    );
    let track = track::LocalAudioTrack::create_audio_track(
        "microphone",
        RtcAudioSource::Native(source.clone()),
    );
    let mut default_change_listener = DeviceChangeListener::new(false)?;

    let task = background_executor.spawn({
        let background_executor = background_executor.clone();
        async move {
            let (mut transmit_task, mut thread_handle) = start_capture(
                apm.clone(),
                device,
                config,
                source.clone(),
                &background_executor,
            );

            while let Some(_) = default_change_listener.next().await {
                let Some((device, config)) = default_device(true).log_err() else {
                    continue;
                };

                if let Ok(name) = device.name() {
                    log::info!("Using speaker: {}", name)
                } else {
                    log::info!("Using speaker: <unknown>")
                }

                (transmit_task, thread_handle) = start_capture(
                    apm.clone(),
                    device,
                    config,
                    source.clone(),
                    &background_executor,
                );
            }

            drop((transmit_task, thread_handle))
        }
    });

    Ok((
        super::LocalAudioTrack(track),
        AudioStream::Output { _task: task },
    ))
}

pub fn play_remote_audio_track(
    apm: Arc<Mutex<apm::AudioProcessingModule>>,
    track: &RemoteAudioTrack,
    background_executor: &BackgroundExecutor,
) -> Result<AudioStream> {
    let track = track.clone();
    // We track device changes in our output because Livekit has a resampler built in,
    // and it's easy to create a new native audio stream when the device changes.
    let mut default_change_listener = DeviceChangeListener::new(false)?;
    let (output_device, output_config) = default_device(false)?;

    let task = background_executor.spawn({
        let background_executor = background_executor.clone();
        async move {
            let (mut _receive_task, mut _thread) = start_output_stream(
                apm.clone(),
                output_config,
                output_device,
                &track.0,
                &background_executor,
            );

            while let Some(_) = default_change_listener.next().await {
                let Some((output_device, output_config)) = get_default_output().log_err() else {
                    continue;
                };

                if let Ok(name) = output_device.name() {
                    log::info!("Using speaker: {}", name)
                } else {
                    log::info!("Using speaker: <unknown>")
                }

                (_receive_task, _thread) = start_output_stream(
                    apm.clone(),
                    output_config,
                    output_device,
                    &track.0,
                    &background_executor,
                );
            }
        }
    });

    Ok(AudioStream::Output { _task: task })
}

fn default_device(input: bool) -> anyhow::Result<(cpal::Device, cpal::SupportedStreamConfig)> {
    let device;
    let config;
    if input {
        device = cpal::default_host()
            .default_input_device()
            .ok_or_else(|| anyhow!("no audio input device available"))?;
        config = device
            .default_input_config()
            .context("failed to get default input config")?;
    } else {
        device = cpal::default_host()
            .default_output_device()
            .ok_or_else(|| anyhow!("no audio output device available"))?;
        config = device
            .default_output_config()
            .context("failed to get default output config")?;
    }
    Ok((device, config))
}

fn get_default_output() -> anyhow::Result<(cpal::Device, cpal::SupportedStreamConfig)> {
    let host = cpal::default_host();
    let output_device = host
        .default_output_device()
        .context("failed to read default output device")?;
    let output_config = output_device.default_output_config()?;
    Ok((output_device, output_config))
}

#[derive(Clone)]
struct AudioMixerSource {
    ssrc: i32,
    sample_rate: u32,
    num_channels: u32,
    buffer: Arc<Mutex<VecDeque<Vec<i16>>>>,
}

impl AudioMixerSource {
    fn receive(&self, frame: AudioFrame) {
        assert_eq!(
            frame.data.len() as u32,
            self.sample_rate * self.num_channels / 100
        );

        let mut buffer = self.buffer.lock();
        buffer.push_back(frame.data.to_vec());
        while buffer.len() > 10 {
            dbg!("bye...");
            buffer.pop_front();
        }
    }
}

impl libwebrtc::native::audio_mixer::AudioMixerSource for AudioMixerSource {
    fn ssrc(&self) -> i32 {
        self.ssrc
    }

    fn preferred_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn get_audio_frame_with_info<'a>(&self, target_sample_rate: u32) -> Option<AudioFrame> {
        assert_eq!(self.sample_rate, target_sample_rate);
        let buf = self.buffer.lock().pop_front()?;
        Some(AudioFrame {
            data: Cow::Owned(buf),
            sample_rate: self.sample_rate,
            num_channels: self.num_channels,
            samples_per_channel: self.sample_rate / 100,
        })
    }
}

fn start_output_stream(
    apm: Arc<Mutex<apm::AudioProcessingModule>>,
    output_config: cpal::SupportedStreamConfig,
    output_device: cpal::Device,
    track: &track::RemoteAudioTrack,
    background_executor: &BackgroundExecutor,
) -> (Task<()>, std::sync::mpsc::Sender<()>) {
    let buffer = Arc::new(Mutex::new(VecDeque::<i16>::new()));
    // NOTE: the audio mixer can only do 16k, 32k, 48k
    // (and irritatingly, macOS seems to default to 44.1k)
    let sample_rate = 48000;

    let mut mixer = libwebrtc::native::audio_mixer::AudioMixer::new();
    let source = AudioMixerSource {
        ssrc: 1,
        sample_rate,
        num_channels: output_config.channels() as u32,
        buffer: Arc::default(),
    };

    let mut stream = NativeAudioStream::new(
        track.rtc_track(),
        sample_rate as i32,
        output_config.channels() as i32,
    );

    let receive_task = background_executor.spawn({
        let source = source.clone();
        async move {
            while let Some(frame) = stream.next().await {
                source.receive(frame);
            }
        }
    });
    mixer.add_source(source);
    let mut resampler = libwebrtc::native::audio_resampler::AudioResampler::default();

    // The _output_stream needs to be on it's own thread because it's !Send
    // and we experienced a deadlock when it's created on the main thread.
    let (thread, end_on_drop_rx) = std::sync::mpsc::channel::<()>();
    thread::spawn(move || {
        let output_stream = output_device.build_output_stream(
            &StreamConfig {
                channels: output_config.channels(),
                sample_rate: output_config.sample_rate(),
                // NOTE: all operations in WebRTC happen on 10ms chunk lengths.
                // We could set this to a multiple of 10ms..
                buffer_size: cpal::BufferSize::Fixed(output_config.sample_rate().0 as u32 / 100),
            },
            {
                move |data, _info| {
                    let mixed = mixer.mix(output_config.channels() as usize);
                    let sampled = resampler.remix_and_resample(
                        mixed,
                        sample_rate as u32 / 100,
                        output_config.channels() as u32,
                        sample_rate as u32,
                        output_config.channels() as u32,
                        output_config.sample_rate().0,
                    );
                    if sampled.len() < data.len() {
                        // Instead of partially filling a buffer, output silence. If a partial
                        // buffer was outputted then this could lead to a perpetual state of
                        // outputting partial buffers as it never gets filled enough for a full
                        // frame.
                        data.fill(0);
                    } else {
                        data.copy_from_slice(&sampled);
                    }
                    apm.lock()
                        .process_reverse_stream(
                            data,
                            output_config.sample_rate().0 as i32,
                            output_config.channels() as i32,
                        )
                        .ok();
                }
            },
            |error| log::error!("error playing audio track: {:?}", error),
            Some(Duration::from_millis(10)),
        );

        let Some(output_stream) = output_stream.log_err() else {
            return;
        };

        output_stream.play().log_err();
        // Block forever to keep the output stream alive
        end_on_drop_rx.recv().ok();
    });

    (receive_task, thread)
}

pub fn play_remote_video_track(
    track: &crate::RemoteVideoTrack,
) -> impl Stream<Item = RemoteVideoFrame> {
    #[cfg(target_os = "macos")]
    {
        let mut pool = None;
        let most_recent_frame_size = (0, 0);
        NativeVideoStream::new(track.0.rtc_track()).filter_map(move |frame| {
            if pool == None
                || most_recent_frame_size != (frame.buffer.width(), frame.buffer.height())
            {
                pool = create_buffer_pool(frame.buffer.width(), frame.buffer.height()).log_err();
            }
            let pool = pool.clone();
            async move { video_frame_buffer_from_webrtc(pool?, frame.buffer) }
        })
    }
    #[cfg(not(target_os = "macos"))]
    {
        NativeVideoStream::new(track.0.rtc_track())
            .filter_map(|frame| async move { video_frame_buffer_from_webrtc(frame.buffer) })
    }
}

#[cfg(target_os = "macos")]
fn create_buffer_pool(
    width: u32,
    height: u32,
) -> Result<core_video::pixel_buffer_pool::CVPixelBufferPool> {
    use core_foundation::{base::TCFType, number::CFNumber, string::CFString};
    use core_video::pixel_buffer;
    use core_video::{
        pixel_buffer::kCVPixelFormatType_420YpCbCr8BiPlanarFullRange,
        pixel_buffer_io_surface::kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey,
        pixel_buffer_pool::{self},
    };

    let width_key: CFString =
        unsafe { CFString::wrap_under_get_rule(pixel_buffer::kCVPixelBufferWidthKey) };
    let height_key: CFString =
        unsafe { CFString::wrap_under_get_rule(pixel_buffer::kCVPixelBufferHeightKey) };
    let animation_key: CFString = unsafe {
        CFString::wrap_under_get_rule(kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey)
    };
    let format_key: CFString =
        unsafe { CFString::wrap_under_get_rule(pixel_buffer::kCVPixelBufferPixelFormatTypeKey) };

    let yes: CFNumber = 1.into();
    let width: CFNumber = (width as i32).into();
    let height: CFNumber = (height as i32).into();
    let format: CFNumber = (kCVPixelFormatType_420YpCbCr8BiPlanarFullRange as i64).into();

    let buffer_attributes = core_foundation::dictionary::CFDictionary::from_CFType_pairs(&[
        (width_key, width.into_CFType()),
        (height_key, height.into_CFType()),
        (animation_key, yes.into_CFType()),
        (format_key, format.into_CFType()),
    ]);

    pixel_buffer_pool::CVPixelBufferPool::new(None, Some(&buffer_attributes)).map_err(|cv_return| {
        anyhow!(
            "failed to create pixel buffer pool: CVReturn({})",
            cv_return
        )
    })
}

#[cfg(target_os = "macos")]
pub type RemoteVideoFrame = core_video::pixel_buffer::CVPixelBuffer;

#[cfg(target_os = "macos")]
fn video_frame_buffer_from_webrtc(
    pool: core_video::pixel_buffer_pool::CVPixelBufferPool,
    buffer: Box<dyn VideoBuffer>,
) -> Option<RemoteVideoFrame> {
    use core_foundation::base::TCFType;
    use core_video::{pixel_buffer::CVPixelBuffer, r#return::kCVReturnSuccess};
    use livekit::webrtc::native::yuv_helper::i420_to_nv12;

    if let Some(native) = buffer.as_native() {
        let pixel_buffer = native.get_cv_pixel_buffer();
        if pixel_buffer.is_null() {
            return None;
        }
        return unsafe { Some(CVPixelBuffer::wrap_under_get_rule(pixel_buffer as _)) };
    }

    let i420_buffer = buffer.as_i420()?;
    let pixel_buffer = pool.create_pixel_buffer().log_err()?;

    let image_buffer = unsafe {
        if pixel_buffer.lock_base_address(0) != kCVReturnSuccess {
            return None;
        }

        let dst_y = pixel_buffer.get_base_address_of_plane(0);
        let dst_y_stride = pixel_buffer.get_bytes_per_row_of_plane(0);
        let dst_y_len = pixel_buffer.get_height_of_plane(0) * dst_y_stride;
        let dst_uv = pixel_buffer.get_base_address_of_plane(1);
        let dst_uv_stride = pixel_buffer.get_bytes_per_row_of_plane(1);
        let dst_uv_len = pixel_buffer.get_height_of_plane(1) * dst_uv_stride;
        let width = pixel_buffer.get_width();
        let height = pixel_buffer.get_height();
        let dst_y_buffer = std::slice::from_raw_parts_mut(dst_y as *mut u8, dst_y_len);
        let dst_uv_buffer = std::slice::from_raw_parts_mut(dst_uv as *mut u8, dst_uv_len);

        let (stride_y, stride_u, stride_v) = i420_buffer.strides();
        let (src_y, src_u, src_v) = i420_buffer.data();
        i420_to_nv12(
            src_y,
            stride_y,
            src_u,
            stride_u,
            src_v,
            stride_v,
            dst_y_buffer,
            dst_y_stride as u32,
            dst_uv_buffer,
            dst_uv_stride as u32,
            width as i32,
            height as i32,
        );

        if pixel_buffer.unlock_base_address(0) != kCVReturnSuccess {
            return None;
        }

        pixel_buffer
    };

    Some(image_buffer)
}

#[cfg(not(target_os = "macos"))]
pub type RemoteVideoFrame = Arc<gpui::RenderImage>;

#[cfg(not(target_os = "macos"))]
fn video_frame_buffer_from_webrtc(buffer: Box<dyn VideoBuffer>) -> Option<RemoteVideoFrame> {
    use gpui::RenderImage;
    use image::{Frame, RgbaImage};
    use livekit::webrtc::prelude::VideoFormatType;
    use smallvec::SmallVec;
    use std::alloc::{alloc, Layout};

    let width = buffer.width();
    let height = buffer.height();
    let stride = width * 4;
    let byte_len = (stride * height) as usize;
    let argb_image = unsafe {
        // Motivation for this unsafe code is to avoid initializing the frame data, since to_argb
        // will write all bytes anyway.
        let start_ptr = alloc(Layout::array::<u8>(byte_len).log_err()?);
        if start_ptr.is_null() {
            return None;
        }
        let bgra_frame_slice = std::slice::from_raw_parts_mut(start_ptr, byte_len);
        buffer.to_argb(
            VideoFormatType::ARGB, // For some reason, this displays correctly while RGBA (the correct format) does not
            bgra_frame_slice,
            stride,
            width as i32,
            height as i32,
        );
        Vec::from_raw_parts(start_ptr, byte_len, byte_len)
    };

    Some(Arc::new(RenderImage::new(SmallVec::from_elem(
        Frame::new(
            RgbaImage::from_raw(width, height, argb_image)
                .with_context(|| "Bug: not enough bytes allocated for image.")
                .log_err()?,
        ),
        1,
    ))))
}

#[cfg(target_os = "macos")]
fn video_frame_buffer_to_webrtc(frame: ScreenCaptureFrame) -> Option<impl AsRef<dyn VideoBuffer>> {
    use livekit::webrtc;

    let pixel_buffer = frame.0.as_concrete_TypeRef();
    std::mem::forget(frame.0);
    unsafe {
        Some(webrtc::video_frame::native::NativeBuffer::from_cv_pixel_buffer(pixel_buffer as _))
    }
}

#[cfg(not(target_os = "macos"))]
fn video_frame_buffer_to_webrtc(_frame: ScreenCaptureFrame) -> Option<impl AsRef<dyn VideoBuffer>> {
    None as Option<Box<dyn VideoBuffer>>
}

trait DeviceChangeListenerApi: Stream<Item = ()> + Sized {
    fn new(input: bool) -> Result<Self>;
}

#[cfg(target_os = "macos")]
mod macos {

    use coreaudio::sys::{
        kAudioHardwarePropertyDefaultInputDevice, kAudioHardwarePropertyDefaultOutputDevice,
        kAudioObjectPropertyElementMaster, kAudioObjectPropertyScopeGlobal,
        kAudioObjectSystemObject, AudioObjectAddPropertyListener, AudioObjectID,
        AudioObjectPropertyAddress, AudioObjectRemovePropertyListener, OSStatus,
    };
    use futures::{channel::mpsc::UnboundedReceiver, StreamExt};

    /// Implementation from: https://github.com/zed-industries/cpal/blob/fd8bc2fd39f1f5fdee5a0690656caff9a26d9d50/src/host/coreaudio/macos/property_listener.rs#L15
    pub struct CoreAudioDefaultDeviceChangeListener {
        rx: UnboundedReceiver<()>,
        callback: Box<PropertyListenerCallbackWrapper>,
        input: bool,
    }

    trait _AssertSend: Send {}
    impl _AssertSend for CoreAudioDefaultDeviceChangeListener {}

    struct PropertyListenerCallbackWrapper(Box<dyn FnMut() + Send>);

    unsafe extern "C" fn property_listener_handler_shim(
        _: AudioObjectID,
        _: u32,
        _: *const AudioObjectPropertyAddress,
        callback: *mut ::std::os::raw::c_void,
    ) -> OSStatus {
        let wrapper = callback as *mut PropertyListenerCallbackWrapper;
        (*wrapper).0();
        0
    }

    impl super::DeviceChangeListenerApi for CoreAudioDefaultDeviceChangeListener {
        fn new(input: bool) -> gpui::Result<Self> {
            let (tx, rx) = futures::channel::mpsc::unbounded();

            let callback = Box::new(PropertyListenerCallbackWrapper(Box::new(move || {
                tx.unbounded_send(()).ok();
            })));

            unsafe {
                coreaudio::Error::from_os_status(AudioObjectAddPropertyListener(
                    kAudioObjectSystemObject,
                    &AudioObjectPropertyAddress {
                        mSelector: if input {
                            kAudioHardwarePropertyDefaultInputDevice
                        } else {
                            kAudioHardwarePropertyDefaultOutputDevice
                        },
                        mScope: kAudioObjectPropertyScopeGlobal,
                        mElement: kAudioObjectPropertyElementMaster,
                    },
                    Some(property_listener_handler_shim),
                    &*callback as *const _ as *mut _,
                ))?;
            }

            Ok(Self {
                rx,
                callback,
                input,
            })
        }
    }

    impl Drop for CoreAudioDefaultDeviceChangeListener {
        fn drop(&mut self) {
            unsafe {
                AudioObjectRemovePropertyListener(
                    kAudioObjectSystemObject,
                    &AudioObjectPropertyAddress {
                        mSelector: if self.input {
                            kAudioHardwarePropertyDefaultInputDevice
                        } else {
                            kAudioHardwarePropertyDefaultOutputDevice
                        },
                        mScope: kAudioObjectPropertyScopeGlobal,
                        mElement: kAudioObjectPropertyElementMaster,
                    },
                    Some(property_listener_handler_shim),
                    &*self.callback as *const _ as *mut _,
                );
            }
        }
    }

    impl futures::Stream for CoreAudioDefaultDeviceChangeListener {
        type Item = ();

        fn poll_next(
            mut self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Option<Self::Item>> {
            self.rx.poll_next_unpin(cx)
        }
    }
}

#[cfg(target_os = "macos")]
type DeviceChangeListener = macos::CoreAudioDefaultDeviceChangeListener;

#[cfg(not(target_os = "macos"))]
mod noop_change_listener {
    use std::task::Poll;

    use super::DeviceChangeListenerApi;

    pub struct NoopOutputDeviceChangelistener {}

    impl DeviceChangeListenerApi for NoopOutputDeviceChangelistener {
        fn new(_input: bool) -> anyhow::Result<Self> {
            Ok(NoopOutputDeviceChangelistener {})
        }
    }

    impl futures::Stream for NoopOutputDeviceChangelistener {
        type Item = ();

        fn poll_next(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> Poll<Option<Self::Item>> {
            Poll::Pending
        }
    }
}

#[cfg(not(target_os = "macos"))]
type DeviceChangeListener = noop_change_listener::NoopOutputDeviceChangelistener;

pub(crate) async fn capture_local_wav_track(
    apm: Arc<Mutex<apm::AudioProcessingModule>>,
    background_executor: &BackgroundExecutor,
) -> Result<(crate::LocalAudioTrack, AudioStream)> {
    let file = tokio::fs::File::open("change-sophie.wav").await?;
    let mut reader = WavReader::new(BufReader::new(file));
    let header = reader.read_header().await?;

    let source = NativeAudioSource::new(
        AudioSourceOptions::default(),
        header.sample_rate,
        header.num_channels as u32,
        1000,
    );
    let track = LocalAudioTrack::create_audio_track("file", RtcAudioSource::Native(source.clone()));
    // Play the wav file and disconnect
    tokio::spawn({
        async move {
            thread::sleep(Duration::from_millis(1000));
            const FRAME_DURATION: Duration = Duration::from_millis(1000); // Write 1s of audio at a time

            let max_samples = header.data_size as usize / size_of::<i16>();
            let ms = FRAME_DURATION.as_millis() as u32;
            let num_samples = (header.sample_rate / 1000 * ms) as usize;

            log::info!("sample_rate: {}", header.sample_rate);
            log::info!("num_channels: {}", header.num_channels);
            log::info!("max samples: {}", max_samples);
            log::info!("chunk size: {}ms - {} samples", ms, num_samples);

            let mut written_samples = 0;
            while written_samples < max_samples {
                let available_samples = max_samples - written_samples;
                let frame_size = num_samples.min(available_samples);

                let mut audio_frame = AudioFrame {
                    data: vec![0i16; frame_size].into(),
                    num_channels: header.num_channels as u32,
                    sample_rate: header.sample_rate,
                    samples_per_channel: (frame_size / header.num_channels as usize) as u32,
                };

                for i in 0..frame_size {
                    let sample = reader.read_i16().await.unwrap();
                    audio_frame.data.to_mut()[i] = sample;
                }

                dbg!("wav");
                source.capture_frame(&audio_frame).await.unwrap();
                written_samples += frame_size;
            }
        }
    });

    Ok((
        super::LocalAudioTrack(track),
        AudioStream::Output {
            _task: Task::ready(()),
        },
    ))
}

use livekit::track::LocalAudioTrack;
use std::{io::SeekFrom, mem::size_of};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt, BufReader};

pub struct WavReader<R: AsyncRead + AsyncSeek + Unpin> {
    reader: R,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct WavHeader {
    file_size: u32,
    data_size: u32,
    format: String,
    format_length: u32,
    format_type: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
}

impl<R: AsyncRead + AsyncSeek + Unpin> WavReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub async fn read_header(&mut self) -> Result<WavHeader> {
        let mut header = [0u8; 4];
        let mut format = [0u8; 4];
        let mut chunk_marker = [0u8; 4];
        let mut data_chunk = [0u8; 4];

        self.reader.read_exact(&mut header).await?;

        if &header != b"RIFF" {
            anyhow::bail!("Invalid RIFF header");
        }

        let file_size = self.reader.read_u32_le().await?;
        self.reader.read_exact(&mut format).await?;

        if &format != b"WAVE" {
            anyhow::bail!("Invalid WAVE header");
        }

        self.reader.read_exact(&mut chunk_marker).await?;

        if &chunk_marker != b"fmt " {
            anyhow::bail!("Invalid fmt chunk");
        }

        let format_length = self.reader.read_u32_le().await?;
        let format_type = self.reader.read_u16_le().await?;
        let num_channels = self.reader.read_u16_le().await?;
        let sample_rate = self.reader.read_u32_le().await?;
        let byte_rate = self.reader.read_u32_le().await?;
        let block_align = self.reader.read_u16_le().await?;
        let bits_per_sample = self.reader.read_u16_le().await?;

        if bits_per_sample != 16 {
            anyhow::bail!("only 16-bit samples supported");
        }

        let mut data_size;
        loop {
            self.reader.read_exact(&mut data_chunk).await?;
            data_size = self.reader.read_u32_le().await?;

            if &data_chunk == b"data" {
                break;
            } else {
                // skip non data chunks
                self.reader
                    .seek(SeekFrom::Current(data_size.into()))
                    .await?;
            }
        }

        if &data_chunk != b"data" {
            anyhow::bail!("Invalid data chunk");
        }

        Ok(WavHeader {
            file_size,
            data_size,
            format: String::from_utf8_lossy(&format).to_string(),
            format_length,
            format_type,
            num_channels,
            sample_rate,
            byte_rate,
            block_align,
            bits_per_sample,
        })
    }

    pub async fn read_i16(&mut self) -> Result<i16> {
        let i = self.reader.read_i16_le().await?;
        Ok(i)
    }
}
