#![cfg_attr(target_os = "windows", allow(unused))]

mod remote_video_track_view;
#[cfg(any(test, feature = "test-support", target_os = "windows"))]
pub mod test;

use anyhow::{anyhow, Context as _, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait as _},
    StreamConfig,
};
use futures::{io, Stream, StreamExt as _};
use gpui::{AppContext, ScreenCaptureFrame, ScreenCaptureSource, ScreenCaptureStream, Task};
use parking_lot::Mutex;
use std::{borrow::Cow, future::Future, pin::Pin, sync::Arc};
use util::{debug_panic, ResultExt as _, TryFutureExt};
#[cfg(not(target_os = "windows"))]
use webrtc::{
    audio_frame::AudioFrame,
    audio_source::{native::NativeAudioSource, AudioSourceOptions, RtcAudioSource},
    audio_stream::native::NativeAudioStream,
    video_frame::{VideoBuffer, VideoFrame, VideoRotation},
    video_source::{native::NativeVideoSource, RtcVideoSource, VideoResolution},
    video_stream::native::NativeVideoStream,
};

#[cfg(all(not(any(test, feature = "test-support")), not(target_os = "windows")))]
pub use livekit::*;
#[cfg(any(test, feature = "test-support", target_os = "windows"))]
pub use test::*;

pub use remote_video_track_view::{RemoteVideoTrackView, RemoteVideoTrackViewEvent};

pub struct AudioStream {
    _tasks: [Task<Option<()>>; 2],
}

struct Dispatcher(Arc<dyn gpui::PlatformDispatcher>);

#[cfg(not(target_os = "windows"))]
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

#[cfg(not(target_os = "windows"))]
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

#[cfg(target_os = "windows")]
pub fn init(
    dispatcher: Arc<dyn gpui::PlatformDispatcher>,
    http_client: Arc<dyn http_client::HttpClient>,
) {
}

#[cfg(not(target_os = "windows"))]
pub fn init(
    dispatcher: Arc<dyn gpui::PlatformDispatcher>,
    http_client: Arc<dyn http_client::HttpClient>,
) {
    livekit::dispatcher::set_dispatcher(Dispatcher(dispatcher));
    livekit::dispatcher::set_http_client(HttpClientAdapter(http_client));
}

#[cfg(not(target_os = "windows"))]
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

#[cfg(not(target_os = "windows"))]
pub fn capture_local_audio_track(
    cx: &mut AppContext,
) -> Result<(track::LocalAudioTrack, AudioStream)> {
    let (frame_tx, mut frame_rx) = futures::channel::mpsc::unbounded();

    let sample_rate;
    let channels;
    let stream;
    if cfg!(any(test, feature = "test-support")) {
        sample_rate = 1;
        channels = 1;
        stream = None;
    } else {
        let device = cpal::default_host()
            .default_input_device()
            .ok_or_else(|| anyhow!("no audio input device available"))?;
        let config = device
            .default_input_config()
            .context("failed to get default input config")?;
        sample_rate = config.sample_rate().0;
        channels = config.channels() as u32;
        stream = Some(
            device
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
                .context("failed to build input stream")?,
        );
    }

    let source = NativeAudioSource::new(
        AudioSourceOptions {
            echo_cancellation: true,
            noise_suppression: true,
            auto_gain_control: false,
        },
        sample_rate,
        channels,
        // TODO livekit: Pull these out of a proto later
        100,
    );

    let stream_task = cx.foreground_executor().spawn(async move {
        if let Some(stream) = &stream {
            stream.play().log_err();
        }
        futures::future::pending().await
    });

    let transmit_task = cx.background_executor().spawn({
        let source = source.clone();
        async move {
            while let Some(frame) = frame_rx.next().await {
                source.capture_frame(&frame).await.ok();
            }
            Some(())
        }
    });

    let track =
        track::LocalAudioTrack::create_audio_track("microphone", RtcAudioSource::Native(source));

    Ok((
        track,
        AudioStream {
            _tasks: [stream_task, transmit_task],
        },
    ))
}

