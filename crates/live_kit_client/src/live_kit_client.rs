use anyhow::Result;
use futures::{Stream, StreamExt as _};
use gpui::{ScreenCaptureFrame, ScreenCaptureSource, ScreenCaptureStream};
use webrtc::{
    audio_source::{native::NativeAudioSource, AudioSourceOptions, RtcAudioSource},
    video_frame::{native::NativeBuffer, VideoFrame, VideoRotation},
    video_source::{native::NativeVideoSource, RtcVideoSource, VideoResolution},
    video_stream::native::NativeVideoStream,
};

#[cfg(not(any(test, feature = "test-support")))]
pub use livekit::*;

#[cfg(any(test, feature = "test-support"))]
pub mod test;
#[cfg(any(test, feature = "test-support"))]
pub use test::*;

pub async fn create_video_track_from_screen_capture_source(
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

pub async fn create_audio_track_from_microphone() -> Result<track::LocalAudioTrack> {
    let source = NativeAudioSource::new(AudioSourceOptions::default(), 100, 1);
    let track =
        track::LocalAudioTrack::create_audio_track("microphone", RtcAudioSource::Native(source));
    Ok(track)
}

pub fn create_screen_capture_frame_stream_from_video_track(
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
