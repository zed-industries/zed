/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef LOGGING_RTC_EVENT_LOG_RTC_EVENT_LOG_PARSER_H_
#define LOGGING_RTC_EVENT_LOG_RTC_EVENT_LOG_PARSER_H_

#include <cstddef>
#include <cstdint>
#include <iterator>
#include <limits>
#include <map>
#include <set>
#include <type_traits>
#include <vector>

#include "absl/base/attributes.h"
#include "absl/strings/string_view.h"
#include "api/candidate.h"
#include "api/dtls_transport_interface.h"
#include "api/rtp_parameters.h"
#include "api/transport/bandwidth_usage.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "logging/rtc_event_log/events/logged_rtp_rtcp.h"
#include "logging/rtc_event_log/events/rtc_event_alr_state.h"
#include "logging/rtc_event_log/events/rtc_event_audio_network_adaptation.h"
#include "logging/rtc_event_log/events/rtc_event_audio_playout.h"
#include "logging/rtc_event_log/events/rtc_event_audio_receive_stream_config.h"
#include "logging/rtc_event_log/events/rtc_event_audio_send_stream_config.h"
#include "logging/rtc_event_log/events/rtc_event_begin_log.h"
#include "logging/rtc_event_log/events/rtc_event_bwe_update_delay_based.h"
#include "logging/rtc_event_log/events/rtc_event_bwe_update_loss_based.h"
#include "logging/rtc_event_log/events/rtc_event_dtls_transport_state.h"
#include "logging/rtc_event_log/events/rtc_event_dtls_writable_state.h"
#include "logging/rtc_event_log/events/rtc_event_end_log.h"
#include "logging/rtc_event_log/events/rtc_event_frame_decoded.h"
#include "logging/rtc_event_log/events/rtc_event_generic_ack_received.h"
#include "logging/rtc_event_log/events/rtc_event_generic_packet_received.h"
#include "logging/rtc_event_log/events/rtc_event_generic_packet_sent.h"
#include "logging/rtc_event_log/events/rtc_event_ice_candidate_pair.h"
#include "logging/rtc_event_log/events/rtc_event_ice_candidate_pair_config.h"
#include "logging/rtc_event_log/events/rtc_event_log_parse_status.h"
#include "logging/rtc_event_log/events/rtc_event_neteq_set_minimum_delay.h"
#include "logging/rtc_event_log/events/rtc_event_probe_cluster_created.h"
#include "logging/rtc_event_log/events/rtc_event_probe_result_failure.h"
#include "logging/rtc_event_log/events/rtc_event_probe_result_success.h"
#include "logging/rtc_event_log/events/rtc_event_remote_estimate.h"
#include "logging/rtc_event_log/events/rtc_event_route_change.h"
#include "logging/rtc_event_log/events/rtc_event_video_receive_stream_config.h"
#include "logging/rtc_event_log/events/rtc_event_video_send_stream_config.h"
#include "logging/rtc_event_log/rtc_stream_config.h"
#include "modules/rtp_rtcp/include/rtp_header_extension_map.h"
#include "rtc_base/checks.h"

// Files generated at build-time by the protobuf compiler.
#ifdef WEBRTC_ANDROID_PLATFORM_BUILD
#include "external/webrtc/webrtc/logging/rtc_event_log/rtc_event_log.pb.h"
#include "external/webrtc/webrtc/logging/rtc_event_log/rtc_event_log2.pb.h"
#else
#include "logging/rtc_event_log/rtc_event_log.pb.h"
#include "logging/rtc_event_log/rtc_event_log2.pb.h"
#endif

namespace webrtc {

enum PacketDirection { kIncomingPacket = 0, kOutgoingPacket };

enum class LoggedMediaType : uint8_t { kUnknown, kAudio, kVideo };

struct LoggedPacketInfo {
  static LoggedPacketInfo CreateEmptyForTesting() { return LoggedPacketInfo(); }

  LoggedPacketInfo(const LoggedRtpPacket& rtp,
                   LoggedMediaType media_type,
                   bool rtx,
                   Timestamp capture_time);
  LoggedPacketInfo(const LoggedPacketInfo&);
  ~LoggedPacketInfo();

