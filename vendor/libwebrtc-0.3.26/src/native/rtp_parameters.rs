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

use webrtc_sys::{rtp_parameters as sys_rp, webrtc as sys_webrtc};

use crate::rtp_parameters::*;

impl From<sys_webrtc::ffi::Priority> for Priority {
    fn from(value: sys_webrtc::ffi::Priority) -> Self {
        match value {
            sys_webrtc::ffi::Priority::VeryLow => Self::VeryLow,
            sys_webrtc::ffi::Priority::Low => Self::Low,
            sys_webrtc::ffi::Priority::Medium => Self::Medium,
            sys_webrtc::ffi::Priority::High => Self::High,
            _ => panic!("unknown Priority"),
        }
    }
}

impl From<sys_rp::ffi::RtpExtension> for RtpHeaderExtensionParameters {
    fn from(value: sys_rp::ffi::RtpExtension) -> Self {
        Self { uri: value.uri, id: value.id, encrypted: value.encrypt }
    }
}

impl From<sys_rp::ffi::RtpParameters> for RtpParameters {
    fn from(value: sys_rp::ffi::RtpParameters) -> Self {
        Self {
            codecs: value.codecs.into_iter().map(Into::into).collect(),
            header_extensions: value.header_extensions.into_iter().map(Into::into).collect(),
            rtcp: value.rtcp.into(),
        }
    }
}

impl From<sys_rp::ffi::RtpCodecParameters> for RtpCodecParameters {
    fn from(value: sys_rp::ffi::RtpCodecParameters) -> Self {
        Self {
            mime_type: value.mime_type,
            payload_type: value.payload_type as u8,
            clock_rate: value.has_clock_rate.then_some(value.clock_rate as u64),
            channels: value.has_num_channels.then_some(value.num_channels as u16),
        }
    }
}

impl From<sys_rp::ffi::RtcpParameters> for RtcpParameters {
    fn from(value: sys_rp::ffi::RtcpParameters) -> Self {
        Self { cname: value.cname, reduced_size: value.reduced_size }
    }
}

impl From<sys_rp::ffi::RtpEncodingParameters> for RtpEncodingParameters {
    fn from(value: sys_rp::ffi::RtpEncodingParameters) -> Self {
        Self {
            active: value.active,
            max_bitrate: value.has_max_bitrate_bps.then_some(value.max_bitrate_bps as u64),
            max_framerate: value.has_max_framerate.then_some(value.max_framerate),
            priority: value.network_priority.into(),
            rid: value.rid,
            scale_resolution_down_by: value
                .has_scale_resolution_down_by
                .then_some(value.scale_resolution_down_by),
        }
    }
}

impl From<sys_rp::ffi::RtpCodecCapability> for RtpCodecCapability {
    fn from(value: sys_rp::ffi::RtpCodecCapability) -> Self {
        Self {
            channels: value.has_num_channels.then_some(value.num_channels as u16),
            mime_type: value.mime_type,
            clock_rate: value.has_clock_rate.then_some(value.clock_rate as u64),
            sdp_fmtp_line: {
                let parameters: Vec<String> = value
                    .parameters
                    .into_iter()
                    .map(|key_value| {
                        if !key_value.key.is_empty() {
                            format!("{}={}", key_value.key, key_value.value)
                        } else {
                            key_value.value
                        }
                    })
                    .collect();

                if !parameters.is_empty() {
                    Some(parameters.join(";"))
                } else {
                    None
                }
            },
        }
    }
}

impl From<sys_rp::ffi::RtpHeaderExtensionCapability> for RtpHeaderExtensionCapability {
    fn from(value: sys_rp::ffi::RtpHeaderExtensionCapability) -> Self {
        Self { direction: value.direction.into(), uri: value.uri }
    }
}

