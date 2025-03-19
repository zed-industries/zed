#![cfg_attr(all(target_os = "windows", target_env = "gnu"), allow(unused))]

mod remote_video_track_view;
#[cfg(any(
    test,
    feature = "test-support",
    all(target_os = "windows", target_env = "gnu")
))]
pub mod test;

use anyhow::{anyhow, Context as _, Result};
use core_foundation::base::TCFType;
use core_video::{
    pixel_buffer::kCVPixelFormatType_420YpCbCr8BiPlanarFullRange,
    pixel_buffer_io_surface::kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey,
    pixel_buffer_pool::{self, CVPixelBufferPool},
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait as _};
use futures::{Stream, StreamExt as _};
use gpui::{
    BackgroundExecutor, ScreenCaptureFrame, ScreenCaptureSource, ScreenCaptureStream, Task,
};

use parking_lot::Mutex;
use std::{borrow::Cow, collections::VecDeque, sync::Arc, thread};
use util::ResultExt as _;
#[cfg(not(all(target_os = "windows", target_env = "gnu")))]
use webrtc::{
    audio_frame::AudioFrame,
    audio_source::{native::NativeAudioSource, AudioSourceOptions, RtcAudioSource},
    audio_stream::native::NativeAudioStream,
    video_frame::{VideoBuffer, VideoFrame, VideoRotation},
    video_source::{native::NativeVideoSource, RtcVideoSource, VideoResolution},
    video_stream::native::NativeVideoStream,
};

#[cfg(all(
    not(any(test, feature = "test-support")),
    not(all(target_os = "windows", target_env = "gnu"))
))]
use livekit::track::RemoteAudioTrack;
#[cfg(all(
    not(any(test, feature = "test-support")),
    not(all(target_os = "windows", target_env = "gnu"))
))]
pub use livekit::*;
#[cfg(any(
    test,
    feature = "test-support",
    all(target_os = "windows", target_env = "gnu")
))]
use test::track::RemoteAudioTrack;
#[cfg(any(
    test,
    feature = "test-support",
    all(target_os = "windows", target_env = "gnu")
))]
pub use test::*;

pub use remote_video_track_view::{RemoteVideoTrackView, RemoteVideoTrackViewEvent};

pub enum AudioStream {
    Input {
        _thread_handle: std::sync::mpsc::Sender<()>,
        _transmit_task: Task<()>,
    },
    Output {
        _task: Task<()>,
    },
}

#[cfg(not(all(target_os = "windows", target_env = "gnu")))]
pub async fn capture_local_video_track(
    capture_source: &dyn ScreenCaptureSource,
    cx: &mut gpui::AsyncApp,
) -> Result<(track::LocalVideoTrack, Box<dyn ScreenCaptureStream>)> {
    let resolution = capture_source.resolution()?;
    dbg!(&resolution);
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
        track::LocalVideoTrack::create_video_track(
            "screen share",
            RtcVideoSource::Native(track_source),
        ),
        capture_stream,
    ))
}