  int64_t log_time_ms() const { return log_packet_time.ms(); }
  int64_t log_time_us() const { return log_packet_time.us(); }
  uint32_t ssrc;
  uint16_t stream_seq_no;
  uint16_t size;
  uint16_t payload_size;
  uint16_t padding_size;
  uint16_t overhead = 0;
  uint8_t payload_type;
  LoggedMediaType media_type = LoggedMediaType::kUnknown;
  bool rtx = false;
  bool marker_bit = false;
  bool has_transport_seq_no = false;
  bool last_in_feedback = false;
  uint16_t transport_seq_no = 0;
  // The RTP header timestamp unwrapped and converted from tick count to seconds
  // based timestamp.
  Timestamp capture_time;
  // The time the packet was logged. This is the receive time for incoming
  // packets and send time for outgoing.
  Timestamp log_packet_time;
  // Send time as reported by abs-send-time extension, For outgoing packets this
  // corresponds to log_packet_time, but might be measured using another clock.
  Timestamp reported_send_time;
  // The receive time that was reported in feedback. For incoming packets this
  // corresponds to log_packet_time, but might be measured using another clock.
  // PlusInfinity indicates that the packet was lost.
  Timestamp reported_recv_time = Timestamp::MinusInfinity();
  // The time feedback message was logged. This is the feedback send time for
  // incoming packets and feedback receive time for outgoing.
  // PlusInfinity indicates that feedback was expected but not received.
  Timestamp log_feedback_time = Timestamp::MinusInfinity();
  // The delay betweeen receiving an RTP packet and sending feedback for
  // incoming packets. For outgoing packets we don't know the feedback send
  // time, and this is instead calculated as the difference in reported receive
  // time between this packet and the last packet in the same feedback message.
  TimeDelta feedback_hold_duration = TimeDelta::MinusInfinity();

 private:
  LoggedPacketInfo()
      : capture_time(Timestamp::MinusInfinity()),
        log_packet_time(Timestamp::MinusInfinity()),
        reported_send_time(Timestamp::MinusInfinity()) {}
};

struct InferredRouteChangeEvent {
  int64_t log_time_ms() const { return log_time.ms(); }
  int64_t log_time_us() const { return log_time.us(); }
  uint32_t route_id;
  Timestamp log_time = Timestamp::MinusInfinity();
  uint16_t send_overhead;
  uint16_t return_overhead;
};

enum class LoggedIceEventType {
  kAdded,
  kUpdated,
  kDestroyed,
  kSelected,
  kCheckSent,
  kCheckReceived,
  kCheckResponseSent,
  kCheckResponseReceived,
};

struct LoggedIceEvent {
  uint32_t candidate_pair_id;
  Timestamp log_time;
  LoggedIceEventType event_type;
};

// This class is used to process lists of LoggedRtpPacketIncoming
// and LoggedRtpPacketOutgoing without duplicating the code.
// TODO(terelius): Remove this class. Instead use e.g. a vector of pointers
// to LoggedRtpPacket or templatize the surrounding code.
template <typename T>
class DereferencingVector {
 public:
  template <bool IsConst>
  class DereferencingIterator {
   public:
    // Standard iterator traits.
    using difference_type = std::ptrdiff_t;
    using value_type = T;
    using pointer = typename std::conditional_t<IsConst, const T*, T*>;
    using reference = typename std::conditional_t<IsConst, const T&, T&>;
    using iterator_category = std::bidirectional_iterator_tag;

    using representation =
        typename std::conditional_t<IsConst, const T* const*, T**>;

    explicit DereferencingIterator(representation ptr) : ptr_(ptr) {}

    DereferencingIterator(const DereferencingIterator& other)
        : ptr_(other.ptr_) {}
    DereferencingIterator(const DereferencingIterator&& other)
        : ptr_(other.ptr_) {}
    ~DereferencingIterator() = default;

    DereferencingIterator& operator=(const DereferencingIterator& other) {
      ptr_ = other.ptr_;
      return *this;
    }
    DereferencingIterator& operator=(const DereferencingIterator&& other) {
      ptr_ = other.ptr_;
      return *this;
    }

    bool operator==(const DereferencingIterator& other) const {
      return ptr_ == other.ptr_;
    }
    bool operator!=(const DereferencingIterator& other) const {
      return ptr_ != other.ptr_;
    }

    DereferencingIterator& operator++() {
      ++ptr_;
      return *this;
    }
    DereferencingIterator& operator--() {
      --ptr_;
      return *this;
    }
    DereferencingIterator operator++(int) {
      DereferencingIterator iter_copy(ptr_);
      ++ptr_;
      return iter_copy;
    }
    DereferencingIterator operator--(int) {
      DereferencingIterator iter_copy(ptr_);
      --ptr_;
      return iter_copy;
    }

    template <bool _IsConst = IsConst>
    std::enable_if_t<!_IsConst, reference> operator*() {
      return **ptr_;
    }

    template <bool _IsConst = IsConst>
    std::enable_if_t<_IsConst, reference> operator*() const {
      return **ptr_;
    }

    template <bool _IsConst = IsConst>
    std::enable_if_t<!_IsConst, pointer> operator->() {
      return *ptr_;
    }

    template <bool _IsConst = IsConst>
    std::enable_if_t<_IsConst, pointer> operator->() const {
      return *ptr_;
    }

   private:
    representation ptr_;
  };

  using value_type = T;
  using reference = value_type&;
  using const_reference = const value_type&;

  using iterator = DereferencingIterator<false>;
  using const_iterator = DereferencingIterator<true>;
  using reverse_iterator = std::reverse_iterator<iterator>;
  using const_reverse_iterator = std::reverse_iterator<const_iterator>;

  iterator begin() { return iterator(elems_.data()); }
  iterator end() { return iterator(elems_.data() + elems_.size()); }

