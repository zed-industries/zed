#![cfg_attr(target_os = "windows", allow(unused))]

mod remote_video_track_view;
#[cfg(any(test, feature = "test-support", target_os = "windows"))]
pub mod test;

use anyhow::{anyhow, Context as _, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait as _};
use futures::{io, Stream, StreamExt as _};
use gpui::{
    BackgroundExecutor, ScreenCaptureFrame, ScreenCaptureSource, ScreenCaptureStream, Task,
};
use parking_lot::Mutex;
use std::{borrow::Cow, future::Future, pin::Pin, sync::Arc};
use util::{debug_panic, ResultExt as _};
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

pub enum AudioStream {
    Input {
        _tasks: [Task<Result<Option<()>>>; 2],
    },
    Output {
        _end_on_drop: std::sync::mpsc::Sender<()>,
        _receive_task: Task<()>,
    },
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
    background_executor: &BackgroundExecutor,
) -> Task<Result<(track::LocalAudioTrack, AudioStream)>> {
    let (frame_tx, mut frame_rx) = futures::channel::mpsc::unbounded();
    let (track_data_tx, mut track_data_rx) = futures::channel::mpsc::unbounded();

    let stream_task = background_executor.spawn(async move {
        let sample_rate;
        let channels;
        let stream;
        if cfg!(any(test, feature = "test-support")) {
            sample_rate = 1;
            channels = 1;
            stream = None;
            track_data_tx
                .unbounded_send((sample_rate, channels))
                .expect("failed to send track data");
        } else {
            let device = cpal::default_host()
                .default_input_device()
                .ok_or_else(|| anyhow!("no audio input device available"))?;
            let config = device
                .default_input_config()
                .context("failed to get default input config")?;
            sample_rate = config.sample_rate().0;
            channels = config.channels() as u32;
            track_data_tx
                .unbounded_send((sample_rate, channels))
                .log_err();
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
        if let Some(stream) = &stream {
            stream.play().log_err();
            // We need to keep the thread alive and task not dropped, so the `stream` is not dropped.
            // `stream` is `!Send` so we cannot move it away anywhere else.
            loop {
                std::thread::park();
                // Suppress the unreachable code warning
                if false {
                    break;
                }
            }
        }

        Ok(Some(()))
    });

    let task_background_executor = background_executor.clone();
    background_executor.spawn(async move {
        let (sample_rate, channels) = track_data_rx
            .next()
            .await
            .context("receiving sample rate and channels data")?;
        let source = NativeAudioSource::new(
            AudioSourceOptions {
                echo_cancellation: true,
                noise_suppression: true,
                auto_gain_control: false,
            },
            sample_rate,
            channels,
            100,
        );
        let transmit_task = task_background_executor.spawn({
            let source = source.clone();
            async move {
                while let Some(frame) = frame_rx.next().await {
                    source.capture_frame(&frame).await.log_err();
                }
                Ok(Some(()))
            }
        });

        let track = track::LocalAudioTrack::create_audio_track(
            "microphone",
            RtcAudioSource::Native(source),
        );

        anyhow::Ok((
            track,
            AudioStream::Input {
                _tasks: [stream_task, transmit_task],
            },
        ))
    })
}

#[cfg(not(target_os = "windows"))]
pub fn play_remote_audio_track(
    track: &track::RemoteAudioTrack,
    background_executor: &BackgroundExecutor,
) -> Result<AudioStream> {
    // TODO(mgsloan): use a concurrent queue that references the Cow slices the source gives us.
    //
    // TODO(mgsloan): put this on the channel? rationale is to not mix up samples that came from a
    // different configuration.

    use std::thread;
    let buffer_mutex = Arc::new(Mutex::new(Vec::<i16>::new()));

    let device = cpal::default_host()
        .default_output_device()
        .context("no audio output device available")?;
    let default_config = device
        .default_output_config()
        .context("no default configuration available for default output device")?;

    let mut stream = NativeAudioStream::new(
        track.rtc_track(),
        default_config.sample_rate().0 as i32,
        default_config.channels() as i32,
    );

    // let mut stream = NativeAudioStream::new(track.rtc_track(), 48000, 1);

    let _receive_task = background_executor.spawn({
        let buffer_mutex = buffer_mutex.clone();
        async move {
            while let Some(frame) = stream.next().await {
                let buffer_size = frame.samples_per_channel * frame.num_channels;
                debug_assert!(frame.data.len() == buffer_size as usize);

                let mut buffer = buffer_mutex.lock();
                // TODO(mgsloan): max_size multiplier was arbitrarily chosen.
                let max_size = (buffer_size * 5) as usize;
                let new_size = buffer.len() + frame.data.len();
                if new_size > max_size {
                    let drain_ix = new_size - max_size;
                    if drain_ix > buffer.len() {
                        buffer.clear();
                    } else {
                        buffer.drain(..new_size - max_size);
                    }
                }
                buffer.extend_from_slice(&frame.data);
            }
        }
    });

    let (_end_on_drop, end_on_drop_rx) = std::sync::mpsc::channel();
    thread::spawn(move || {
        if cfg!(any(test, feature = "test-support")) {
            // Can't play audio in tests
            return;
        }

        let mut _output_stream = device
            .build_output_stream(
                &default_config.config(),
                {
                    let buffer_mutex = buffer_mutex.clone();
                    move |data, _info| {
                        let mut buffer = buffer_mutex.lock();
                        while data.len() > buffer.len() {
                            drop(buffer);
                            std::hint::spin_loop();
                            buffer = buffer_mutex.lock();
                        }
                        data.copy_from_slice(&buffer[..data.len()]);
                        buffer.drain(..data.len());
                    }
                },
                |error| log::error!("error playing audio track: {:?}", error),
                None,
            )
            .ok();

        // Block forever to keep the output stream alive
        end_on_drop_rx.recv().ok();
    });

    Ok(AudioStream::Output {
        _end_on_drop,
        _receive_task,
    })
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
