use anyhow::{Context as _, Result};

use audio::{AudioSettings, CHANNEL_COUNT, LEGACY_CHANNEL_COUNT, LEGACY_SAMPLE_RATE, SAMPLE_RATE};
use cpal::traits::{DeviceTrait, StreamTrait as _};
use futures::channel::mpsc::UnboundedSender;
use futures::{Stream, StreamExt as _};
use gpui::{
    AsyncApp, BackgroundExecutor, ScreenCaptureFrame, ScreenCaptureSource, ScreenCaptureStream,
    Task,
};
use libwebrtc::native::{apm, audio_mixer, audio_resampler};
use livekit::track;

use livekit::webrtc::{
    audio_frame::AudioFrame,
    audio_source::{AudioSourceOptions, RtcAudioSource, native::NativeAudioSource},
    audio_stream::native::NativeAudioStream,
    video_frame::{VideoBuffer, VideoFrame, VideoRotation},
    video_source::{RtcVideoSource, VideoResolution, native::NativeVideoSource},
    video_stream::native::NativeVideoStream,
};
use log::info;
use parking_lot::Mutex;
use rodio::Source;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::cell::RefCell;
use std::sync::Weak;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::time::Duration;
use std::{borrow::Cow, collections::VecDeque, sync::Arc, thread};
use util::{ResultExt as _, maybe};

mod source;

pub(crate) struct AudioStack {
    executor: BackgroundExecutor,
    apm: Arc<Mutex<apm::AudioProcessingModule>>,
    mixer: Arc<Mutex<audio_mixer::AudioMixer>>,
    _output_task: RefCell<Weak<Task<()>>>,
    next_ssrc: AtomicI32,
}

pub(crate) fn play_remote_audio_track(
    track: &livekit::track::RemoteAudioTrack,
    speaker: Speaker,
    cx: &mut gpui::App,
) -> Result<AudioStream> {
    let stream = source::LiveKitStream::new(
        cx.background_executor(),
        track,
        speaker.legacy_audio_compatible,
    );

    let stop_handle = Arc::new(AtomicBool::new(false));
    let stop_handle_clone = stop_handle.clone();
    let stream = stream
        .stoppable()
        .periodic_access(Duration::from_millis(50), move |s| {
            if stop_handle.load(Ordering::Relaxed) {
                s.stop();
            }
        });

    audio::Audio::play_voip_stream(stream, speaker.name, speaker.is_staff, cx)
        .context("Could not play audio")?;

    let on_drop = util::defer(move || {
        stop_handle_clone.store(true, Ordering::Relaxed);
    });
    Ok(AudioStream::Output {
        _drop: Box::new(on_drop),
    })
}

impl AudioStack {
    pub(crate) fn new(executor: BackgroundExecutor) -> Self {
        let apm = Arc::new(Mutex::new(apm::AudioProcessingModule::new(
            true, true, true, true,
        )));
        let mixer = Arc::new(Mutex::new(audio_mixer::AudioMixer::new()));
        Self {
            executor,
            apm,
            mixer,
            _output_task: RefCell::new(Weak::new()),
            next_ssrc: AtomicI32::new(1),
        }
    }

    pub(crate) fn play_remote_audio_track(
        &self,
        track: &livekit::track::RemoteAudioTrack,
    ) -> AudioStream {
        let output_task = self.start_output();

        let next_ssrc = self.next_ssrc.fetch_add(1, Ordering::Relaxed);
        let source = AudioMixerSource {
            ssrc: next_ssrc,
            sample_rate: SAMPLE_RATE.get(),
            num_channels: CHANNEL_COUNT.get() as u32,
            buffer: Arc::default(),
        };
        self.mixer.lock().add_source(source.clone());

        let mut stream = NativeAudioStream::new(
            track.rtc_track(),
            source.sample_rate as i32,
            source.num_channels as i32,
        );

        let receive_task = self.executor.spawn({
            let source = source.clone();
            async move {
                while let Some(frame) = stream.next().await {
                    source.receive(frame);
                }
            }
        });

        let mixer = self.mixer.clone();
        let on_drop = util::defer(move || {
            mixer.lock().remove_source(source.ssrc);
            drop(receive_task);
            drop(output_task);
        });

        AudioStream::Output {
            _drop: Box::new(on_drop),
        }
    }