  const_iterator begin() const { return const_iterator(elems_.data()); }
  const_iterator end() const {
    return const_iterator(elems_.data() + elems_.size());
  }

  reverse_iterator rbegin() { return reverse_iterator(end()); }
  reverse_iterator rend() { return reverse_iterator(begin()); }

  const_reverse_iterator rbegin() const {
    return const_reverse_iterator(end());
  }
  const_reverse_iterator rend() const {
    return const_reverse_iterator(begin());
  }

  size_t size() const { return elems_.size(); }

  bool empty() const { return elems_.empty(); }

  T& operator[](size_t i) {
    RTC_DCHECK_LT(i, elems_.size());
    return *elems_[i];
  }

  const T& operator[](size_t i) const {
    RTC_DCHECK_LT(i, elems_.size());
    return *elems_[i];
  }

  void push_back(T* elem) {
    RTC_DCHECK(elem != nullptr);
    elems_.push_back(elem);
  }

 private:
  std::vector<T*> elems_;
};

// Conversion functions for version 2 of the wire format.
BandwidthUsage GetRuntimeDetectorState(
    rtclog2::DelayBasedBweUpdates::DetectorState detector_state);

ProbeFailureReason GetRuntimeProbeFailureReason(
    rtclog2::BweProbeResultFailure::FailureReason failure);

DtlsTransportState GetRuntimeDtlsTransportState(
    rtclog2::DtlsTransportStateEvent::DtlsTransportState state);

IceCandidatePairConfigType GetRuntimeIceCandidatePairConfigType(
    rtclog2::IceCandidatePairConfig::IceCandidatePairConfigType type);

IceCandidateType GetRuntimeIceCandidateType(
    rtclog2::IceCandidatePairConfig::IceCandidateType type);

IceCandidatePairProtocol GetRuntimeIceCandidatePairProtocol(
    rtclog2::IceCandidatePairConfig::Protocol protocol);

IceCandidatePairAddressFamily GetRuntimeIceCandidatePairAddressFamily(
    rtclog2::IceCandidatePairConfig::AddressFamily address_family);

IceCandidateNetworkType GetRuntimeIceCandidateNetworkType(
    rtclog2::IceCandidatePairConfig::NetworkType network_type);

IceCandidatePairEventType GetRuntimeIceCandidatePairEventType(
    rtclog2::IceCandidatePairEvent::IceCandidatePairEventType type);

std::vector<RtpExtension> GetRuntimeRtpHeaderExtensionConfig(
    const rtclog2::RtpHeaderExtensionConfig& proto_header_extensions);
// End of conversion functions.

class ParsedRtcEventLog {
 public:
  enum class MediaType { ANY, AUDIO, VIDEO, DATA };
  enum class UnconfiguredHeaderExtensions {
    kDontParse,
    kAttemptWebrtcDefaultConfig
  };

  using ParseStatus = RtcEventLogParseStatus;

  template <typename T>
  using ParseStatusOr = RtcEventLogParseStatusOr<T>;

  struct LoggedRtpStreamIncoming {
    LoggedRtpStreamIncoming();
    LoggedRtpStreamIncoming(const LoggedRtpStreamIncoming&);
    ~LoggedRtpStreamIncoming();
    uint32_t ssrc;
    std::vector<LoggedRtpPacketIncoming> incoming_packets;
  };

  struct LoggedRtpStreamOutgoing {
    LoggedRtpStreamOutgoing();
    LoggedRtpStreamOutgoing(const LoggedRtpStreamOutgoing&);
    ~LoggedRtpStreamOutgoing();
    uint32_t ssrc;
    std::vector<LoggedRtpPacketOutgoing> outgoing_packets;
  };

  struct LoggedRtpStreamView {
    LoggedRtpStreamView(uint32_t ssrc,
                        const std::vector<LoggedRtpPacketIncoming>& packets);
    LoggedRtpStreamView(uint32_t ssrc,
                        const std::vector<LoggedRtpPacketOutgoing>& packets);
    LoggedRtpStreamView(const LoggedRtpStreamView&);
    uint32_t ssrc;
    DereferencingVector<const LoggedRtpPacket> packet_view;
  };

  class LogSegment {
   public:
    LogSegment(int64_t start_time_us, int64_t stop_time_us)
        : start_time_us_(start_time_us), stop_time_us_(stop_time_us) {}
    int64_t start_time_ms() const { return start_time_us_ / 1000; }
    int64_t start_time_us() const { return start_time_us_; }
    int64_t stop_time_ms() const { return stop_time_us_ / 1000; }
    int64_t stop_time_us() const { return stop_time_us_; }

   private:
    int64_t start_time_us_;
    int64_t stop_time_us_;
  };

  static webrtc::RtpHeaderExtensionMap GetDefaultHeaderExtensionMap();

  explicit ParsedRtcEventLog(
      UnconfiguredHeaderExtensions parse_unconfigured_header_extensions =
          UnconfiguredHeaderExtensions::kDontParse,
      bool allow_incomplete_log = false);

