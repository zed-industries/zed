mod remote_video_track_view;
#[cfg(any(test, feature = "test-support"))]
pub mod test;

use anyhow::{anyhow, Context as _, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait as _};
use futures::{io, Stream, StreamExt as _};
use gpui::{
    BackgroundExecutor, ScreenCaptureFrame, ScreenCaptureSource, ScreenCaptureStream, Task,
};
use parking_lot::Mutex;
use std::{borrow::Cow, collections::VecDeque, future::Future, pin::Pin, sync::Arc, thread};
use util::{debug_panic, ResultExt as _};
use webrtc::{
    audio_frame::AudioFrame,
    audio_source::{native::NativeAudioSource, AudioSourceOptions, RtcAudioSource},
    audio_stream::native::NativeAudioStream,
    video_frame::{VideoBuffer, VideoFrame, VideoRotation},
    video_source::{native::NativeVideoSource, RtcVideoSource, VideoResolution},
    video_stream::native::NativeVideoStream,
};

#[cfg(not(any(test, feature = "test-support")))]
use livekit::track::RemoteAudioTrack;
#[cfg(not(any(test, feature = "test-support")))]
pub use livekit::*;
#[cfg(any(test, feature = "test-support"))]
use test::track::RemoteAudioTrack;
#[cfg(any(test, feature = "test-support"))]
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

struct Dispatcher(Arc<dyn gpui::PlatformDispatcher>);

impl livekit::dispatcher::Dispatcher for Dispatcher {
    fn dispatch(&self, runnable: livekit::dispatcher::Runnable) {
        self.0.dispatch(runnable, None);
    }

    fn dispatch_after(
        &self,
        duration: std::time::Duration,
        runnable: livekit::dispatcher::Runnable,
    ) {
        self.0.dispatch_after(duration, runnable);
    }
}

struct HttpClientAdapter(Arc<dyn http_client::HttpClient>);

fn http_2_status(status: http_client::http::StatusCode) -> http_2::StatusCode {
    http_2::StatusCode::from_u16(status.as_u16())
        .expect("valid status code to status code conversion")
}

impl livekit::dispatcher::HttpClient for HttpClientAdapter {
    fn get(
        &self,
        url: &str,
    ) -> Pin<Box<dyn Future<Output = io::Result<livekit::dispatcher::Response>> + Send>> {
        let http_client = self.0.clone();
        let url = url.to_string();
        Box::pin(async move {
            let response = http_client
                .get(&url, http_client::AsyncBody::empty(), false)
                .await
                .map_err(io::Error::other)?;
            Ok(livekit::dispatcher::Response {
                status: http_2_status(response.status()),
                body: Box::pin(response.into_body()),
            })
        })
    }

    fn send_async(
        &self,
        request: http_2::Request<Vec<u8>>,
    ) -> Pin<Box<dyn Future<Output = io::Result<livekit::dispatcher::Response>> + Send>> {
        let http_client = self.0.clone();
        let mut builder = http_client::http::Request::builder()
            .method(request.method().as_str())
            .uri(request.uri().to_string());

        for (key, value) in request.headers().iter() {
            builder = builder.header(key.as_str(), value.as_bytes());
        }

        if !request.extensions().is_empty() {
            debug_panic!(
                "Livekit sent an HTTP request with a protocol extension that Zed doesn't support!"
            );
        }

        let request = builder
            .body(http_client::AsyncBody::from_bytes(
                request.into_body().into(),
            ))
            .unwrap();

        Box::pin(async move {
            let response = http_client.send(request).await.map_err(io::Error::other)?;
            Ok(livekit::dispatcher::Response {
                status: http_2_status(response.status()),
                body: Box::pin(response.into_body()),
            })
        })
    }
}

pub fn init(
    dispatcher: Arc<dyn gpui::PlatformDispatcher>,
    http_client: Arc<dyn http_client::HttpClient>,
) {
    livekit::dispatcher::set_dispatcher(Dispatcher(dispatcher));
    livekit::dispatcher::set_http_client(HttpClientAdapter(http_client));
}

pub async fn capture_local_video_track(
    capture_source: &dyn ScreenCaptureSource,
) -> Result<(track::LocalVideoTrack, Box<dyn ScreenCaptureStream>)> {
    let resolution = capture_source.resolution()?;
    let track_source = NativeVideoSource::new(VideoResolution {
        width: resolution.width.0 as u32,
        height: resolution.height.0 as u32,
    });

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

fn get_default_output() -> anyhow::Result<(cpal::Device, cpal::SupportedStreamConfig)> {
    let host = cpal::default_host();
    let output_device = host
        .default_output_device()
        .context("failed to read default output device")?;
    let output_config = output_device.default_output_config()?;
    Ok((output_device, output_config))
}

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

pub fn play_remote_video_track(
    track: &track::RemoteVideoTrack,
) -> impl Stream<Item = RemoteVideoFrame> {
    NativeVideoStream::new(track.rtc_track())
        .filter_map(|frame| async move { video_frame_buffer_from_webrtc(frame.buffer) })
}

#[cfg(target_os = "macos")]
pub type RemoteVideoFrame = media::core_video::CVImageBuffer;

#[cfg(target_os = "macos")]
fn video_frame_buffer_from_webrtc(buffer: Box<dyn VideoBuffer>) -> Option<RemoteVideoFrame> {
    use core_foundation::base::TCFType as _;
    use media::core_video::CVImageBuffer;

    let buffer = buffer.as_native()?;
    let pixel_buffer = buffer.get_cv_pixel_buffer();
    if pixel_buffer.is_null() {
        return None;
    }

    unsafe { Some(CVImageBuffer::wrap_under_get_rule(pixel_buffer as _)) }
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
    use core_foundation::base::TCFType as _;

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
