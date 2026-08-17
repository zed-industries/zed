/*
 * Copyright 2025 LiveKit, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "livekit/rtp_parameters.h"

namespace livekit_ffi {

webrtc::RtcpFeedback to_native_rtcp_feedback(RtcpFeedback feedback) {
  webrtc::RtcpFeedback native{};
  native.type = static_cast<webrtc::RtcpFeedbackType>(feedback.feedback_type);
  if (feedback.has_message_type)
    native.message_type =
        static_cast<webrtc::RtcpFeedbackMessageType>(feedback.message_type);

  return native;
}

webrtc::RtpCodecCapability to_native_rtp_codec_capability(
    RtpCodecCapability capability) {
  webrtc::RtpCodecCapability native{};
  // native.mime_type(); IGNORED

  native.name = capability.name.c_str();
  native.kind = static_cast<webrtc::MediaType>(capability.kind);

  if (capability.has_clock_rate)
    native.clock_rate = capability.clock_rate;

  if (capability.has_preferred_payload_type)
    native.preferred_payload_type = capability.preferred_payload_type;

  if (capability.has_num_channels)
    native.num_channels = capability.num_channels;

  for (auto feedback : capability.rtcp_feedback)
    native.rtcp_feedback.push_back(to_native_rtcp_feedback(feedback));

  for (auto pair : capability.parameters)
    native.parameters.insert(std::pair(pair.key, pair.value));

  return native;
}

webrtc::RtpHeaderExtensionCapability to_native_rtp_header_extension_capability(
    RtpHeaderExtensionCapability header) {
  webrtc::RtpHeaderExtensionCapability native{};
  native.uri = header.uri.c_str();

  if (header.has_preferred_id)
    native.preferred_id = header.preferred_id;

  native.preferred_encrypt = header.preferred_encrypt;
  native.direction =
      static_cast<webrtc::RtpTransceiverDirection>(header.direction);

  return native;
}

webrtc::RtpExtension to_native_rtp_extension(RtpExtension ext) {
  webrtc::RtpExtension native{};
  native.uri = ext.uri.c_str();
  native.id = ext.id;
  native.encrypt = ext.encrypt;
  return native;
}

webrtc::RtpFecParameters to_rtp_fec_parameters(RtpFecParameters fec) {
  webrtc::RtpFecParameters native{};

  if (fec.has_ssrc)
    native.ssrc = fec.ssrc;

  native.mechanism = static_cast<webrtc::FecMechanism>(fec.mechanism);
  return native;
}

webrtc::RtpRtxParameters to_rtp_rtx_parameters(RtpRtxParameters rtx) {
  webrtc::RtpRtxParameters native{};

  if (rtx.has_ssrc)
    native.ssrc = rtx.ssrc;
  return native;
}

webrtc::RtpEncodingParameters to_native_rtp_encoding_paramters(
    RtpEncodingParameters parameters) {
  webrtc::RtpEncodingParameters native{};
  native.rid = parameters.rid.c_str();

  if (parameters.has_ssrc)
    native.ssrc = parameters.ssrc;

  native.active = parameters.active;
  if (parameters.has_max_framerate)
    native.max_framerate = parameters.max_framerate;

  native.adaptive_ptime = parameters.adaptive_ptime;
  if (parameters.has_max_bitrate_bps)
    native.max_bitrate_bps = parameters.max_bitrate_bps;

  if (parameters.has_min_bitrate_bps)
    native.min_bitrate_bps = parameters.min_bitrate_bps;

  native.bitrate_priority = parameters.bitrate_priority;
  native.network_priority =
      static_cast<webrtc::Priority>(parameters.network_priority);

  if (parameters.has_scalability_mode)
    native.scalability_mode = parameters.scalability_mode.c_str();

  if (parameters.has_num_temporal_layers)
    native.num_temporal_layers = parameters.num_temporal_layers;

  if (parameters.has_scale_resolution_down_by)
    native.scale_resolution_down_by = parameters.scale_resolution_down_by;
  return native;
}

webrtc::RtpCodecParameters to_native_rtp_codec_parameters(
    RtpCodecParameters params) {
  webrtc::RtpCodecParameters native{};
  native.name = params.name.c_str();
  native.kind = static_cast<webrtc::MediaType>(params.kind);
  native.payload_type = params.payload_type;

  for (auto pair : params.parameters)
    native.parameters.insert(std::pair(pair.key, pair.value));

  for (auto feedback : params.rtcp_feedback)
    native.rtcp_feedback.push_back(to_native_rtcp_feedback(feedback));

  if (params.has_num_channels)
    native.num_channels = params.num_channels;

  if (params.has_clock_rate)
    native.clock_rate = params.clock_rate;

  return native;
}

webrtc::RtpCapabilities to_rtp_capabilities(RtpCapabilities capabilities) {
  webrtc::RtpCapabilities native{};
  for (auto codec : capabilities.codecs)
    native.codecs.push_back(to_native_rtp_codec_capability(codec));

  for (auto header : capabilities.header_extensions)
    native.header_extensions.push_back(
        to_native_rtp_header_extension_capability(header));

  for (auto fec : capabilities.fec)
    native.fec.push_back(static_cast<webrtc::FecMechanism>(fec));

  return native;
}

webrtc::RtcpParameters to_native_rtcp_paramaters(RtcpParameters params) {
  webrtc::RtcpParameters native{};
  if (params.has_ssrc)
    native.ssrc = params.ssrc;

  native.mux = params.mux;
  native.cname = params.cname.c_str();
  native.reduced_size = params.reduced_size;
  return native;
}

webrtc::RtpParameters to_native_rtp_parameters(RtpParameters params) {
  webrtc::RtpParameters native{};
  native.transaction_id = params.transaction_id.c_str();
  native.mid = params.mid.c_str();

  for (auto codec : params.codecs)
    native.codecs.push_back(to_native_rtp_codec_parameters(codec));

  for (auto header : params.header_extensions)
    native.header_extensions.push_back(to_native_rtp_extension(header));

  for (auto encoding : params.encodings)
    native.encodings.push_back(to_native_rtp_encoding_paramters(encoding));

  native.rtcp = to_native_rtcp_paramaters(params.rtcp);

  if (params.has_degradation_preference)
    native.degradation_preference = static_cast<webrtc::DegradationPreference>(
        params.degradation_preference);

  return native;
}

RtcpFeedback to_rust_rtcp_feedback(webrtc::RtcpFeedback feedback) {
  RtcpFeedback rust{};
  rust.feedback_type = static_cast<RtcpFeedbackType>(feedback.type);

  if (feedback.message_type.has_value()) {
    rust.has_message_type = true;
    rust.message_type =
        static_cast<RtcpFeedbackMessageType>(feedback.message_type.value());
  }

  return rust;
}

RtpCodecCapability to_rust_rtp_codec_capability(
    webrtc::RtpCodecCapability capability) {
  RtpCodecCapability rust{};
  rust.mime_type = capability.mime_type();
  rust.name = capability.name;
  rust.kind = static_cast<MediaType>(capability.kind);

  if (capability.clock_rate.has_value()) {
    rust.has_clock_rate = true;
    rust.clock_rate = capability.clock_rate.value();
  }

  if (capability.preferred_payload_type.has_value()) {
    rust.has_preferred_payload_type = true;
    rust.preferred_payload_type = capability.preferred_payload_type.value();
  }

  if (capability.num_channels) {
    rust.has_num_channels = true;
    rust.num_channels = capability.num_channels.value();
  }

  for (auto feedback : capability.rtcp_feedback)
    rust.rtcp_feedback.push_back(to_rust_rtcp_feedback(feedback));

  for (auto param : capability.parameters)
    rust.parameters.push_back(StringKeyValue{param.first, param.second});

  return rust;
}

RtpHeaderExtensionCapability to_rust_rtp_header_extension_capability(
    webrtc::RtpHeaderExtensionCapability header) {
  RtpHeaderExtensionCapability rust{};
  rust.uri = header.uri;
  if (header.preferred_id.has_value()) {
    rust.has_preferred_id = true;
    rust.preferred_id = header.preferred_id.value();
  }

  rust.preferred_encrypt = header.preferred_encrypt;
  rust.direction = static_cast<RtpTransceiverDirection>(header.direction);
  return rust;
}

RtpExtension to_rust_rtp_extension(webrtc::RtpExtension ext) {
  RtpExtension rust{};
  rust.uri = ext.uri;
  rust.id = ext.id;
  rust.encrypt = ext.encrypt;
  return rust;
}

RtpFecParameters to_rust_rtp_fec_parameters(webrtc::RtpFecParameters fec) {
  RtpFecParameters rust{};
  if (fec.ssrc.has_value()) {
    rust.has_ssrc = true;
    rust.ssrc = fec.ssrc.value();
  }

  rust.mechanism = static_cast<FecMechanism>(rust.mechanism);
  return rust;
}

RtpRtxParameters to_rust_rtp_rtx_parameters(webrtc::RtpRtxParameters param) {
  RtpRtxParameters rust{};
  if (param.ssrc.has_value()) {
    rust.has_ssrc = param.ssrc.has_value();
    rust.ssrc = param.ssrc.value();
  }
  return rust;
}

RtpEncodingParameters to_rust_rtp_encoding_parameters(
    webrtc::RtpEncodingParameters params) {
  RtpEncodingParameters rust{};
  if (params.ssrc.has_value()) {
    rust.has_ssrc = params.ssrc.has_value();
    rust.ssrc = params.ssrc.value();
  }

  rust.bitrate_priority = params.bitrate_priority;
  rust.network_priority = static_cast<Priority>(params.network_priority);
  if (params.max_bitrate_bps.has_value()) {
    rust.has_max_bitrate_bps = true;
    rust.max_bitrate_bps = params.max_bitrate_bps.value();
  }

  if (params.min_bitrate_bps.has_value()) {
    rust.has_min_bitrate_bps = true;
    rust.min_bitrate_bps = params.min_bitrate_bps.value();
  }

  if (params.max_framerate.has_value()) {
    rust.has_max_framerate = true;
    rust.max_framerate = params.max_framerate.value();
  }

  if (params.num_temporal_layers.has_value()) {
    rust.has_num_temporal_layers = true;
    rust.num_temporal_layers = params.num_temporal_layers.value();
  }

  if (params.scale_resolution_down_by.has_value()) {
    rust.has_scale_resolution_down_by = true;
    rust.scale_resolution_down_by = params.scale_resolution_down_by.value();
  }

  if (params.scalability_mode.has_value()) {
    rust.has_scalability_mode = true;
    rust.scalability_mode = params.scalability_mode.value();
  }

  rust.active = params.active;
  rust.rid = params.rid;
  rust.adaptive_ptime = params.adaptive_ptime;
  return rust;
}

RtpCodecParameters to_rust_rtp_codec_parameters(
    webrtc::RtpCodecParameters params) {
  RtpCodecParameters rust{};
  rust.mime_type = params.mime_type();
  rust.name = params.name;
  rust.kind = static_cast<MediaType>(params.kind);
  rust.payload_type = params.payload_type;
  if (params.clock_rate.has_value()) {
    rust.has_clock_rate = true;
    rust.clock_rate = params.clock_rate.value();
  }

  if (params.num_channels.has_value()) {
    rust.has_num_channels = true;
    rust.num_channels = params.num_channels.value();
  }

  for (auto feedback : params.rtcp_feedback)
    rust.rtcp_feedback.push_back(to_rust_rtcp_feedback(feedback));

  for (auto pair : params.parameters)
    rust.parameters.push_back(StringKeyValue{pair.first, pair.second});

  return rust;
}

RtpCapabilities to_rust_rtp_capabilities(webrtc::RtpCapabilities capabilities) {
  RtpCapabilities rust{};
  for (auto codec : capabilities.codecs)
    rust.codecs.push_back(to_rust_rtp_codec_capability(codec));

  for (auto header : capabilities.header_extensions)
    rust.header_extensions.push_back(
        to_rust_rtp_header_extension_capability(header));

  for (auto fec : capabilities.fec)
    rust.fec.push_back(static_cast<FecMechanism>(fec));

  return rust;
}

RtcpParameters to_rust_rtcp_parameters(webrtc::RtcpParameters params) {
  RtcpParameters rust{};
  if (params.ssrc.has_value()) {
    rust.has_ssrc = true;
    rust.ssrc = params.ssrc.value();
  }

  rust.cname = params.cname;
  rust.reduced_size = params.reduced_size;
  rust.mux = params.mux;
  return rust;
}

RtpParameters to_rust_rtp_parameters(webrtc::RtpParameters params) {
  RtpParameters rust{};
  rust.transaction_id = params.transaction_id;
  rust.mid = params.mid;

  for (auto codec : params.codecs)
    rust.codecs.push_back(to_rust_rtp_codec_parameters(codec));

  for (auto header : params.header_extensions)
    rust.header_extensions.push_back(to_rust_rtp_extension(header));

  for (auto encoding : params.encodings)
    rust.encodings.push_back(to_rust_rtp_encoding_parameters(encoding));

  rust.rtcp = to_rust_rtcp_parameters(params.rtcp);

  if (params.degradation_preference.has_value()) {
    rust.has_degradation_preference = true;
    rust.degradation_preference = static_cast<DegradationPreference>(
        params.degradation_preference.value());
  }

  return rust;
}

}  // namespace livekit_ffi
