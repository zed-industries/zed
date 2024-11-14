use super::track::{RtcAudioTrack, RtcVideoTrack};
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

        pub struct NativeVideoStream {
            pub track: RtcVideoTrack,
        }

        impl NativeVideoStream {
            pub fn new(track: RtcVideoTrack) -> Self {
                Self { track }
            }
        }

        impl Stream for NativeVideoStream {
            type Item = BoxVideoFrame;

            fn poll_next(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<Self::Item>> {
                Poll::Pending
            }
        }
    }
}

pub mod audio_stream {
    use super::*;

    pub mod native {
        use super::*;
        use real::audio_frame::AudioFrame;

        pub struct NativeAudioStream {
            pub track: RtcAudioTrack,
        }

        impl NativeAudioStream {
            pub fn new(track: RtcAudioTrack, _sample_rate: i32, _num_channels: i32) -> Self {
                Self { track }
            }
        }

        impl Stream for NativeAudioStream {
            type Item = AudioFrame<'static>;

            fn poll_next(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<Self::Item>> {
                Poll::Pending
            }
        }
    }
}

pub mod audio_source {
    use super::*;

    pub use real::audio_source::AudioSourceOptions;

    pub mod native {
        use std::sync::Arc;

        use super::*;
        use real::{audio_frame::AudioFrame, RtcError};

        #[derive(Clone)]
        pub struct NativeAudioSource {
            pub options: Arc<AudioSourceOptions>,
            pub sample_rate: u32,
            pub num_channels: u32,
        }

        impl NativeAudioSource {
            pub fn new(
                options: AudioSourceOptions,
                sample_rate: u32,
                num_channels: u32,
                _queue_size_ms: u32,
            ) -> Self {
                Self {
                    options: Arc::new(options),
                    sample_rate,
                    num_channels,
                }
            }

            pub async fn capture_frame(&self, _frame: &AudioFrame<'_>) -> Result<(), RtcError> {
                Ok(())
            }
        }
    }

    pub enum RtcAudioSource {
        Native(native::NativeAudioSource),
    }
}

pub use livekit::webrtc::audio_frame;
pub use livekit::webrtc::video_frame;

pub mod video_source {
    use super::*;
    pub use real::video_source::VideoResolution;

    pub struct RTCVideoSource;

    pub mod native {
        use super::*;
        use real::video_frame::{VideoBuffer, VideoFrame};

        #[derive(Clone)]
        pub struct NativeVideoSource {
            pub resolution: VideoResolution,
        }

        impl NativeVideoSource {
            pub fn new(resolution: super::VideoResolution) -> Self {
                Self { resolution }
            }

            pub fn capture_frame<T: AsRef<dyn VideoBuffer>>(&self, _frame: &VideoFrame<T>) {}
        }
    }

    pub enum RtcVideoSource {
        Native(native::NativeVideoSource),
    }
}
