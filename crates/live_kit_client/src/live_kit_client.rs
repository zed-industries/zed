use anyhow::Result;
use collections::VecDeque;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait as _};
use futures::{Stream, StreamExt as _};
use gpui::{
    BackgroundExecutor, ScreenCaptureFrame, ScreenCaptureSource, ScreenCaptureStream, Task,
};
use parking_lot::Mutex;
use std::{borrow::Cow, sync::Arc};
use webrtc::{
    audio_frame::AudioFrame,
    audio_source::{native::NativeAudioSource, AudioSourceOptions, RtcAudioSource},
    audio_stream::native::NativeAudioStream,
    video_frame::{native::NativeBuffer, VideoFrame, VideoRotation},
    video_source::{native::NativeVideoSource, RtcVideoSource, VideoResolution},
    video_stream::native::NativeVideoStream,
};

#[cfg(any(test, feature = "test-support"))]
pub mod test;

#[cfg(not(any(test, feature = "test-support")))]
pub use livekit::*;
#[cfg(any(test, feature = "test-support"))]
pub use test::*;

pub fn init(dispatcher: Arc<dyn gpui::PlatformDispatcher>) {
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

    livekit::dispatcher::set_dispatcher(Dispatcher(dispatcher));
}

pub async fn capture_local_video_track(
    capture_source: &dyn ScreenCaptureSource,
) -> Result<(track::LocalVideoTrack, Box<dyn ScreenCaptureStream>)> {
    let track_source = NativeVideoSource::new(VideoResolution {
        width: 1,
        height: 1,
    });

    let capture_stream = capture_source
        .stream({
            let track_source = track_source.clone();
            Box::new(move |frame| {
                let buffer: NativeBuffer = todo!();
                track_source.capture_frame(&VideoFrame {
                    rotation: VideoRotation::VideoRotation0,
                    timestamp_us: 0,
                    buffer,
                });
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

pub async fn capture_local_audio_track(
    cx: &BackgroundExecutor,
) -> Result<(track::LocalAudioTrack, AudioStream)> {
    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .expect("No input device available");
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    let sample_rate = config.sample_rate();
    let channels = config.channels() as u32;
    let source = NativeAudioSource::new(AudioSourceOptions::default(), sample_rate.0, channels);

    let (frame_tx, mut frame_rx) = futures::channel::mpsc::unbounded();

    let _task = cx.spawn({
        let source = source.clone();
        async move {
            while let Some(frame) = frame_rx.next().await {
                source.capture_frame(&frame).await.ok();
            }
        }
    });

    let stream = device
        .build_input_stream_raw(
            &config.config(),
            cpal::SampleFormat::I16,
            move |data, _: &_| {
                frame_tx
                    .unbounded_send(AudioFrame {
                        data: Cow::Owned(data.as_slice::<i16>().unwrap().to_vec()),
                        sample_rate: sample_rate.0,
                        num_channels: channels,
                        samples_per_channel: data.len() as u32 / channels,
                    })
                    .ok();
            },
            move |err| eprintln!("Error: {:?}", err),
            None,
        )
        .expect("Failed to build input stream");

    stream.play().expect("Failed to play stream");

    let track =
        track::LocalAudioTrack::create_audio_track("microphone", RtcAudioSource::Native(source));

    Ok((
        track,
        AudioStream {
            _stream: stream,
            _task,
        },
    ))
}

pub struct AudioStream {
    _stream: cpal::Stream,
    _task: Task<()>,
}

pub fn play_remote_audio_track(
    track: &track::RemoteAudioTrack,
    executor: &BackgroundExecutor,
) -> AudioStream {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available");
    let config = device
        .default_output_config()
        .expect("Failed to get default input config");

    let ring_buffer = Arc::new(Mutex::new(VecDeque::new()));

    let _stream = device
        .build_output_stream::<i16, _, _>(
            &config.config(),
            {
                let ring_buffer = ring_buffer.clone();
                move |data, _info| {
                    let mut buffer = ring_buffer.lock();
                    let (a, b) = buffer.as_slices();
                    let buffer_len = buffer.len();

                    if a.len() > data.len() {
                        data.copy_from_slice(&a[..data.len()]);
                        buffer.drain(..data.len());
                        return;
                    }

                    data[..a.len()].copy_from_slice(a);

                    let remainder = (data.len() - a.len()).min(b.len());
                    data[a.len()..a.len() + remainder].copy_from_slice(&b[..remainder]);

                    if buffer_len < data.len() {
                        eprintln!(
                            "not enough data. have {}, need {}",
                            buffer.len(),
                            data.len()
                        );
                    }

                    buffer.drain(0..data.len().min(buffer_len));
                }
            },
            move |err| eprintln!("Error: {:?}", err),
            None,
        )
        .unwrap();

    let mut stream = NativeAudioStream::new(track.rtc_track());

    let _task = executor.spawn({
        let ring_buffer = ring_buffer.clone();
        async move {
            while let Some(frame) = stream.next().await {
                ring_buffer.lock().extend(frame.data.iter());
            }
        }
    });

    AudioStream { _stream, _task }
}

pub fn play_remote_video_track(
    track: &track::RemoteVideoTrack,
) -> impl Stream<Item = ScreenCaptureFrame> {
    NativeVideoStream::new(track.rtc_track()).filter_map(|video_frame| async move {
        dbg!(
            video_frame.buffer.width(),
            video_frame.buffer.height(),
            video_frame.buffer.buffer_type()
        );
        let native_buffer = video_frame.buffer.as_native()?;
        None
    })
}
