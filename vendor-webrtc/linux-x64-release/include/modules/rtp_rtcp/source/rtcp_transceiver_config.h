/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTCP_TRANSCEIVER_CONFIG_H_
#define MODULES_RTP_RTCP_SOURCE_RTCP_TRANSCEIVER_CONFIG_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <string>

#include "api/array_view.h"
#include "api/rtp_headers.h"
#include "api/task_queue/task_queue_base.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "api/video/video_bitrate_allocation.h"
#include "modules/rtp_rtcp/include/report_block_data.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "system_wrappers/include/clock.h"
#include "system_wrappers/include/ntp_time.h"

namespace webrtc {
class ReceiveStatisticsProvider;

// Interface to watch incoming rtcp packets by media (rtp) receiver.
// All message handlers have default empty implementation. This way users only
// need to implement the ones they are interested in.
class MediaReceiverRtcpObserver {
 public:
  virtual ~MediaReceiverRtcpObserver() = default;

  virtual void OnSenderReport(uint32_t /* sender_ssrc */,
                              NtpTime /* ntp_time */,
                              uint32_t /* rtp_time */) {}
  virtual void OnBye(uint32_t /* sender_ssrc */) {}
  virtual void OnBitrateAllocation(
      uint32_t /* sender_ssrc */,
      const VideoBitrateAllocation& /* allocation */) {}
};

// Handles RTCP related messages for a single RTP stream (i.e. single SSRC)
class RtpStreamRtcpHandler {
 public:
  virtual ~RtpStreamRtcpHandler() = default;

  // Statistic about sent RTP packets to propagate to RTCP sender report.
  class RtpStats {
   public:
    RtpStats() = default;
    RtpStats(const RtpStats&) = default;
    RtpStats& operator=(const RtpStats&) = default;
    ~RtpStats() = default;

    size_t num_sent_packets() const { return num_sent_packets_; }
    size_t num_sent_bytes() const { return num_sent_bytes_; }
    Timestamp last_capture_time() const { return last_capture_time_; }
    uint32_t last_rtp_timestamp() const { return last_rtp_timestamp_; }
    int last_clock_rate() const { return last_clock_rate_; }

    void set_num_sent_packets(size_t v) { num_sent_packets_ = v; }
    void set_num_sent_bytes(size_t v) { num_sent_bytes_ = v; }
    void set_last_capture_time(Timestamp v) { last_capture_time_ = v; }
    void set_last_rtp_timestamp(uint32_t v) { last_rtp_timestamp_ = v; }
    void set_last_clock_rate(int v) { last_clock_rate_ = v; }

   private:
    size_t num_sent_packets_ = 0;
    size_t num_sent_bytes_ = 0;
    Timestamp last_capture_time_ = Timestamp::Zero();
    uint32_t last_rtp_timestamp_ = 0;
    int last_clock_rate_ = 90'000;
  };
  virtual RtpStats SentStats() = 0;

  virtual void OnNack(uint32_t /* sender_ssrc */,
                      ArrayView<const uint16_t> /* sequence_numbers */) {}
  virtual void OnFir(uint32_t /* sender_ssrc */) {}
  virtual void OnPli(uint32_t /* sender_ssrc */) {}

  // Called on an RTCP packet with sender or receiver reports with a report
  // block for the handled RTP stream.
  virtual void OnReport(const ReportBlockData& /* report_block */) {}
};

struct RtcpTransceiverConfig {
  RtcpTransceiverConfig();
  RtcpTransceiverConfig(const RtcpTransceiverConfig&);
  RtcpTransceiverConfig& operator=(const RtcpTransceiverConfig&);
  ~RtcpTransceiverConfig();

  // Logs the error and returns false if configuration miss key objects or
  // is inconsistant. May log warnings.
  bool Validate() const;

  // Used to prepend all log messages. Can be empty.
  std::string debug_id;

  // Ssrc to use as default sender ssrc, e.g. for transport-wide feedbacks.
  uint32_t feedback_ssrc = 1;

  // Canonical End-Point Identifier of the local particiapnt.
  // Defined in rfc3550 section 6 note 2 and section 6.5.1.
  std::string cname;

  // Maximum packet size outgoing transport accepts.
  size_t max_packet_size = 1200;

  // The clock to use when querying for the NTP time. Should be set.
  Clock* clock = nullptr;

  // Transport to send RTCP packets to.
  std::function<void(ArrayView<const uint8_t>)> rtcp_transport;

  // Queue for scheduling delayed tasks, e.g. sending periodic compound packets.
  TaskQueueBase* task_queue = nullptr;

  // Rtcp report block generator for outgoing receiver reports.
  ReceiveStatisticsProvider* receive_statistics = nullptr;

  // Should outlive RtcpTransceiver.
  // Callbacks will be invoked on the `task_queue`.
  NetworkLinkRtcpObserver* network_link_observer = nullptr;

  // Configures if sending should
  //  enforce compound packets: https://tools.ietf.org/html/rfc4585#section-3.1
  //  or allow reduced size packets: https://tools.ietf.org/html/rfc5506
  // Receiving accepts both compound and reduced-size packets.
  RtcpMode rtcp_mode = RtcpMode::kCompound;

  //
  // Tuning parameters.
  //
  // Initial flag if `rtcp_transport` can be used to send packets.
  // If set to false, RtcpTransciever won't call `rtcp_transport` until
  // `RtcpTransceover(Impl)::SetReadyToSend(true)` is called.
  bool initial_ready_to_send = true;

  // Delay before 1st periodic compound packet.
  TimeDelta initial_report_delay = TimeDelta::Millis(500);

  // Period between periodic compound packets.
  TimeDelta report_period = TimeDelta::Seconds(1);

  //
  // Flags for features and experiments.
  //
  bool schedule_periodic_compound_packets = true;
  // Estimate RTT as non-sender as described in
  // https://tools.ietf.org/html/rfc3611#section-4.4 and #section-4.5
  bool non_sender_rtt_measurement = false;

  // Reply to incoming RRTR messages so that remote endpoint may estimate RTT as
  // non-sender as described in https://tools.ietf.org/html/rfc3611#section-4.4
  // and #section-4.5
  bool reply_to_non_sender_rtt_measurement = true;

  // Reply to incoming RRTR messages multiple times, one per sender SSRC, to
  // support clients that calculate and process RTT per sender SSRC.
  bool reply_to_non_sender_rtt_mesaurments_on_all_ssrcs = true;

  // Allows a REMB message to be sent immediately when SetRemb is called without
  // having to wait for the next compount message to be sent.
  bool send_remb_on_change = false;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_RTCP_TRANSCEIVER_CONFIG_H_