    fn start_output(&self) -> Arc<Task<()>> {
        if let Some(task) = self._output_task.borrow().upgrade() {
            return task;
        }
        let task = Arc::new(self.executor.spawn({
            let apm = self.apm.clone();
            let mixer = self.mixer.clone();
            async move {
                Self::play_output(apm, mixer, SAMPLE_RATE.get(), CHANNEL_COUNT.get().into())
                    .await
                    .log_err();
            }
        }));
        *self._output_task.borrow_mut() = Arc::downgrade(&task);
        task
    }

    pub(crate) fn capture_local_microphone_track(
        &self,
        user_name: String,
        is_staff: bool,
        cx: &AsyncApp,
    ) -> Result<(crate::LocalAudioTrack, AudioStream)> {
        let legacy_audio_compatible =
            AudioSettings::try_read_global(cx, |setting| setting.legacy_audio_compatible)
                .unwrap_or_default();

        let source = if legacy_audio_compatible {
            NativeAudioSource::new(
                // n.b. this struct's options are always ignored, noise cancellation is provided by apm.
                AudioSourceOptions::default(),
                LEGACY_SAMPLE_RATE.get(),
                LEGACY_CHANNEL_COUNT.get().into(),
                10,
            )
        } else {
            NativeAudioSource::new(
                // n.b. this struct's options are always ignored, noise cancellation is provided by apm.
                AudioSourceOptions::default(),
                SAMPLE_RATE.get(),
                CHANNEL_COUNT.get().into(),
                10,
            )
        };

        let track_name = serde_urlencoded::to_string(Speaker {
            name: user_name,
            is_staff,
            legacy_audio_compatible,
        })
        .context("Could not encode user information in track name")?;

        let track = track::LocalAudioTrack::create_audio_track(
            &track_name,
            RtcAudioSource::Native(source.clone()),
        );

        let apm = self.apm.clone();

        let (frame_tx, mut frame_rx) = futures::channel::mpsc::unbounded();
        let transmit_task = self.executor.spawn({
            async move {
                while let Some(frame) = frame_rx.next().await {
                    source.capture_frame(&frame).await.log_err();
                }
            }
        });
        let rodio_pipeline =
            AudioSettings::try_read_global(cx, |setting| setting.rodio_audio).unwrap_or_default();
        let capture_task = if rodio_pipeline {
            info!("Using experimental.rodio_audio audio pipeline");
            let voip_parts = audio::VoipParts::new(cx)?;
            // Audio needs to run real-time and should never be paused. That is
            // why we are using a normal std::thread and not a background task
            thread::Builder::new()
                .name("MicrophoneToLivekit".to_string())
                .spawn(move || {
                    // microphone is non send on mac
                    let microphone = match audio::Audio::open_microphone(voip_parts) {
                        Ok(m) => m,
                        Err(e) => {
                            log::error!("Could not open microphone: {e}");
                            return;
                        }
                    };
                    send_to_livekit(frame_tx, microphone);
                })
                .expect("should be able to spawn threads");
            Task::ready(Ok(()))
        } else {
            self.executor.spawn(async move {
                Self::capture_input(
                    apm,
                    frame_tx,
                    LEGACY_SAMPLE_RATE.get(),
                    LEGACY_CHANNEL_COUNT.get().into(),
                )
                .await
            })
        };

        let on_drop = util::defer(|| {
            drop(transmit_task);
            drop(capture_task);
        });
        Ok((
            super::LocalAudioTrack(track),
            AudioStream::Output {
                _drop: Box::new(on_drop),
            },
        ))
    }