  ParsedRtcEventLog(const ParsedRtcEventLog&) = delete;
  ParsedRtcEventLog& operator=(const ParsedRtcEventLog&) = delete;

  ~ParsedRtcEventLog();

  // Clears previously parsed events and resets the ParsedRtcEventLogNew to an
  // empty state.
  void Clear();

  // Reads an RtcEventLog file and returns success if parsing was successful.
  ParseStatus ParseFile(absl::string_view filename);

  // Reads an RtcEventLog from a string and returns success if successful.
  ParseStatus ParseString(absl::string_view s);

  // Reads an RtcEventLog from an string and returns success if successful.
  ParseStatus ParseStream(absl::string_view s);

  MediaType GetMediaType(uint32_t ssrc, PacketDirection direction) const;

  // Configured SSRCs.
  const std::set<uint32_t>& incoming_rtx_ssrcs() const {
    return incoming_rtx_ssrcs_;
  }

  const std::set<uint32_t>& incoming_video_ssrcs() const {
    return incoming_video_ssrcs_;
  }

  const std::set<uint32_t>& incoming_audio_ssrcs() const {
    return incoming_audio_ssrcs_;
  }

  const std::set<uint32_t>& outgoing_rtx_ssrcs() const {
    return outgoing_rtx_ssrcs_;
  }

  const std::set<uint32_t>& outgoing_video_ssrcs() const {
    return outgoing_video_ssrcs_;
  }

  const std::set<uint32_t>& outgoing_audio_ssrcs() const {
    return outgoing_audio_ssrcs_;
  }

  // Stream configurations.
  const std::vector<LoggedAudioRecvConfig>& audio_recv_configs() const {
    return audio_recv_configs_;
  }

  const std::vector<LoggedAudioSendConfig>& audio_send_configs() const {
    return audio_send_configs_;
  }

  const std::vector<LoggedVideoRecvConfig>& video_recv_configs() const {
    return video_recv_configs_;
  }

  const std::vector<LoggedVideoSendConfig>& video_send_configs() const {
    return video_send_configs_;
  }

  // Beginning and end of log segments.
  const std::vector<LoggedStartEvent>& start_log_events() const {
    return start_log_events_;
  }

  const std::vector<LoggedStopEvent>& stop_log_events() const {
    return stop_log_events_;
  }

  const std::vector<LoggedAlrStateEvent>& alr_state_events() const {
    return alr_state_events_;
  }

  // Audio
  const std::map<uint32_t, std::vector<LoggedAudioPlayoutEvent>>&
  audio_playout_events() const {
    return audio_playout_events_;
  }

  const std::map<uint32_t, std::vector<LoggedNetEqSetMinimumDelayEvent>>&
  neteq_set_minimum_delay_events() const {
    return neteq_set_minimum_delay_events_;
  }

  const std::vector<LoggedAudioNetworkAdaptationEvent>&
  audio_network_adaptation_events() const {
    return audio_network_adaptation_events_;
  }

  // Bandwidth estimation
  const std::vector<LoggedBweProbeClusterCreatedEvent>&
  bwe_probe_cluster_created_events() const {
    return bwe_probe_cluster_created_events_;
  }

  const std::vector<LoggedBweProbeFailureEvent>& bwe_probe_failure_events()
      const {
    return bwe_probe_failure_events_;
  }

  const std::vector<LoggedBweProbeSuccessEvent>& bwe_probe_success_events()
      const {
    return bwe_probe_success_events_;
  }

  const std::vector<LoggedBweDelayBasedUpdate>& bwe_delay_updates() const {
    return bwe_delay_updates_;
  }

  const std::vector<LoggedBweLossBasedUpdate>& bwe_loss_updates() const {
    return bwe_loss_updates_;
  }

  // DTLS
  const std::vector<LoggedDtlsTransportState>& dtls_transport_states() const {
    return dtls_transport_states_;
  }

  const std::vector<LoggedDtlsWritableState>& dtls_writable_states() const {
    return dtls_writable_states_;
  }

  // ICE events
  const std::vector<LoggedIceCandidatePairConfig>& ice_candidate_pair_configs()
      const {
    return ice_candidate_pair_configs_;
  }

  const std::vector<LoggedIceCandidatePairEvent>& ice_candidate_pair_events()
      const {
    return ice_candidate_pair_events_;
  }

  const std::vector<LoggedRouteChangeEvent>& route_change_events() const {
    return route_change_events_;
  }

  const std::vector<LoggedRemoteEstimateEvent>& remote_estimate_events() const {
    return remote_estimate_events_;
  }

  // RTP
  const std::vector<LoggedRtpStreamIncoming>& incoming_rtp_packets_by_ssrc()
      const {
    return incoming_rtp_packets_by_ssrc_;
  }

  const std::vector<LoggedRtpStreamOutgoing>& outgoing_rtp_packets_by_ssrc()
      const {
    return outgoing_rtp_packets_by_ssrc_;
  }

