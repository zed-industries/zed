/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_STATS_RTCSTATS_OBJECTS_H_
#define API_STATS_RTCSTATS_OBJECTS_H_

#include <stdint.h>

#include <map>
#include <memory>
#include <optional>
#include <string>

#include "api/stats/rtc_stats.h"
#include "api/units/timestamp.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// https://w3c.github.io/webrtc-stats/#certificatestats-dict*
class RTC_EXPORT RTCCertificateStats final : public RTCStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCCertificateStats);
  RTCCertificateStats(std::string id, Timestamp timestamp);
  ~RTCCertificateStats() override;

  std::optional<std::string> fingerprint;
  std::optional<std::string> fingerprint_algorithm;
  std::optional<std::string> base64_certificate;
  std::optional<std::string> issuer_certificate_id;
};

// https://w3c.github.io/webrtc-stats/#codec-dict*
class RTC_EXPORT RTCCodecStats final : public RTCStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCCodecStats);
  RTCCodecStats(std::string id, Timestamp timestamp);
  ~RTCCodecStats() override;

  std::optional<std::string> transport_id;
  std::optional<uint32_t> payload_type;
  std::optional<std::string> mime_type;
  std::optional<uint32_t> clock_rate;
  std::optional<uint32_t> channels;
  std::optional<std::string> sdp_fmtp_line;
};

// https://w3c.github.io/webrtc-stats/#dcstats-dict*
class RTC_EXPORT RTCDataChannelStats final : public RTCStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCDataChannelStats);
  RTCDataChannelStats(std::string id, Timestamp timestamp);
  ~RTCDataChannelStats() override;

  std::optional<std::string> label;
  std::optional<std::string> protocol;
  std::optional<int32_t> data_channel_identifier;
  std::optional<std::string> state;
  std::optional<uint32_t> messages_sent;
  std::optional<uint64_t> bytes_sent;
  std::optional<uint32_t> messages_received;
  std::optional<uint64_t> bytes_received;
};

// https://w3c.github.io/webrtc-stats/#candidatepair-dict*
class RTC_EXPORT RTCIceCandidatePairStats final : public RTCStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCIceCandidatePairStats);
  RTCIceCandidatePairStats(std::string id, Timestamp timestamp);
  ~RTCIceCandidatePairStats() override;

  std::optional<std::string> transport_id;
  std::optional<std::string> local_candidate_id;
  std::optional<std::string> remote_candidate_id;
  std::optional<std::string> state;
  // Obsolete: priority
  std::optional<uint64_t> priority;
  std::optional<bool> nominated;
  // `writable` does not exist in the spec and old comments suggest it used to
  // exist but was incorrectly implemented.
  // TODO(https://crbug.com/webrtc/14171): Standardize and/or modify
  // implementation.
  std::optional<bool> writable;
  std::optional<uint64_t> packets_sent;
  std::optional<uint64_t> packets_received;
  std::optional<uint64_t> bytes_sent;
  std::optional<uint64_t> bytes_received;
  std::optional<double> total_round_trip_time;
  std::optional<double> current_round_trip_time;
  std::optional<double> available_outgoing_bitrate;
  std::optional<double> available_incoming_bitrate;
  std::optional<uint64_t> requests_received;
  std::optional<uint64_t> requests_sent;
  std::optional<uint64_t> responses_received;
  std::optional<uint64_t> responses_sent;
  std::optional<uint64_t> consent_requests_sent;
  std::optional<uint64_t> packets_discarded_on_send;
  std::optional<uint64_t> bytes_discarded_on_send;
  std::optional<double> last_packet_received_timestamp;
  std::optional<double> last_packet_sent_timestamp;
};