#[cfg(not(all(target_os = "windows", target_env = "gnu")))]
pub fn capture_local_audio_track(
    background_executor: &BackgroundExecutor,
) -> Result<Task<(track::LocalAudioTrack, AudioStream)>> {
    use util::maybe;

    let (frame_tx, mut frame_rx) = futures::channel::mpsc::unbounded();
    let (thread_handle, thread_kill_rx) = std::sync::mpsc::channel::<()>();
    let sample_rate;
    let channels;

    if cfg!(any(test, feature = "test-support")) {
        sample_rate = 2;
        channels = 1;
    } else {
        let (device, config) = default_device(true)?;
        sample_rate = config.sample_rate().0;
        channels = config.channels() as u32;
        thread::spawn(move || {
            maybe!({
                if let Some(name) = device.name().ok() {
                    log::info!("Using microphone: {}", name)
                } else {
                    log::info!("Using microphone: <unknown>");
                }

                let stream = device
                    .build_input_stream_raw(
                        &config.config(),
                        cpal::SampleFormat::I16,
                        move |data, _: &_| {
                            frame_tx
                                .unbounded_send(AudioFrame {
                                    data: Cow::Owned(data.as_slice::<i16>().unwrap().to_vec()),
                                    sample_rate,
                                    num_channels: channels,
                                    samples_per_channel: data.len() as u32 / channels,
                                })
                                .ok();
                        },
                        |err| log::error!("error capturing audio track: {:?}", err),
                        None,
                    )
                    .context("failed to build input stream")?;

                stream.play()?;
                // Keep the thread alive and holding onto the `stream`
                thread_kill_rx.recv().ok();
                anyhow::Ok(Some(()))
            })
            .log_err();
        });
    }

    Ok(background_executor.spawn({
        let background_executor = background_executor.clone();
        async move {
            let source = NativeAudioSource::new(
                AudioSourceOptions {
                    echo_cancellation: true,
                    noise_suppression: true,
                    auto_gain_control: true,
                },
                sample_rate,
                channels,
                100,
            );
            let transmit_task = background_executor.spawn({
                let source = source.clone();
                async move {
                    while let Some(frame) = frame_rx.next().await {
                        source.capture_frame(&frame).await.log_err();
                    }
                }
            });

            let track = track::LocalAudioTrack::create_audio_track(
                "microphone",
                RtcAudioSource::Native(source),
            );

            (
                track,
                AudioStream::Input {
                    _thread_handle: thread_handle,
                    _transmit_task: transmit_task,
                },
            )
        }
    }))
}