impl From<sys_rp::ffi::RtpCapabilities> for RtpCapabilities {
    fn from(value: sys_rp::ffi::RtpCapabilities) -> Self {
        Self {
            codecs: value.codecs.into_iter().map(Into::into).collect(),
            header_extensions: value.header_extensions.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<Priority> for sys_webrtc::ffi::Priority {
    fn from(value: Priority) -> Self {
        match value {
            Priority::VeryLow => Self::VeryLow,
            Priority::Low => Self::Low,
            Priority::Medium => Self::Medium,
            Priority::High => Self::High,
        }
    }
}

impl From<RtpHeaderExtensionParameters> for sys_rp::ffi::RtpExtension {
    fn from(value: RtpHeaderExtensionParameters) -> Self {
        Self { uri: value.uri, id: value.id, encrypt: value.encrypted }
    }
}

impl From<RtpParameters> for sys_rp::ffi::RtpParameters {
    fn from(value: RtpParameters) -> Self {
        Self {
            codecs: value.codecs.into_iter().map(Into::into).collect(),
            header_extensions: value.header_extensions.into_iter().map(Into::into).collect(),
            encodings: Vec::new(),
            rtcp: value.rtcp.into(),
            transaction_id: "".to_string(),
            mid: "".to_string(),
            has_degradation_preference: false,
            degradation_preference: sys_rp::ffi::DegradationPreference::Balanced,
        }
    }
}

impl From<RtpCodecParameters> for sys_rp::ffi::RtpCodecParameters {
    fn from(value: RtpCodecParameters) -> Self {
        Self {
            payload_type: value.payload_type as i32,
            mime_type: value.mime_type,
            has_clock_rate: value.clock_rate.is_some(),
            clock_rate: value.clock_rate.unwrap_or_default() as i32,
            has_num_channels: value.channels.is_some(),
            num_channels: value.channels.unwrap_or_default() as i32,
            name: "".to_string(),
            kind: sys_rp::ffi::MediaType::Audio,
            has_max_ptime: false,
            max_ptime: 0,
            has_ptime: false,
            ptime: 0,
            rtcp_feedback: Vec::new(),
            parameters: Vec::new(),
        }
    }
}

impl From<RtcpParameters> for sys_rp::ffi::RtcpParameters {
    fn from(value: RtcpParameters) -> Self {
        Self {
            cname: value.cname,
            reduced_size: value.reduced_size,
            has_ssrc: false,
            ssrc: 0,
            mux: false,
        }
    }
}

impl From<RtpEncodingParameters> for sys_rp::ffi::RtpEncodingParameters {
    fn from(value: RtpEncodingParameters) -> Self {
        Self {
            active: value.active,
            has_max_bitrate_bps: value.max_bitrate.is_some(),
            max_bitrate_bps: value.max_bitrate.unwrap_or_default() as i32,
            has_max_framerate: value.max_framerate.is_some(),
            max_framerate: value.max_framerate.unwrap_or_default(),
            network_priority: value.priority.into(),
            rid: value.rid,
            has_scale_resolution_down_by: value.scale_resolution_down_by.is_some(),
            scale_resolution_down_by: value.scale_resolution_down_by.unwrap_or_default(),
            adaptive_ptime: false,
            bitrate_priority: sys_rp::DEFAULT_BITRATE_PRIORITY,
            has_min_bitrate_bps: false,
            min_bitrate_bps: 0,
            has_num_temporal_layers: false,
            num_temporal_layers: 0,
            has_scalability_mode: false,
            scalability_mode: "".to_string(),
            has_ssrc: false,
            ssrc: 0,
        }
    }
}

impl From<RtpCodecCapability> for sys_rp::ffi::RtpCodecCapability {
    fn from(value: RtpCodecCapability) -> Self {
        let mime_type: Vec<&str> = value.mime_type.split('/').collect();
        let kind = match mime_type[0] {
            "audio" => sys_webrtc::ffi::MediaType::Audio,
            "video" => sys_webrtc::ffi::MediaType::Video,
            _ => panic!("invalid media type"),
        };
        let name = mime_type[1].to_string();

        Self {
            name,
            kind,
            has_clock_rate: value.clock_rate.is_some(),
            clock_rate: value.clock_rate.unwrap_or_default() as i32,
            has_num_channels: value.channels.is_some(),
            num_channels: value.channels.unwrap_or_default() as i32,
            parameters: {
                value
                    .sdp_fmtp_line
                    .map(|sdp_fmtp_line| {
                        sdp_fmtp_line
                            .split(';')
                            .map(|v| {
                                let key_value: Vec<&str> = v.split('=').collect();
                                if key_value.len() == 2 {
                                    sys_rp::ffi::StringKeyValue {
                                        key: key_value[0].to_string(),
                                        value: key_value[1].to_string(),
                                    }
                                } else {
                                    sys_rp::ffi::StringKeyValue {
                                        key: "".to_string(),
                                        value: key_value[0].to_string(),
                                    }
                                }
                            })
                            .collect()
                    })
                    .unwrap_or_default()
            },
            // Ignore
            mime_type: String::default(), // !!
            has_preferred_payload_type: false,
            preferred_payload_type: 0,
            rtcp_feedback: Vec::default(),
        }
    }
}
