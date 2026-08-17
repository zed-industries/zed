/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_LOG_SIMULATION_H_
#define RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_LOG_SIMULATION_H_

#include <cstddef>
#include <cstdint>
#include <deque>
#include <functional>
#include <map>
#include <memory>

#include "api/rtc_event_log/rtc_event_log.h"
#include "api/transport/network_control.h"
#include "api/transport/network_types.h"
#include "api/units/timestamp.h"
#include "logging/rtc_event_log/events/logged_rtp_rtcp.h"
#include "logging/rtc_event_log/events/rtc_event_ice_candidate_pair_config.h"
#include "logging/rtc_event_log/events/rtc_event_probe_cluster_created.h"
#include "logging/rtc_event_log/rtc_event_log_parser.h"
#include "modules/congestion_controller/rtp/transport_feedback_adapter.h"
#include "modules/rtp_rtcp/source/rtcp_packet/report_block.h"

namespace webrtc {

class LogBasedNetworkControllerSimulation {
 public:
  explicit LogBasedNetworkControllerSimulation(
      std::unique_ptr<NetworkControllerFactoryInterface> factory,
      std::function<void(const NetworkControlUpdate&, Timestamp)>
          update_handler);
  ~LogBasedNetworkControllerSimulation();
  void ProcessEventsInLog(const ParsedRtcEventLog& parsed_log_);

 private:
  struct ProbingStatus {
    const LoggedBweProbeClusterCreatedEvent event;
    size_t bytes_sent;
    size_t packets_sent;
  };
  void HandleStateUpdate(const NetworkControlUpdate& update);
  void ProcessUntil(Timestamp to_time);

  void OnProbeCreated(const LoggedBweProbeClusterCreatedEvent& probe_cluster);
  void OnPacketSent(const LoggedPacketInfo& packet);
  void OnFeedback(const LoggedRtcpPacketTransportFeedback& feedback);
  void OnReceiverReport(const LoggedRtcpPacketReceiverReport& report);
  void OnIceConfig(const LoggedIceCandidatePairConfig& candidate);
  RtcEventLogNull null_event_log_;

  const std::function<void(const NetworkControlUpdate&, Timestamp)>
      update_handler_;
  std::unique_ptr<NetworkControllerFactoryInterface> factory_;
  std::unique_ptr<NetworkControllerInterface> controller_;

  Timestamp current_time_ = Timestamp::MinusInfinity();
  Timestamp last_process_ = Timestamp::MinusInfinity();
  TransportFeedbackAdapter transport_feedback_;
  std::deque<ProbingStatus> pending_probes_;
  std::map<uint32_t, rtcp::ReportBlock> last_report_blocks_;
  Timestamp last_report_block_time_ = Timestamp::MinusInfinity();
};
}  // namespace webrtc

#endif  // RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_LOG_SIMULATION_H_