// https://w3c.github.io/webrtc-stats/#icecandidate-dict*
class RTC_EXPORT RTCIceCandidateStats : public RTCStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCIceCandidateStats);
  ~RTCIceCandidateStats() override;

  std::optional<std::string> transport_id;
  // Obsolete: is_remote
  std::optional<bool> is_remote;
  std::optional<std::string> network_type;
  std::optional<std::string> ip;
  std::optional<std::string> address;
  std::optional<int32_t> port;
  std::optional<std::string> protocol;
  std::optional<std::string> relay_protocol;
  std::optional<std::string> candidate_type;
  std::optional<int32_t> priority;
  std::optional<std::string> url;
  std::optional<std::string> foundation;
  std::optional<std::string> related_address;
  std::optional<int32_t> related_port;
  std::optional<std::string> username_fragment;
  std::optional<std::string> tcp_type;

  // The following metrics are NOT exposed to JavaScript. We should consider
  // standardizing or removing them.
  std::optional<bool> vpn;
  std::optional<std::string> network_adapter_type;

 protected:
  RTCIceCandidateStats(std::string id, Timestamp timestamp, bool is_remote);
};

// In the spec both local and remote varieties are of type RTCIceCandidateStats.
// But here we define them as subclasses of `RTCIceCandidateStats` because the
// `kType` need to be different ("RTCStatsType type") in the local/remote case.
// https://w3c.github.io/webrtc-stats/#rtcstatstype-str*
// This forces us to have to override copy() and type().
class RTC_EXPORT RTCLocalIceCandidateStats final : public RTCIceCandidateStats {
 public:
  static const char kType[];
  RTCLocalIceCandidateStats(std::string id, Timestamp timestamp);
  std::unique_ptr<RTCStats> copy() const override;
  const char* type() const override;
};

class RTC_EXPORT RTCRemoteIceCandidateStats final
    : public RTCIceCandidateStats {
 public:
  static const char kType[];
  RTCRemoteIceCandidateStats(std::string id, Timestamp timestamp);
  std::unique_ptr<RTCStats> copy() const override;
  const char* type() const override;
};

// https://w3c.github.io/webrtc-stats/#pcstats-dict*
class RTC_EXPORT RTCPeerConnectionStats final : public RTCStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCPeerConnectionStats);
  RTCPeerConnectionStats(std::string id, Timestamp timestamp);
  ~RTCPeerConnectionStats() override;

  std::optional<uint32_t> data_channels_opened;
  std::optional<uint32_t> data_channels_closed;
};

// https://w3c.github.io/webrtc-stats/#streamstats-dict*
class RTC_EXPORT RTCRtpStreamStats : public RTCStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCRtpStreamStats);
  ~RTCRtpStreamStats() override;

  std::optional<uint32_t> ssrc;
  std::optional<std::string> kind;
  std::optional<std::string> transport_id;
  std::optional<std::string> codec_id;

 protected:
  RTCRtpStreamStats(std::string id, Timestamp timestamp);
};

// https://www.w3.org/TR/webrtc-stats/#receivedrtpstats-dict*
class RTC_EXPORT RTCReceivedRtpStreamStats : public RTCRtpStreamStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCReceivedRtpStreamStats);
  ~RTCReceivedRtpStreamStats() override;

  std::optional<double> jitter;
  std::optional<int32_t> packets_lost;  // Signed per RFC 3550

 protected:
  RTCReceivedRtpStreamStats(std::string id, Timestamp timestamp);
};

// https://www.w3.org/TR/webrtc-stats/#sentrtpstats-dict*
class RTC_EXPORT RTCSentRtpStreamStats : public RTCRtpStreamStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCSentRtpStreamStats);
  ~RTCSentRtpStreamStats() override;

  std::optional<uint64_t> packets_sent;
  std::optional<uint64_t> bytes_sent;

 protected:
  RTCSentRtpStreamStats(std::string id, Timestamp timestamp);
};