  const std::vector<LoggedRtpStreamView>& rtp_packets_by_ssrc(
      PacketDirection direction) const {
    if (direction == kIncomingPacket)
      return incoming_rtp_packet_views_by_ssrc_;
    else
      return outgoing_rtp_packet_views_by_ssrc_;
  }

  // RTCP
  const std::vector<LoggedRtcpPacketIncoming>& incoming_rtcp_packets() const {
    return incoming_rtcp_packets_;
  }

  const std::vector<LoggedRtcpPacketOutgoing>& outgoing_rtcp_packets() const {
    return outgoing_rtcp_packets_;
  }

  const std::vector<LoggedRtcpPacketReceiverReport>& receiver_reports(
      PacketDirection direction) const {
    if (direction == kIncomingPacket) {
      return incoming_rr_;
    } else {
      return outgoing_rr_;
    }
  }

  const std::vector<LoggedRtcpPacketSenderReport>& sender_reports(
      PacketDirection direction) const {
    if (direction == kIncomingPacket) {
      return incoming_sr_;
    } else {
      return outgoing_sr_;
    }
  }

  const std::vector<LoggedRtcpPacketExtendedReports>& extended_reports(
      PacketDirection direction) const {
    if (direction == kIncomingPacket) {
      return incoming_xr_;
    } else {
      return outgoing_xr_;
    }
  }

  const std::vector<LoggedRtcpPacketNack>& nacks(
      PacketDirection direction) const {
    if (direction == kIncomingPacket) {
      return incoming_nack_;
    } else {
      return outgoing_nack_;
    }
  }

  const std::vector<LoggedRtcpPacketRemb>& rembs(
      PacketDirection direction) const {
    if (direction == kIncomingPacket) {
      return incoming_remb_;
    } else {
      return outgoing_remb_;
    }
  }

  const std::vector<LoggedRtcpPacketFir>& firs(
      PacketDirection direction) const {
    if (direction == kIncomingPacket) {
      return incoming_fir_;
    } else {
      return outgoing_fir_;
    }
  }

  const std::vector<LoggedRtcpPacketPli>& plis(
      PacketDirection direction) const {
    if (direction == kIncomingPacket) {
      return incoming_pli_;
    } else {
      return outgoing_pli_;
    }
  }

  const std::vector<LoggedRtcpPacketBye>& byes(
      PacketDirection direction) const {
    if (direction == kIncomingPacket) {
      return incoming_bye_;
    } else {
      return outgoing_bye_;
    }
  }

  const std::vector<LoggedRtcpPacketTransportFeedback>& transport_feedbacks(
      PacketDirection direction) const {
    if (direction == kIncomingPacket) {
      return incoming_transport_feedback_;
    } else {
      return outgoing_transport_feedback_;
    }
  }

  const std::vector<LoggedRtcpCongestionControlFeedback>& congestion_feedback(
      PacketDirection direction) const {
    if (direction == kIncomingPacket) {
      return incoming_congestion_feedback_;
    } else {
      return outgoing_congestion_feedback_;
    }
  }

  const std::vector<LoggedRtcpPacketLossNotification>& loss_notifications(
      PacketDirection direction) {
    if (direction == kIncomingPacket) {
      return incoming_loss_notification_;
    } else {
      return outgoing_loss_notification_;
    }
  }

  const std::vector<LoggedGenericPacketReceived>& generic_packets_received()
      const {
    return generic_packets_received_;
  }
  const std::vector<LoggedGenericPacketSent>& generic_packets_sent() const {
    return generic_packets_sent_;
  }

  const std::vector<LoggedGenericAckReceived>& generic_acks_received() const {
    return generic_acks_received_;
  }

  // Media
  const std::map<uint32_t, std::vector<LoggedFrameDecoded>>& decoded_frames()
      const {
    return decoded_frames_;
  }

  Timestamp first_timestamp() const { return first_timestamp_; }
  Timestamp last_timestamp() const { return last_timestamp_; }

  const LogSegment& first_log_segment() const { return first_log_segment_; }

  std::vector<LoggedPacketInfo> GetPacketInfos(PacketDirection direction) const;
  std::vector<LoggedPacketInfo> GetIncomingPacketInfos() const {
    return GetPacketInfos(kIncomingPacket);
  }
  std::vector<LoggedPacketInfo> GetOutgoingPacketInfos() const {
    return GetPacketInfos(kOutgoingPacket);
  }
  std::vector<LoggedIceCandidatePairConfig> GetIceCandidates() const;
  std::vector<LoggedIceEvent> GetIceEvents() const;

  std::vector<InferredRouteChangeEvent> GetRouteChanges() const;

 private:
  ABSL_MUST_USE_RESULT ParseStatus ParseStreamInternal(absl::string_view s);
  ABSL_MUST_USE_RESULT ParseStatus ParseStreamInternalV3(absl::string_view s);

  ABSL_MUST_USE_RESULT ParseStatus
  StoreParsedLegacyEvent(const rtclog::Event& event);