#[cfg(not(all(target_os = "windows", target_env = "gnu")))]
pub fn play_remote_audio_track(
    track: &RemoteAudioTrack,
    background_executor: &BackgroundExecutor,
) -> Result<AudioStream> {
    let track = track.clone();
    // We track device changes in our output because Livekit has a resampler built in,
    // and it's easy to create a new native audio stream when the device changes.
    if cfg!(any(test, feature = "test-support")) {
        Ok(AudioStream::Output {
            _task: background_executor.spawn(async {}),
        })
    } else {
        let mut default_change_listener = DeviceChangeListener::new(false)?;
        let (output_device, output_config) = default_device(false)?;

        let _task = background_executor.spawn({
            let background_executor = background_executor.clone();
            async move {
                let (mut _receive_task, mut _thread) =
                    start_output_stream(output_config, output_device, &track, &background_executor);

                while let Some(_) = default_change_listener.next().await {
                    let Some((output_device, output_config)) = get_default_output().log_err()
                    else {
                        continue;
                    };

                    if let Ok(name) = output_device.name() {
                        log::info!("Using speaker: {}", name)
                    } else {
                        log::info!("Using speaker: <unknown>")
                    }

                    (_receive_task, _thread) = start_output_stream(
                        output_config,
                        output_device,
                        &track,
                        &background_executor,
                    );
                }

                futures::future::pending::<()>().await;
            }
        });

        Ok(AudioStream::Output { _task })
    }
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

#[cfg(not(all(target_os = "windows", target_env = "gnu")))]
fn get_default_output() -> anyhow::Result<(cpal::Device, cpal::SupportedStreamConfig)> {
    let host = cpal::default_host();
    let output_device = host
        .default_output_device()
        .context("failed to read default output device")?;
    let output_config = output_device.default_output_config()?;
    Ok((output_device, output_config))
}

#[cfg(not(all(target_os = "windows", target_env = "gnu")))]
fn start_output_stream(
    output_config: cpal::SupportedStreamConfig,
    output_device: cpal::Device,
    track: &track::RemoteAudioTrack,
    background_executor: &BackgroundExecutor,
) -> (Task<()>, std::sync::mpsc::Sender<()>) {
    let buffer = Arc::new(Mutex::new(VecDeque::<i16>::new()));
    let sample_rate = output_config.sample_rate();

    let mut stream = NativeAudioStream::new(
        track.rtc_track(),
        sample_rate.0 as i32,
        output_config.channels() as i32,
    );

    let receive_task = background_executor.spawn({
        let buffer = buffer.clone();
        async move {
            const MS_OF_BUFFER: u32 = 100;
            const MS_IN_SEC: u32 = 1000;
            while let Some(frame) = stream.next().await {
                let frame_size = frame.samples_per_channel * frame.num_channels;
                debug_assert!(frame.data.len() == frame_size as usize);

                let buffer_size =
                    ((frame.sample_rate * frame.num_channels) / MS_IN_SEC * MS_OF_BUFFER) as usize;

                let mut buffer = buffer.lock();
                let new_size = buffer.len() + frame.data.len();
                if new_size > buffer_size {
                    let overflow = new_size - buffer_size;
                    buffer.drain(0..overflow);
                }

                buffer.extend(frame.data.iter());
            }
        }
    });

    // The _output_stream needs to be on it's own thread because it's !Send
    // and we experienced a deadlock when it's created on the main thread.
    let (thread, end_on_drop_rx) = std::sync::mpsc::channel::<()>();
    thread::spawn(move || {
        if cfg!(any(test, feature = "test-support")) {
            // Can't play audio in tests
            return;
        }

        let output_stream = output_device.build_output_stream(
            &output_config.config(),
            {
                let buffer = buffer.clone();
                move |data, _info| {
                    let mut buffer = buffer.lock();
                    if buffer.len() < data.len() {
                        // Instead of partially filling a buffer, output silence. If a partial
                        // buffer was outputted then this could lead to a perpetual state of
                        // outputting partial buffers as it never gets filled enough for a full
                        // frame.
                        data.fill(0);
                    } else {
                        // SAFETY: We know that buffer has at least data.len() values in it.
                        // because we just checked
                        let mut drain = buffer.drain(..data.len());
                        data.fill_with(|| unsafe { drain.next().unwrap_unchecked() });
                    }
                }
            },
            |error| log::error!("error playing audio track: {:?}", error),
            None,
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

#[cfg(all(target_os = "windows", target_env = "gnu"))]
pub fn play_remote_video_track(
    track: &track::RemoteVideoTrack,
) -> impl Stream<Item = RemoteVideoFrame> {
    Ok(futures::stream::empty())
}

#[cfg(not(all(target_os = "windows", target_env = "gnu")))]
pub fn play_remote_video_track(
    track: &track::RemoteVideoTrack,
) -> impl Stream<Item = RemoteVideoFrame> {
    #[cfg(target_os = "macos")]
    {
        let mut pool = None;
        let most_recent_frame_size = (0, 0);
        NativeVideoStream::new(track.rtc_track()).filter_map(move |frame| {
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
        NativeVideoStream::new(track.rtc_track())
            .filter_map(|frame| async move { video_frame_buffer_from_webrtc(frame.buffer) })
    }
}

fn create_buffer_pool(width: u32, height: u32) -> Result<CVPixelBufferPool> {
    use core_foundation::{base::TCFType, number::CFNumber, string::CFString};
    use core_video::pixel_buffer;

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

    Ok(
        pixel_buffer_pool::CVPixelBufferPool::new(None, Some(&buffer_attributes)).map_err(
            |cv_return| {
                anyhow!(
                    "failed to create pixel buffer pool: CVReturn({})",
                    cv_return
                )
            },
        )?,
    )
}

#[cfg(target_os = "macos")]
pub type RemoteVideoFrame = core_video::pixel_buffer::CVPixelBuffer;

#[cfg(target_os = "macos")]
fn video_frame_buffer_from_webrtc(
    pool: core_video::pixel_buffer_pool::CVPixelBufferPool,
    buffer: Box<dyn VideoBuffer>,
) -> Option<RemoteVideoFrame> {
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
        let dst_y_buffer = std::slice::from_raw_parts_mut(dst_y as *mut u8, dst_y_len as usize);
        let dst_uv_buffer = std::slice::from_raw_parts_mut(dst_uv as *mut u8, dst_uv_len as usize);

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

#[cfg(not(any(target_os = "macos", all(target_os = "windows", target_env = "gnu"))))]
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
    use core_video::buffer::__CVBuffer;

    let pixel_buffer = frame.0.as_concrete_TypeRef() as *mut __CVBuffer;
    std::mem::forget(frame.0);
    unsafe {
        Some(webrtc::video_frame::native::NativeBuffer::from_cv_pixel_buffer(pixel_buffer as _))
    }
}

#[cfg(not(any(target_os = "macos", all(target_os = "windows", target_env = "gnu"))))]
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

    use crate::DeviceChangeListenerApi;

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

    impl DeviceChangeListenerApi for CoreAudioDefaultDeviceChangeListener {
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

    use crate::DeviceChangeListenerApi;

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