// https://w3c.github.io/webrtc-stats/#inboundrtpstats-dict*
class RTC_EXPORT RTCInboundRtpStreamStats final
    : public RTCReceivedRtpStreamStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCInboundRtpStreamStats);
  RTCInboundRtpStreamStats(std::string id, Timestamp timestamp);
  ~RTCInboundRtpStreamStats() override;

  std::optional<std::string> playout_id;
  std::optional<std::string> track_identifier;
  std::optional<std::string> mid;
  std::optional<std::string> remote_id;
  std::optional<uint32_t> packets_received;
  std::optional<uint64_t> packets_discarded;
  std::optional<uint64_t> fec_packets_received;
  std::optional<uint64_t> fec_bytes_received;
  std::optional<uint64_t> fec_packets_discarded;
  // Inbound FEC SSRC. Only present if a mechanism like FlexFEC is negotiated.
  std::optional<uint32_t> fec_ssrc;
  std::optional<uint64_t> bytes_received;
  std::optional<uint64_t> header_bytes_received;
  // Inbound RTX stats. Only defined when RTX is used and it is therefore
  // possible to distinguish retransmissions.
  std::optional<uint64_t> retransmitted_packets_received;
  std::optional<uint64_t> retransmitted_bytes_received;
  std::optional<uint32_t> rtx_ssrc;

  std::optional<double> last_packet_received_timestamp;
  std::optional<double> jitter_buffer_delay;
  std::optional<double> jitter_buffer_target_delay;
  std::optional<double> jitter_buffer_minimum_delay;
  std::optional<uint64_t> jitter_buffer_emitted_count;
  std::optional<uint64_t> total_samples_received;
  std::optional<uint64_t> concealed_samples;
  std::optional<uint64_t> silent_concealed_samples;
  std::optional<uint64_t> concealment_events;
  std::optional<uint64_t> inserted_samples_for_deceleration;
  std::optional<uint64_t> removed_samples_for_acceleration;
  std::optional<double> audio_level;
  std::optional<double> total_audio_energy;
  std::optional<double> total_samples_duration;
  // Stats below are only implemented or defined for video.
  std::optional<uint32_t> frames_received;
  std::optional<uint32_t> frame_width;
  std::optional<uint32_t> frame_height;
  std::optional<double> frames_per_second;
  std::optional<uint32_t> frames_decoded;
  std::optional<uint32_t> key_frames_decoded;
  std::optional<uint32_t> frames_dropped;
  std::optional<double> total_decode_time;
  std::optional<double> total_processing_delay;
  std::optional<double> total_assembly_time;
  std::optional<uint32_t> frames_assembled_from_multiple_packets;
  // TODO(https://crbug.com/webrtc/15600): Implement framesRendered, which is
  // incremented at the same time that totalInterFrameDelay and
  // totalSquaredInterFrameDelay is incremented. (Dividing inter-frame delay by
  // framesDecoded is slightly wrong.)
  // https://w3c.github.io/webrtc-stats/#dom-rtcinboundrtpstreamstats-framesrendered
  //
  // TODO(https://crbug.com/webrtc/15601): Inter-frame, pause and freeze metrics
  // all related to when the frame is rendered, but our implementation measures
  // at delivery to sink, not at actual render time. When we have an actual
  // frame rendered callback, move the calculating of these metrics to there in
  // order to make them more accurate.
  std::optional<double> total_inter_frame_delay;
  std::optional<double> total_squared_inter_frame_delay;
  std::optional<uint32_t> pause_count;
  std::optional<double> total_pauses_duration;
  std::optional<uint32_t> freeze_count;
  std::optional<double> total_freezes_duration;
  // https://w3c.github.io/webrtc-provisional-stats/#dom-rtcinboundrtpstreamstats-contenttype
  std::optional<std::string> content_type;
  // Only populated if audio/video sync is enabled.
  // TODO(https://crbug.com/webrtc/14177): Expose even if A/V sync is off?
  std::optional<double> estimated_playout_timestamp;
  // Only defined for video.
  // In JavaScript, this is only exposed if HW exposure is allowed.
  std::optional<std::string> decoder_implementation;
  // FIR and PLI counts are only defined for |kind == "video"|.
  std::optional<uint32_t> fir_count;
  std::optional<uint32_t> pli_count;
  std::optional<uint32_t> nack_count;
  std::optional<uint64_t> qp_sum;
  std::optional<double> total_corruption_probability;
  std::optional<double> total_squared_corruption_probability;
  std::optional<uint64_t> corruption_measurements;
  // This is a remnant of the legacy getStats() API. When the "video-timing"
  // header extension is used,
  // https://webrtc.github.io/webrtc-org/experiments/rtp-hdrext/video-timing/,
  // `googTimingFrameInfo` is exposed with the value of
  // TimingFrameInfo::ToString().
  // TODO(https://crbug.com/webrtc/14586): Unship or standardize this metric.
  std::optional<std::string> goog_timing_frame_info;
  // In JavaScript, this is only exposed if HW exposure is allowed.
  std::optional<bool> power_efficient_decoder;

  // The following metrics are NOT exposed to JavaScript. We should consider
  // standardizing or removing them.
  std::optional<uint64_t> jitter_buffer_flushes;
  std::optional<uint64_t> delayed_packet_outage_samples;
  std::optional<double> relative_packet_arrival_delay;
  std::optional<uint32_t> interruption_count;
  std::optional<double> total_interruption_duration;
  std::optional<double> min_playout_delay;
};