#[cfg(not(target_os = "windows"))]
pub fn play_remote_audio_track(
    track: &track::RemoteAudioTrack,
    cx: &mut AppContext,
) -> AudioStream {
    let buffer = Arc::new(Mutex::new(Vec::<i16>::new()));
    let (stream_config_tx, mut stream_config_rx) = futures::channel::mpsc::unbounded();
    // TODO livekit: Pull these out of a proto later
    let mut stream = NativeAudioStream::new(track.rtc_track(), 48000, 1);

    let receive_task = cx.background_executor().spawn({
        let buffer = buffer.clone();
        async move {
            let mut stream_config = None;
            while let Some(frame) = stream.next().await {
                let mut buffer = buffer.lock();
                let buffer_size = frame.samples_per_channel * frame.num_channels;
                debug_assert!(frame.data.len() == buffer_size as usize);

                let frame_config = StreamConfig {
                    channels: frame.num_channels as u16,
                    sample_rate: cpal::SampleRate(frame.sample_rate),
                    buffer_size: cpal::BufferSize::Fixed(buffer_size),
                };

                if stream_config.as_ref().map_or(true, |c| *c != frame_config) {
                    buffer.resize(buffer_size as usize, 0);
                    stream_config = Some(frame_config.clone());
                    stream_config_tx.unbounded_send(frame_config).ok();
                }

                if frame.data.len() == buffer.len() {
                    buffer.copy_from_slice(&frame.data);
                } else {
                    buffer.iter_mut().for_each(|x| *x = 0);
                }
            }
            Some(())
        }
    });

    let play_task = cx.foreground_executor().spawn(
        {
            let buffer = buffer.clone();
            async move {
                if cfg!(any(test, feature = "test-support")) {
                    return Err(anyhow!("can't play audio in tests"));
                }

                let device = cpal::default_host()
                    .default_output_device()
                    .ok_or_else(|| anyhow!("no audio output device available"))?;

                let mut _output_stream = None;
                while let Some(config) = stream_config_rx.next().await {
                    _output_stream = Some(device.build_output_stream(
                        &config,
                        {
                            let buffer = buffer.clone();
                            move |data, _info| {
                                let buffer = buffer.lock();
                                if data.len() == buffer.len() {
                                    data.copy_from_slice(&buffer);
                                } else {
                                    data.iter_mut().for_each(|x| *x = 0);
                                }
                            }
                        },
                        |error| log::error!("error playing audio track: {:?}", error),
                        None,
                    )?);
                }

                Ok(())
            }
        }
        .log_err(),
    );

    AudioStream {
        _tasks: [receive_task, play_task],
    }
}

#[cfg(target_os = "windows")]
pub fn play_remote_video_track(
    track: &track::RemoteVideoTrack,
) -> impl Stream<Item = ScreenCaptureFrame> {
    futures::stream::empty()
}

#[cfg(not(target_os = "windows"))]
pub fn play_remote_video_track(
    track: &track::RemoteVideoTrack,
) -> impl Stream<Item = ScreenCaptureFrame> {
    NativeVideoStream::new(track.rtc_track())
        .filter_map(|frame| async move { video_frame_buffer_from_webrtc(frame.buffer) })
}

#[cfg(target_os = "macos")]
fn video_frame_buffer_from_webrtc(buffer: Box<dyn VideoBuffer>) -> Option<ScreenCaptureFrame> {
    use core_foundation::base::TCFType as _;
    use media::core_video::CVImageBuffer;

    let buffer = buffer.as_native()?;
    let pixel_buffer = buffer.get_cv_pixel_buffer();
    if pixel_buffer.is_null() {
        return None;
    }

    unsafe {
        Some(ScreenCaptureFrame(CVImageBuffer::wrap_under_get_rule(
            pixel_buffer as _,
        )))
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn video_frame_buffer_from_webrtc(_buffer: Box<dyn VideoBuffer>) -> Option<ScreenCaptureFrame> {
    None
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

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn video_frame_buffer_to_webrtc(_frame: ScreenCaptureFrame) -> Option<impl AsRef<dyn VideoBuffer>> {
    None as Option<Box<dyn VideoBuffer>>
}