    async fn play_output(
        apm: Arc<Mutex<apm::AudioProcessingModule>>,
        mixer: Arc<Mutex<audio_mixer::AudioMixer>>,
        sample_rate: u32,
        num_channels: u32,
    ) -> Result<()> {
        loop {
            let mut device_change_listener = DeviceChangeListener::new(false)?;
            let (output_device, output_config) = crate::default_device(false)?;
            let (end_on_drop_tx, end_on_drop_rx) = std::sync::mpsc::channel::<()>();
            let mixer = mixer.clone();
            let apm = apm.clone();
            let mut resampler = audio_resampler::AudioResampler::default();
            let mut buf = Vec::new();

            thread::Builder::new()
                .name("AudioPlayback".to_owned())
                .spawn(move || {
                    let output_stream = output_device.build_output_stream(
                        &output_config.config(),
                        {
                            move |mut data, _info| {
                                while data.len() > 0 {
                                    if data.len() <= buf.len() {
                                        let rest = buf.split_off(data.len());
                                        data.copy_from_slice(&buf);
                                        buf = rest;
                                        return;
                                    }
                                    if buf.len() > 0 {
                                        let (prefix, suffix) = data.split_at_mut(buf.len());
                                        prefix.copy_from_slice(&buf);
                                        data = suffix;
                                    }

                                    let mut mixer = mixer.lock();
                                    let mixed = mixer.mix(output_config.channels() as usize);
                                    let sampled = resampler.remix_and_resample(
                                        mixed,
                                        sample_rate / 100,
                                        num_channels,
                                        sample_rate,
                                        output_config.channels() as u32,
                                        output_config.sample_rate().0,
                                    );
                                    buf = sampled.to_vec();
                                    apm.lock()
                                        .process_reverse_stream(
                                            &mut buf,
                                            output_config.sample_rate().0 as i32,
                                            output_config.channels() as i32,
                                        )
                                        .ok();
                                }
                            }
                        },
                        |error| log::error!("error playing audio track: {:?}", error),
                        Some(Duration::from_millis(100)),
                    );

                    let Some(output_stream) = output_stream.log_err() else {
                        return;
                    };

                    output_stream.play().log_err();
                    // Block forever to keep the output stream alive
                    end_on_drop_rx.recv().ok();
                })
                .unwrap();

            device_change_listener.next().await;
            drop(end_on_drop_tx)
        }
    }