// https://w3c.github.io/webrtc-stats/#outboundrtpstats-dict*
class RTC_EXPORT RTCOutboundRtpStreamStats final
    : public RTCSentRtpStreamStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCOutboundRtpStreamStats);
  RTCOutboundRtpStreamStats(std::string id, Timestamp timestamp);
  ~RTCOutboundRtpStreamStats() override;

  std::optional<std::string> media_source_id;
  std::optional<std::string> remote_id;
  std::optional<std::string> mid;
  std::optional<std::string> rid;
  std::optional<uint32_t> encoding_index;
  std::optional<uint64_t> retransmitted_packets_sent;
  std::optional<uint64_t> header_bytes_sent;
  std::optional<uint64_t> retransmitted_bytes_sent;
  std::optional<double> target_bitrate;
  std::optional<uint32_t> frames_encoded;
  std::optional<uint32_t> key_frames_encoded;
  std::optional<double> total_encode_time;
  std::optional<uint64_t> total_encoded_bytes_target;
  std::optional<uint32_t> frame_width;
  std::optional<uint32_t> frame_height;
  std::optional<double> frames_per_second;
  std::optional<uint32_t> frames_sent;
  std::optional<uint32_t> huge_frames_sent;
  std::optional<double> total_packet_send_delay;
  std::optional<std::string> quality_limitation_reason;
  std::optional<std::map<std::string, double>> quality_limitation_durations;
  // https://w3c.github.io/webrtc-stats/#dom-rtcoutboundrtpstreamstats-qualitylimitationresolutionchanges
  std::optional<uint32_t> quality_limitation_resolution_changes;
  // https://w3c.github.io/webrtc-provisional-stats/#dom-rtcoutboundrtpstreamstats-contenttype
  std::optional<std::string> content_type;
  // In JavaScript, this is only exposed if HW exposure is allowed.
  // Only implemented for video.
  // TODO(https://crbug.com/webrtc/14178): Implement for audio as well.
  std::optional<std::string> encoder_implementation;
  // FIR and PLI counts are only defined for |kind == "video"|.
  std::optional<uint32_t> fir_count;
  std::optional<uint32_t> pli_count;
  std::optional<uint32_t> nack_count;
  std::optional<uint64_t> qp_sum;
  std::optional<bool> active;
  // In JavaScript, this is only exposed if HW exposure is allowed.
  std::optional<bool> power_efficient_encoder;
  std::optional<std::string> scalability_mode;

  // RTX ssrc. Only present if RTX is negotiated.
  std::optional<uint32_t> rtx_ssrc;
};

// https://w3c.github.io/webrtc-stats/#remoteinboundrtpstats-dict*
class RTC_EXPORT RTCRemoteInboundRtpStreamStats final
    : public RTCReceivedRtpStreamStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCRemoteInboundRtpStreamStats);
  RTCRemoteInboundRtpStreamStats(std::string id, Timestamp timestamp);
  ~RTCRemoteInboundRtpStreamStats() override;

  std::optional<std::string> local_id;
  std::optional<double> round_trip_time;
  std::optional<double> fraction_lost;
  std::optional<double> total_round_trip_time;
  std::optional<int32_t> round_trip_time_measurements;
};

