use super::track::RtcVideoTrack;
use futures::Stream;
use livekit::webrtc as real;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub mod video_stream {
    use super::*;

    pub mod native {
        use super::*;
        use real::video_frame::BoxVideoFrame;

        pub struct NativeVideoStream(RtcVideoTrack);

        impl NativeVideoStream {
            pub fn new(track: RtcVideoTrack) -> Self {
                Self(track)
            }
        }

        impl Stream for NativeVideoStream {
            type Item = BoxVideoFrame;

            fn poll_next(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<Self::Item>> {
                Poll::Ready(None)
            }
        }
    }
}

pub mod audio_source {
    use super::*;

    pub use real::audio_source::AudioSourceOptions;

    pub mod native {
        use super::*;

        pub struct NativeAudioSource {
            options: AudioSourceOptions,
            sample_rate: u32,
            num_channels: u32,
        }

        impl NativeAudioSource {
            pub fn new(options: AudioSourceOptions, sample_rate: u32, num_channels: u32) -> Self {
                Self {
                    options,
                    sample_rate,
                    num_channels,
                }
            }
        }
    }

    pub enum RtcAudioSource {
        Native(native::NativeAudioSource),
    }
}

pub use livekit::webrtc::video_frame;

pub mod video_source {
    use super::*;
    pub use real::video_source::VideoResolution;

    pub struct RTCVideoSource;

    pub mod native {
        use super::*;
        use real::video_frame::{VideoBuffer, VideoFrame};

        #[derive(Clone)]
        pub struct NativeVideoSource(VideoResolution);

        impl NativeVideoSource {
            pub fn new(resolution: super::VideoResolution) -> Self {
                Self(resolution)
            }

            pub fn capture_frame<T: AsRef<dyn VideoBuffer>>(&self, frame: &VideoFrame<T>) {}
        }
    }

    pub enum RtcVideoSource {
        Native(native::NativeVideoSource),
    }
}