    async fn capture_input(
        apm: Arc<Mutex<apm::AudioProcessingModule>>,
        frame_tx: UnboundedSender<AudioFrame<'static>>,
        sample_rate: u32,
        num_channels: u32,
    ) -> Result<()> {
        loop {
            let mut device_change_listener = DeviceChangeListener::new(true)?;
            let (device, config) = crate::default_device(true)?;
            let (end_on_drop_tx, end_on_drop_rx) = std::sync::mpsc::channel::<()>();
            let apm = apm.clone();
            let frame_tx = frame_tx.clone();
            let mut resampler = audio_resampler::AudioResampler::default();

            thread::Builder::new()
                .name("AudioCapture".to_owned())
                .spawn(move || {
                    maybe!({
                        if let Some(name) = device.name().ok() {
                            log::info!("Using microphone: {}", name)
                        } else {
                            log::info!("Using microphone: <unknown>");
                        }

                        let ten_ms_buffer_size =
                            (config.channels() as u32 * config.sample_rate().0 / 100) as usize;
                        let mut buf: Vec<i16> = Vec::with_capacity(ten_ms_buffer_size);

                        let stream = device
                            .build_input_stream_raw(
                                &config.config(),
                                config.sample_format(),
                                move |data, _: &_| {
                                    let data = crate::get_sample_data(config.sample_format(), data)
                                        .log_err();
                                    let Some(data) = data else {
                                        return;
                                    };
                                    let mut data = data.as_slice();

                                    while data.len() > 0 {
                                        let remainder =
                                            (buf.capacity() - buf.len()).min(data.len());
                                        buf.extend_from_slice(&data[..remainder]);
                                        data = &data[remainder..];

                                        if buf.capacity() == buf.len() {
                                            let mut sampled = resampler
                                                .remix_and_resample(
                                                    buf.as_slice(),
                                                    config.sample_rate().0 / 100,
                                                    config.channels() as u32,
                                                    config.sample_rate().0,
                                                    num_channels,
                                                    sample_rate,
                                                )
                                                .to_owned();
                                            apm.lock()
                                                .process_stream(
                                                    &mut sampled,
                                                    sample_rate as i32,
                                                    num_channels as i32,
                                                )
                                                .log_err();
                                            buf.clear();
                                            frame_tx
                                                .unbounded_send(AudioFrame {
                                                    data: Cow::Owned(sampled),
                                                    sample_rate,
                                                    num_channels,
                                                    samples_per_channel: sample_rate / 100,
                                                })
                                                .ok();
                                        }
                                    }
                                },
                                |err| log::error!("error capturing audio track: {:?}", err),
                                Some(Duration::from_millis(100)),
                            )
                            .context("failed to build input stream")?;

                        stream.play()?;
                        // Keep the thread alive and holding onto the `stream`
                        end_on_drop_rx.recv().ok();
                        anyhow::Ok(Some(()))
                    })
                    .log_err();
                })
                .unwrap();

            device_change_listener.next().await;
            drop(end_on_drop_tx)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Speaker {
    pub name: String,
    pub is_staff: bool,
    pub legacy_audio_compatible: bool,
}

fn send_to_livekit(frame_tx: UnboundedSender<AudioFrame<'static>>, mut microphone: impl Source) {
    use cpal::Sample;
    let sample_rate = microphone.sample_rate().get();
    let num_channels = microphone.channels().get() as u32;
    let buffer_size = sample_rate / 100 * num_channels;

    loop {
        let sampled: Vec<_> = microphone
            .by_ref()
            .take(buffer_size as usize)
            .map(|s| s.to_sample())
            .collect();

        if frame_tx
            .unbounded_send(AudioFrame {
                sample_rate,
                num_channels,
                samples_per_channel: sampled.len() as u32 / num_channels,
                data: Cow::Owned(sampled),
            })
            .is_err()
        {
            // must rx has dropped or is not consuming
            break;
        }
    }
}

use super::LocalVideoTrack;

pub enum AudioStream {
    Input { _task: Task<()> },
    Output { _drop: Box<dyn std::any::Any> },
}

pub(crate) async fn capture_local_video_track(
    capture_source: &dyn ScreenCaptureSource,
    cx: &mut gpui::AsyncApp,
) -> Result<(crate::LocalVideoTrack, Box<dyn ScreenCaptureStream>)> {
    let metadata = capture_source.metadata()?;
    let track_source = gpui_tokio::Tokio::spawn(cx, async move {
        NativeVideoSource::new(VideoResolution {
            width: metadata.resolution.width.0 as u32,
            height: metadata.resolution.height.0 as u32,
        })
    })?
    .await?;

    let capture_stream = capture_source
        .stream(cx.foreground_executor(), {
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

    fn get_audio_frame_with_info<'a>(&self, target_sample_rate: u32) -> Option<AudioFrame<'_>> {
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

pub fn play_remote_video_track(
    track: &crate::RemoteVideoTrack,
) -> impl Stream<Item = RemoteVideoFrame> + use<> {
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
            async move {
                if frame.buffer.width() < 10 && frame.buffer.height() < 10 {
                    // when the remote stops sharing, we get an 8x8 black image.
                    // In a lil bit, the unpublish will come through and close the view,
                    // but until then, don't flash black.
                    return None;
                }

                video_frame_buffer_from_webrtc(pool?, frame.buffer)
            }
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
        anyhow::anyhow!("failed to create pixel buffer pool: CVReturn({cv_return})",)
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
    use std::alloc::{Layout, alloc};

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
        let argb_frame_slice = std::slice::from_raw_parts_mut(start_ptr, byte_len);
        buffer.to_argb(
            VideoFormatType::ARGB,
            argb_frame_slice,
            stride,
            width as i32,
            height as i32,
        );
        Vec::from_raw_parts(start_ptr, byte_len, byte_len)
    };

    // TODO: Unclear why providing argb_image to RgbaImage works properly.
    let image = RgbaImage::from_raw(width, height, argb_image)
        .with_context(|| "Bug: not enough bytes allocated for image.")
        .log_err()?;

    Some(Arc::new(RenderImage::new(SmallVec::from_elem(
        Frame::new(image),
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
fn video_frame_buffer_to_webrtc(frame: ScreenCaptureFrame) -> Option<impl AsRef<dyn VideoBuffer>> {
    use libwebrtc::native::yuv_helper::{abgr_to_nv12, argb_to_nv12};
    use livekit::webrtc::prelude::NV12Buffer;
    match frame.0 {
        scap::frame::Frame::BGRx(frame) => {
            let mut buffer = NV12Buffer::new(frame.width as u32, frame.height as u32);
            let (stride_y, stride_uv) = buffer.strides();
            let (data_y, data_uv) = buffer.data_mut();
            argb_to_nv12(
                &frame.data,
                frame.width as u32 * 4,
                data_y,
                stride_y,
                data_uv,
                stride_uv,
                frame.width,
                frame.height,
            );
            Some(buffer)
        }
        scap::frame::Frame::RGBx(frame) => {
            let mut buffer = NV12Buffer::new(frame.width as u32, frame.height as u32);
            let (stride_y, stride_uv) = buffer.strides();
            let (data_y, data_uv) = buffer.data_mut();
            abgr_to_nv12(
                &frame.data,
                frame.width as u32 * 4,
                data_y,
                stride_y,
                data_uv,
                stride_uv,
                frame.width,
                frame.height,
            );
            Some(buffer)
        }
        scap::frame::Frame::YUVFrame(yuvframe) => {
            let mut buffer = NV12Buffer::with_strides(
                yuvframe.width as u32,
                yuvframe.height as u32,
                yuvframe.luminance_stride as u32,
                yuvframe.chrominance_stride as u32,
            );
            let (luminance, chrominance) = buffer.data_mut();
            luminance.copy_from_slice(yuvframe.luminance_bytes.as_slice());
            chrominance.copy_from_slice(yuvframe.chrominance_bytes.as_slice());
            Some(buffer)
        }
        _ => {
            log::error!(
                "Expected BGRx or YUV frame from scap screen capture but got some other format."
            );
            None
        }
    }
}

trait DeviceChangeListenerApi: Stream<Item = ()> + Sized {
    fn new(input: bool) -> Result<Self>;
}

#[cfg(target_os = "macos")]
mod macos {

    use coreaudio::sys::{
        AudioObjectAddPropertyListener, AudioObjectID, AudioObjectPropertyAddress,
        AudioObjectRemovePropertyListener, OSStatus, kAudioHardwarePropertyDefaultInputDevice,
        kAudioHardwarePropertyDefaultOutputDevice, kAudioObjectPropertyElementMaster,
        kAudioObjectPropertyScopeGlobal, kAudioObjectSystemObject,
    };
    use futures::{StreamExt, channel::mpsc::UnboundedReceiver};

    /// Implementation from: https://github.com/zed-industries/cpal/blob/fd8bc2fd39f1f5fdee5a0690656caff9a26d9d50/src/host/coreaudio/macos/property_listener.rs#L15
    pub struct CoreAudioDefaultDeviceChangeListener {
        rx: UnboundedReceiver<()>,
        callback: Box<PropertyListenerCallbackWrapper>,
        input: bool,
        device_id: AudioObjectID, // Store the device ID to properly remove listeners
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
        unsafe { (*wrapper).0() };
        0
    }

    impl super::DeviceChangeListenerApi for CoreAudioDefaultDeviceChangeListener {
        fn new(input: bool) -> anyhow::Result<Self> {
            let (tx, rx) = futures::channel::mpsc::unbounded();

            let callback = Box::new(PropertyListenerCallbackWrapper(Box::new(move || {
                tx.unbounded_send(()).ok();
            })));

            // Get the current default device ID
            let device_id = unsafe {
                // Listen for default device changes
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

                // Also listen for changes to the device configuration
                let device_id = if input {
                    let mut input_device: AudioObjectID = 0;
                    let mut prop_size = std::mem::size_of::<AudioObjectID>() as u32;
                    let result = coreaudio::sys::AudioObjectGetPropertyData(
                        kAudioObjectSystemObject,
                        &AudioObjectPropertyAddress {
                            mSelector: kAudioHardwarePropertyDefaultInputDevice,
                            mScope: kAudioObjectPropertyScopeGlobal,
                            mElement: kAudioObjectPropertyElementMaster,
                        },
                        0,
                        std::ptr::null(),
                        &mut prop_size as *mut _,
                        &mut input_device as *mut _ as *mut _,
                    );
                    if result != 0 {
                        log::warn!("Failed to get default input device ID");
                        0
                    } else {
                        input_device
                    }
                } else {
                    let mut output_device: AudioObjectID = 0;
                    let mut prop_size = std::mem::size_of::<AudioObjectID>() as u32;
                    let result = coreaudio::sys::AudioObjectGetPropertyData(
                        kAudioObjectSystemObject,
                        &AudioObjectPropertyAddress {
                            mSelector: kAudioHardwarePropertyDefaultOutputDevice,
                            mScope: kAudioObjectPropertyScopeGlobal,
                            mElement: kAudioObjectPropertyElementMaster,
                        },
                        0,
                        std::ptr::null(),
                        &mut prop_size as *mut _,
                        &mut output_device as *mut _ as *mut _,
                    );
                    if result != 0 {
                        log::warn!("Failed to get default output device ID");
                        0
                    } else {
                        output_device
                    }
                };

                if device_id != 0 {
                    // Listen for format changes on the device
                    coreaudio::Error::from_os_status(AudioObjectAddPropertyListener(
                        device_id,
                        &AudioObjectPropertyAddress {
                            mSelector: coreaudio::sys::kAudioDevicePropertyStreamFormat,
                            mScope: if input {
                                coreaudio::sys::kAudioObjectPropertyScopeInput
                            } else {
                                coreaudio::sys::kAudioObjectPropertyScopeOutput
                            },
                            mElement: kAudioObjectPropertyElementMaster,
                        },
                        Some(property_listener_handler_shim),
                        &*callback as *const _ as *mut _,
                    ))?;
                }

                device_id
            };

            Ok(Self {
                rx,
                callback,
                input,
                device_id,
            })
        }
    }

    impl Drop for CoreAudioDefaultDeviceChangeListener {
        fn drop(&mut self) {
            unsafe {
                // Remove the system-level property listener
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

                // Remove the device-specific property listener if we have a valid device ID
                if self.device_id != 0 {
                    AudioObjectRemovePropertyListener(
                        self.device_id,
                        &AudioObjectPropertyAddress {
                            mSelector: coreaudio::sys::kAudioDevicePropertyStreamFormat,
                            mScope: if self.input {
                                coreaudio::sys::kAudioObjectPropertyScopeInput
                            } else {
                                coreaudio::sys::kAudioObjectPropertyScopeOutput
                            },
                            mElement: kAudioObjectPropertyElementMaster,
                        },
                        Some(property_listener_handler_shim),
                        &*self.callback as *const _ as *mut _,
                    );
                }
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
