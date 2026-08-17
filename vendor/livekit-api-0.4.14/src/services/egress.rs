// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use livekit_protocol as proto;

use super::{ServiceBase, ServiceResult, LIVEKIT_PACKAGE};
use crate::{access_token::VideoGrants, get_env_keys, services::twirp_client::TwirpClient};

#[derive(Clone, Copy, Debug, Default)]
pub enum AudioMixing {
    /// All users are mixed together.
    #[default]
    DefaultMixing,
    /// Agent audio in the left channel, all other audio in the right channel.
    DualChannelAgent,
    /// Each new audio track alternates between left and right channels.
    DualChannelAlternate,
}

impl From<AudioMixing> for proto::AudioMixing {
    fn from(value: AudioMixing) -> Self {
        match value {
            AudioMixing::DefaultMixing => proto::AudioMixing::DefaultMixing,
            AudioMixing::DualChannelAgent => proto::AudioMixing::DualChannelAgent,
            AudioMixing::DualChannelAlternate => proto::AudioMixing::DualChannelAlternate,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct RoomCompositeOptions {
    pub layout: String,
    pub encoding: encoding::EncodingOptions,
    pub audio_only: bool,
    pub video_only: bool,
    pub custom_base_url: String,
    /// Only applies when audio_only is true (default: DefaultMixing)
    pub audio_mixing: AudioMixing,
}

#[derive(Default, Clone, Debug)]
pub struct WebOptions {
    pub encoding: encoding::EncodingOptions,
    pub audio_only: bool,
    pub video_only: bool,
    pub await_start_signal: bool,
}

#[derive(Default, Clone, Debug)]
pub struct ParticipantEgressOptions {
    pub screenshare: bool,
    pub encoding: encoding::EncodingOptions,
}

#[derive(Default, Clone, Debug)]
pub struct TrackCompositeOptions {
    pub encoding: encoding::EncodingOptions,
    pub audio_track_id: String,
    pub video_track_id: String,
}

#[derive(Debug, Clone)]
pub enum EgressOutput {
    File(proto::EncodedFileOutput),
    Stream(proto::StreamOutput),
    Segments(proto::SegmentedFileOutput),
    Image(proto::ImageOutput),
}

#[derive(Debug, Clone)]
pub enum TrackEgressOutput {
    File(Box<proto::DirectFileOutput>),
    WebSocket(String),
}

#[derive(Debug, Clone)]
pub enum EgressListFilter {
    All,
    Egress(String),
    Room(String),
}

#[derive(Debug, Clone)]
pub struct EgressListOptions {
    pub filter: EgressListFilter,
    pub active: bool,
}

const SVC: &str = "Egress";

#[derive(Debug)]
pub struct EgressClient {
    base: ServiceBase,
    client: TwirpClient,
}

impl EgressClient {
    pub fn with_api_key(host: &str, api_key: &str, api_secret: &str) -> Self {
        Self {
            base: ServiceBase::with_api_key(api_key, api_secret),
            client: TwirpClient::new(host, LIVEKIT_PACKAGE, None),
        }
    }

    pub fn new(host: &str) -> ServiceResult<Self> {
        let (api_key, api_secret) = get_env_keys()?;
        Ok(Self::with_api_key(host, &api_key, &api_secret))
    }

    pub async fn start_room_composite_egress(
        &self,
        room: &str,
        outputs: Vec<EgressOutput>,
        options: RoomCompositeOptions,
    ) -> ServiceResult<proto::EgressInfo> {
        let (file_outputs, stream_outputs, segment_outputs, image_outputs) = get_outputs(outputs);
        self.client
            .request(
                SVC,
                "StartRoomCompositeEgress",
                proto::RoomCompositeEgressRequest {
                    room_name: room.to_string(),
                    layout: options.layout,
                    audio_only: options.audio_only,
                    audio_mixing: Into::<proto::AudioMixing>::into(options.audio_mixing) as i32,
                    video_only: options.video_only,
                    options: Some(proto::room_composite_egress_request::Options::Advanced(
                        options.encoding.into(),
                    )),
                    custom_base_url: options.custom_base_url,
                    file_outputs,
                    stream_outputs,
                    segment_outputs,
                    image_outputs,
                    output: None, // Deprecated
                    ..Default::default()
                },
                self.base
                    .auth_header(VideoGrants { room_record: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn start_web_egress(
        &self,
        url: &str,
        outputs: Vec<EgressOutput>,
        options: WebOptions,
    ) -> ServiceResult<proto::EgressInfo> {
        let (file_outputs, stream_outputs, segment_outputs, image_outputs) = get_outputs(outputs);
        self.client
            .request(
                SVC,
                "StartWebEgress",
                proto::WebEgressRequest {
                    url: url.to_string(),
                    options: Some(proto::web_egress_request::Options::Advanced(
                        options.encoding.into(),
                    )),
                    audio_only: options.audio_only,
                    video_only: options.video_only,
                    file_outputs,
                    stream_outputs,
                    segment_outputs,
                    image_outputs,
                    output: None, // Deprecated
                    await_start_signal: options.await_start_signal,
                    ..Default::default()
                },
                self.base
                    .auth_header(VideoGrants { room_record: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn start_participant_egress(
        &self,
        room: &str,
        participant_identity: &str,
        outputs: Vec<EgressOutput>,
        options: ParticipantEgressOptions,
    ) -> ServiceResult<proto::EgressInfo> {
        let (file_outputs, stream_outputs, segment_outputs, image_outputs) = get_outputs(outputs);
        self.client
            .request(
                SVC,
                "StartParticipantEgress",
                proto::ParticipantEgressRequest {
                    room_name: room.to_string(),
                    identity: participant_identity.to_string(),
                    options: Some(proto::participant_egress_request::Options::Advanced(
                        options.encoding.into(),
                    )),
                    screen_share: options.screenshare,
                    file_outputs,
                    stream_outputs,
                    segment_outputs,
                    image_outputs,
                    ..Default::default()
                },
                self.base
                    .auth_header(VideoGrants { room_record: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn start_track_composite_egress(
        &self,
        room: &str,
        outputs: Vec<EgressOutput>,
        options: TrackCompositeOptions,
    ) -> ServiceResult<proto::EgressInfo> {
        let (file_outputs, stream_outputs, segment_outputs, image_outputs) = get_outputs(outputs);
        self.client
            .request(
                SVC,
                "StartTrackCompositeEgress",
                proto::TrackCompositeEgressRequest {
                    room_name: room.to_string(),
                    options: Some(proto::track_composite_egress_request::Options::Advanced(
                        options.encoding.into(),
                    )),
                    audio_track_id: options.audio_track_id,
                    video_track_id: options.video_track_id,
                    file_outputs,
                    stream_outputs,
                    segment_outputs,
                    image_outputs,
                    output: None, // Deprecated
                    ..Default::default()
                },
                self.base
                    .auth_header(VideoGrants { room_record: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn start_track_egress(
        &self,
        room: &str,
        output: TrackEgressOutput,
        track_id: &str,
    ) -> ServiceResult<proto::EgressInfo> {
        self.client
            .request(
                SVC,
                "StartTrackEgress",
                proto::TrackEgressRequest {
                    room_name: room.to_string(),
                    output: match output {
                        TrackEgressOutput::File(f) => {
                            Some(proto::track_egress_request::Output::File(*f))
                        }
                        TrackEgressOutput::WebSocket(url) => {
                            Some(proto::track_egress_request::Output::WebsocketUrl(url))
                        }
                    },
                    track_id: track_id.to_string(),
                    ..Default::default()
                },
                self.base
                    .auth_header(VideoGrants { room_record: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn update_layout(
        &self,
        egress_id: &str,
        layout: &str,
    ) -> ServiceResult<proto::EgressInfo> {
        self.client
            .request(
                SVC,
                "UpdateLayout",
                proto::UpdateLayoutRequest {
                    egress_id: egress_id.to_owned(),
                    layout: layout.to_owned(),
                },
                self.base
                    .auth_header(VideoGrants { room_record: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn update_stream(
        &self,
        egress_id: &str,
        add_output_urls: Vec<String>,
        remove_output_urls: Vec<String>,
    ) -> ServiceResult<proto::EgressInfo> {
        self.client
            .request(
                SVC,
                "UpdateStream",
                proto::UpdateStreamRequest {
                    egress_id: egress_id.to_owned(),
                    add_output_urls,
                    remove_output_urls,
                },
                self.base
                    .auth_header(VideoGrants { room_record: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn list_egress(
        &self,
        options: EgressListOptions,
    ) -> ServiceResult<Vec<proto::EgressInfo>> {
        let mut room_name = String::default();
        let mut egress_id = String::default();

        match options.filter {
            EgressListFilter::Room(room) => room_name = room,
            EgressListFilter::Egress(egress) => egress_id = egress,
            _ => {}
        }

        let resp: proto::ListEgressResponse = self
            .client
            .request(
                SVC,
                "ListEgress",
                proto::ListEgressRequest { room_name, egress_id, active: options.active },
                self.base
                    .auth_header(VideoGrants { room_record: true, ..Default::default() }, None)?,
            )
            .await?;

        Ok(resp.items)
    }

    pub async fn stop_egress(&self, egress_id: &str) -> ServiceResult<proto::EgressInfo> {
        self.client
            .request(
                SVC,
                "StopEgress",
                proto::StopEgressRequest { egress_id: egress_id.to_owned() },
                self.base
                    .auth_header(VideoGrants { room_record: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }
}

fn get_outputs(
    outputs: Vec<EgressOutput>,
) -> (
    Vec<proto::EncodedFileOutput>,
    Vec<proto::StreamOutput>,
    Vec<proto::SegmentedFileOutput>,
    Vec<proto::ImageOutput>,
) {
    let mut file_outputs = Vec::new();
    let mut stream_outputs = Vec::new();
    let mut segment_outputs = Vec::new();
    let mut image_outputs = Vec::new();

    for output in outputs {
        match output {
            EgressOutput::File(f) => file_outputs.push(f),
            EgressOutput::Stream(s) => stream_outputs.push(s),
            EgressOutput::Segments(s) => segment_outputs.push(s),
            EgressOutput::Image(i) => image_outputs.push(i),
        }
    }

    (file_outputs, stream_outputs, segment_outputs, image_outputs)
}

pub mod encoding {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct EncodingOptions {
        pub width: i32,
        pub height: i32,
        pub depth: i32,
        pub framerate: i32,
        pub audio_codec: proto::AudioCodec,
        pub audio_bitrate: i32,
        pub audio_frequency: i32,
        pub video_codec: proto::VideoCodec,
        pub video_bitrate: i32,
        pub keyframe_interval: f64,
        pub audio_quality: i32,
        pub video_quality: i32,
    }

    impl From<EncodingOptions> for proto::EncodingOptions {
        fn from(opts: EncodingOptions) -> Self {
            Self {
                width: opts.width,
                height: opts.height,
                depth: opts.depth,
                framerate: opts.framerate,
                audio_codec: opts.audio_codec as i32,
                audio_bitrate: opts.audio_bitrate,
                audio_frequency: opts.audio_frequency,
                video_codec: opts.video_codec as i32,
                video_bitrate: opts.video_bitrate,
                key_frame_interval: opts.keyframe_interval,
                audio_quality: opts.audio_quality,
                video_quality: opts.video_quality,
            }
        }
    }

    impl EncodingOptions {
        const fn new() -> Self {
            Self {
                width: 1920,
                height: 1080,
                depth: 24,
                framerate: 30,
                audio_codec: proto::AudioCodec::Opus,
                audio_bitrate: 128,
                audio_frequency: 44100,
                video_codec: proto::VideoCodec::H264Main,
                video_bitrate: 4500,
                keyframe_interval: 0.0,
                audio_quality: 0,
                video_quality: 0,
            }
        }
    }

    impl Default for EncodingOptions {
        fn default() -> Self {
            Self::new()
        }
    }

    pub const H264_720P_30: EncodingOptions =
        EncodingOptions { width: 1280, height: 720, video_bitrate: 3000, ..EncodingOptions::new() };
    pub const H264_720P_60: EncodingOptions =
        EncodingOptions { width: 1280, height: 720, framerate: 60, ..EncodingOptions::new() };
    pub const H264_1080P_30: EncodingOptions = EncodingOptions::new();
    pub const H264_1080P_60: EncodingOptions =
        EncodingOptions { framerate: 60, video_bitrate: 6000, ..EncodingOptions::new() };
    pub const PORTRAIT_H264_720P_30: EncodingOptions =
        EncodingOptions { width: 720, height: 1280, video_bitrate: 3000, ..EncodingOptions::new() };
    pub const PORTRAIT_H264_720P_60: EncodingOptions =
        EncodingOptions { width: 720, height: 1280, framerate: 60, ..EncodingOptions::new() };
    pub const PORTRAIT_H264_1080P_30: EncodingOptions =
        EncodingOptions { width: 1080, height: 1920, ..EncodingOptions::new() };
    pub const PORTRAIT_H264_1080P_60: EncodingOptions = EncodingOptions {
        width: 1080,
        height: 1920,
        framerate: 60,
        video_bitrate: 6000,
        ..EncodingOptions::new()
    };
}