  template <typename T>
  void StoreFirstAndLastTimestamp(const std::vector<T>& v);

  // Returns: a pointer to a header extensions map acquired from parsing
  // corresponding Audio/Video Sender/Receiver config events.
  // Warning: if the same SSRC is reused by both video and audio streams during
  // call, extensions maps may be incorrect (the last one would be returned).
  const RtpHeaderExtensionMap* GetRtpHeaderExtensionMap(bool incoming,
                                                        uint32_t ssrc);

  // Reads packet, direction and packet length from the RTCP event at `index`,
  // and stores the values in the corresponding output parameters.
  // Each output parameter can be set to nullptr if that value isn't needed.
  // NB: The packet must have space for at least IP_PACKET_SIZE bytes.
  ParseStatus GetRtcpPacket(const rtclog::Event& event,
                            PacketDirection* incoming,
                            std::vector<uint8_t>* packet) const;

  ParseStatusOr<rtclog::StreamConfig> GetVideoReceiveConfig(
      const rtclog::Event& event) const;
  ParseStatusOr<rtclog::StreamConfig> GetVideoSendConfig(
      const rtclog::Event& event) const;
  ParseStatusOr<rtclog::StreamConfig> GetAudioReceiveConfig(
      const rtclog::Event& event) const;
  ParseStatusOr<rtclog::StreamConfig> GetAudioSendConfig(
      const rtclog::Event& event) const;

  ParsedRtcEventLog::ParseStatusOr<LoggedAudioPlayoutEvent> GetAudioPlayout(
      const rtclog::Event& event) const;

  ParsedRtcEventLog::ParseStatusOr<LoggedBweLossBasedUpdate>
  GetLossBasedBweUpdate(const rtclog::Event& event) const;

  ParsedRtcEventLog::ParseStatusOr<LoggedBweDelayBasedUpdate>
  GetDelayBasedBweUpdate(const rtclog::Event& event) const;

  ParsedRtcEventLog::ParseStatusOr<LoggedAudioNetworkAdaptationEvent>
  GetAudioNetworkAdaptation(const rtclog::Event& event) const;

  ParsedRtcEventLog::ParseStatusOr<LoggedBweProbeClusterCreatedEvent>
  GetBweProbeClusterCreated(const rtclog::Event& event) const;

  ParsedRtcEventLog::ParseStatusOr<LoggedBweProbeFailureEvent>
  GetBweProbeFailure(const rtclog::Event& event) const;

  ParsedRtcEventLog::ParseStatusOr<LoggedBweProbeSuccessEvent>
  GetBweProbeSuccess(const rtclog::Event& event) const;

  ParsedRtcEventLog::ParseStatusOr<LoggedAlrStateEvent> GetAlrState(
      const rtclog::Event& event) const;

  ParsedRtcEventLog::ParseStatusOr<LoggedIceCandidatePairConfig>
  GetIceCandidatePairConfig(const rtclog::Event& event) const;

  ParsedRtcEventLog::ParseStatusOr<LoggedIceCandidatePairEvent>
  GetIceCandidatePairEvent(const rtclog::Event& event) const;

  ParsedRtcEventLog::ParseStatusOr<LoggedRemoteEstimateEvent>
  GetRemoteEstimateEvent(const rtclog::Event& event) const;

  // Parsing functions for new format.
  ParseStatus StoreAlrStateEvent(const rtclog2::AlrState& proto);
  ParseStatus StoreAudioNetworkAdaptationEvent(
      const rtclog2::AudioNetworkAdaptations& proto);
  ParseStatus StoreAudioPlayoutEvent(const rtclog2::AudioPlayoutEvents& proto);
  ParseStatus StoreAudioRecvConfig(const rtclog2::AudioRecvStreamConfig& proto);
  ParseStatus StoreAudioSendConfig(const rtclog2::AudioSendStreamConfig& proto);
  ParseStatus StoreBweDelayBasedUpdate(
      const rtclog2::DelayBasedBweUpdates& proto);
  ParseStatus StoreBweLossBasedUpdate(
      const rtclog2::LossBasedBweUpdates& proto);
  ParseStatus StoreBweProbeClusterCreated(
      const rtclog2::BweProbeCluster& proto);
  ParseStatus StoreBweProbeFailureEvent(
      const rtclog2::BweProbeResultFailure& proto);
  ParseStatus StoreBweProbeSuccessEvent(
      const rtclog2::BweProbeResultSuccess& proto);
  ParseStatus StoreDtlsTransportState(
      const rtclog2::DtlsTransportStateEvent& proto);
  ParseStatus StoreDtlsWritableState(const rtclog2::DtlsWritableState& proto);
  ParsedRtcEventLog::ParseStatus StoreFrameDecodedEvents(
      const rtclog2::FrameDecodedEvents& proto);
  ParseStatus StoreGenericAckReceivedEvent(
      const rtclog2::GenericAckReceived& proto);
  ParseStatus StoreGenericPacketReceivedEvent(
      const rtclog2::GenericPacketReceived& proto);
  ParseStatus StoreGenericPacketSentEvent(
      const rtclog2::GenericPacketSent& proto);
  ParseStatus StoreIceCandidateEvent(
      const rtclog2::IceCandidatePairEvent& proto);
  ParseStatus StoreIceCandidatePairConfig(
      const rtclog2::IceCandidatePairConfig& proto);
  ParseStatus StoreIncomingRtcpPackets(
      const rtclog2::IncomingRtcpPackets& proto);
  ParseStatus StoreIncomingRtpPackets(const rtclog2::IncomingRtpPackets& proto);
  ParseStatus StoreNetEqSetMinimumDelay(
      const rtclog2::NetEqSetMinimumDelay& proto);
  ParseStatus StoreOutgoingRtcpPackets(
      const rtclog2::OutgoingRtcpPackets& proto);
  ParseStatus StoreOutgoingRtpPackets(const rtclog2::OutgoingRtpPackets& proto);
  ParseStatus StoreParsedNewFormatEvent(const rtclog2::EventStream& stream);
  ParseStatus StoreRouteChangeEvent(const rtclog2::RouteChange& proto);
  ParseStatus StoreRemoteEstimateEvent(const rtclog2::RemoteEstimates& proto);
  ParseStatus StoreStartEvent(const rtclog2::BeginLogEvent& proto);
  ParseStatus StoreStopEvent(const rtclog2::EndLogEvent& proto);
  ParseStatus StoreVideoRecvConfig(const rtclog2::VideoRecvStreamConfig& proto);
  ParseStatus StoreVideoSendConfig(const rtclog2::VideoSendStreamConfig& proto);
  // End of new parsing functions.