// https://w3c.github.io/webrtc-stats/#remoteoutboundrtpstats-dict*
class RTC_EXPORT RTCRemoteOutboundRtpStreamStats final
    : public RTCSentRtpStreamStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCRemoteOutboundRtpStreamStats);
  RTCRemoteOutboundRtpStreamStats(std::string id, Timestamp timestamp);
  ~RTCRemoteOutboundRtpStreamStats() override;

  std::optional<std::string> local_id;
  std::optional<double> remote_timestamp;
  std::optional<uint64_t> reports_sent;
  std::optional<double> round_trip_time;
  std::optional<uint64_t> round_trip_time_measurements;
  std::optional<double> total_round_trip_time;
};

// https://w3c.github.io/webrtc-stats/#dom-rtcmediasourcestats
class RTC_EXPORT RTCMediaSourceStats : public RTCStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCMediaSourceStats);
  ~RTCMediaSourceStats() override;

  std::optional<std::string> track_identifier;
  std::optional<std::string> kind;

 protected:
  RTCMediaSourceStats(std::string id, Timestamp timestamp);
};

// https://w3c.github.io/webrtc-stats/#dom-rtcaudiosourcestats
class RTC_EXPORT RTCAudioSourceStats final : public RTCMediaSourceStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCAudioSourceStats);
  RTCAudioSourceStats(std::string id, Timestamp timestamp);
  ~RTCAudioSourceStats() override;

  std::optional<double> audio_level;
  std::optional<double> total_audio_energy;
  std::optional<double> total_samples_duration;
  std::optional<double> echo_return_loss;
  std::optional<double> echo_return_loss_enhancement;
};

// https://w3c.github.io/webrtc-stats/#dom-rtcvideosourcestats
class RTC_EXPORT RTCVideoSourceStats final : public RTCMediaSourceStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCVideoSourceStats);
  RTCVideoSourceStats(std::string id, Timestamp timestamp);
  ~RTCVideoSourceStats() override;

  std::optional<uint32_t> width;
  std::optional<uint32_t> height;
  std::optional<uint32_t> frames;
  std::optional<double> frames_per_second;
};

// https://w3c.github.io/webrtc-stats/#transportstats-dict*
class RTC_EXPORT RTCTransportStats final : public RTCStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCTransportStats);
  RTCTransportStats(std::string id, Timestamp timestamp);
  ~RTCTransportStats() override;

  std::optional<uint64_t> bytes_sent;
  std::optional<uint64_t> packets_sent;
  std::optional<uint64_t> bytes_received;
  std::optional<uint64_t> packets_received;
  std::optional<std::string> rtcp_transport_stats_id;
  std::optional<std::string> dtls_state;
  std::optional<std::string> selected_candidate_pair_id;
  std::optional<std::string> local_certificate_id;
  std::optional<std::string> remote_certificate_id;
  std::optional<std::string> tls_version;
  std::optional<std::string> dtls_cipher;
  std::optional<std::string> dtls_role;
  std::optional<std::string> srtp_cipher;
  std::optional<uint32_t> selected_candidate_pair_changes;
  std::optional<std::string> ice_role;
  std::optional<std::string> ice_local_username_fragment;
  std::optional<std::string> ice_state;
};

// https://w3c.github.io/webrtc-stats/#playoutstats-dict*
class RTC_EXPORT RTCAudioPlayoutStats final : public RTCStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCAudioPlayoutStats);
  RTCAudioPlayoutStats(const std::string& id, Timestamp timestamp);
  ~RTCAudioPlayoutStats() override;

  std::optional<std::string> kind;
  std::optional<double> synthesized_samples_duration;
  std::optional<uint64_t> synthesized_samples_events;
  std::optional<double> total_samples_duration;
  std::optional<double> total_playout_delay;
  std::optional<uint64_t> total_samples_count;
};

}  // namespace webrtc

#endif  // API_STATS_RTCSTATS_OBJECTS_H_