  struct Stream {
    Stream(uint32_t ssrc,
           MediaType media_type,
           PacketDirection direction,
           webrtc::RtpHeaderExtensionMap map)
        : ssrc(ssrc),
          media_type(media_type),
          direction(direction),
          rtp_extensions_map(map) {}
    uint32_t ssrc;
    MediaType media_type;
    PacketDirection direction;
    webrtc::RtpHeaderExtensionMap rtp_extensions_map;
  };

  const UnconfiguredHeaderExtensions parse_unconfigured_header_extensions_;
  const bool allow_incomplete_logs_;

  // Make a default extension map for streams without configuration information.
  // TODO(ivoc): Once configuration of audio streams is stored in the event log,
  //             this can be removed. Tracking bug: webrtc:6399
  RtpHeaderExtensionMap default_extension_map_;

  // Tracks what each stream is configured for. Note that a single SSRC can be
  // in several sets. For example, the SSRC used for sending video over RTX
  // will appear in both video_ssrcs_ and rtx_ssrcs_. In the unlikely case that
  // an SSRC is reconfigured to a different media type mid-call, it will also
  // appear in multiple sets.
  std::set<uint32_t> incoming_rtx_ssrcs_;
  std::set<uint32_t> incoming_video_ssrcs_;
  std::set<uint32_t> incoming_audio_ssrcs_;
  std::set<uint32_t> outgoing_rtx_ssrcs_;
  std::set<uint32_t> outgoing_video_ssrcs_;
  std::set<uint32_t> outgoing_audio_ssrcs_;

  // Maps an SSRC to the parsed  RTP headers in that stream. Header extensions
  // are parsed if the stream has been configured. This is only used for
  // grouping the events by SSRC during parsing; the events are moved to
  // incoming_rtp_packets_by_ssrc_ once the parsing is done.
  std::map<uint32_t, std::vector<LoggedRtpPacketIncoming>>
      incoming_rtp_packets_map_;
  std::map<uint32_t, std::vector<LoggedRtpPacketOutgoing>>
      outgoing_rtp_packets_map_;

  // RTP headers.
  std::vector<LoggedRtpStreamIncoming> incoming_rtp_packets_by_ssrc_;
  std::vector<LoggedRtpStreamOutgoing> outgoing_rtp_packets_by_ssrc_;
  std::vector<LoggedRtpStreamView> incoming_rtp_packet_views_by_ssrc_;
  std::vector<LoggedRtpStreamView> outgoing_rtp_packet_views_by_ssrc_;

  // Raw RTCP packets.
  std::vector<LoggedRtcpPacketIncoming> incoming_rtcp_packets_;
  std::vector<LoggedRtcpPacketOutgoing> outgoing_rtcp_packets_;

  // Parsed RTCP messages. Currently not separated based on SSRC.
  std::vector<LoggedRtcpPacketReceiverReport> incoming_rr_;
  std::vector<LoggedRtcpPacketReceiverReport> outgoing_rr_;
  std::vector<LoggedRtcpPacketSenderReport> incoming_sr_;
  std::vector<LoggedRtcpPacketSenderReport> outgoing_sr_;
  std::vector<LoggedRtcpPacketExtendedReports> incoming_xr_;
  std::vector<LoggedRtcpPacketExtendedReports> outgoing_xr_;
  std::vector<LoggedRtcpPacketNack> incoming_nack_;
  std::vector<LoggedRtcpPacketNack> outgoing_nack_;
  std::vector<LoggedRtcpPacketRemb> incoming_remb_;
  std::vector<LoggedRtcpPacketRemb> outgoing_remb_;
  std::vector<LoggedRtcpPacketFir> incoming_fir_;
  std::vector<LoggedRtcpPacketFir> outgoing_fir_;
  std::vector<LoggedRtcpPacketPli> incoming_pli_;
  std::vector<LoggedRtcpPacketPli> outgoing_pli_;
  std::vector<LoggedRtcpPacketBye> incoming_bye_;
  std::vector<LoggedRtcpPacketBye> outgoing_bye_;
  std::vector<LoggedRtcpPacketTransportFeedback> incoming_transport_feedback_;
  std::vector<LoggedRtcpPacketTransportFeedback> outgoing_transport_feedback_;
  std::vector<LoggedRtcpCongestionControlFeedback>
      incoming_congestion_feedback_;
  std::vector<LoggedRtcpCongestionControlFeedback>
      outgoing_congestion_feedback_;
  std::vector<LoggedRtcpPacketLossNotification> incoming_loss_notification_;
  std::vector<LoggedRtcpPacketLossNotification> outgoing_loss_notification_;

  std::vector<LoggedStartEvent> start_log_events_;
  std::vector<LoggedStopEvent> stop_log_events_;

  std::vector<LoggedAlrStateEvent> alr_state_events_;

  std::map<uint32_t, std::vector<LoggedAudioPlayoutEvent>>
      audio_playout_events_;
  std::map<uint32_t, std::vector<LoggedNetEqSetMinimumDelayEvent>>
      neteq_set_minimum_delay_events_;

  std::vector<LoggedAudioNetworkAdaptationEvent>
      audio_network_adaptation_events_;

  std::vector<LoggedBweProbeClusterCreatedEvent>
      bwe_probe_cluster_created_events_;

  std::vector<LoggedBweProbeFailureEvent> bwe_probe_failure_events_;
  std::vector<LoggedBweProbeSuccessEvent> bwe_probe_success_events_;

  std::vector<LoggedBweDelayBasedUpdate> bwe_delay_updates_;
  std::vector<LoggedBweLossBasedUpdate> bwe_loss_updates_;

  std::vector<LoggedDtlsTransportState> dtls_transport_states_;
  std::vector<LoggedDtlsWritableState> dtls_writable_states_;

  std::map<uint32_t, std::vector<LoggedFrameDecoded>> decoded_frames_;

  std::vector<LoggedIceCandidatePairConfig> ice_candidate_pair_configs_;
  std::vector<LoggedIceCandidatePairEvent> ice_candidate_pair_events_;

  std::vector<LoggedAudioRecvConfig> audio_recv_configs_;
  std::vector<LoggedAudioSendConfig> audio_send_configs_;
  std::vector<LoggedVideoRecvConfig> video_recv_configs_;
  std::vector<LoggedVideoSendConfig> video_send_configs_;

  std::vector<LoggedGenericPacketReceived> generic_packets_received_;
  std::vector<LoggedGenericPacketSent> generic_packets_sent_;
  std::vector<LoggedGenericAckReceived> generic_acks_received_;

  std::vector<LoggedRouteChangeEvent> route_change_events_;
  std::vector<LoggedRemoteEstimateEvent> remote_estimate_events_;

  std::vector<uint8_t> last_incoming_rtcp_packet_;

  Timestamp first_timestamp_ = Timestamp::PlusInfinity();
  Timestamp last_timestamp_ = Timestamp::MinusInfinity();

  LogSegment first_log_segment_ =
      LogSegment(0, std::numeric_limits<int64_t>::max());

  // The extension maps are mutable to allow us to insert the default
  // configuration when parsing an RTP header for an unconfigured stream.
  // TODO(terelius): This is only used for the legacy format. Remove once we've
  // fully transitioned to the new format.
  mutable std::map<uint32_t, webrtc::RtpHeaderExtensionMap>
      incoming_rtp_extensions_maps_;
  mutable std::map<uint32_t, webrtc::RtpHeaderExtensionMap>
      outgoing_rtp_extensions_maps_;
};

struct MatchedSendArrivalTimes {
  static constexpr int64_t kNotReceived = -1;

  MatchedSendArrivalTimes(int64_t fb, int64_t tx, int64_t rx, int64_t ps)
      : feedback_arrival_time_ms(fb),
        send_time_ms(tx),
        arrival_time_ms(rx),
        payload_size(ps) {}

  int64_t feedback_arrival_time_ms;
  int64_t send_time_ms;
  int64_t arrival_time_ms;  // kNotReceived for lost packets.
  int64_t payload_size;
};

std::vector<MatchedSendArrivalTimes> GetNetworkTrace(
    const ParsedRtcEventLog& parsed_log);

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_RTC_EVENT_LOG_PARSER_H_
