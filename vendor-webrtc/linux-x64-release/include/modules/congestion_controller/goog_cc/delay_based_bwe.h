/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_CONGESTION_CONTROLLER_GOOG_CC_DELAY_BASED_BWE_H_
#define MODULES_CONGESTION_CONTROLLER_GOOG_CC_DELAY_BASED_BWE_H_

#include <stdint.h>

#include <memory>
#include <optional>
#include <vector>

#include "api/field_trials_view.h"
#include "api/network_state_predictor.h"
#include "api/transport/bandwidth_usage.h"
#include "api/transport/network_types.h"
#include "api/units/data_rate.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "modules/congestion_controller/goog_cc/delay_increase_detector_interface.h"
#include "modules/congestion_controller/goog_cc/inter_arrival_delta.h"
#include "modules/congestion_controller/goog_cc/link_capacity_estimator.h"
#include "modules/congestion_controller/goog_cc/probe_bitrate_estimator.h"
#include "modules/remote_bitrate_estimator/aimd_rate_control.h"
#include "modules/remote_bitrate_estimator/inter_arrival.h"
#include "rtc_base/experiments/struct_parameters_parser.h"
#include "rtc_base/race_checker.h"

namespace webrtc {
class RtcEventLog;

struct BweSeparateAudioPacketsSettings {
  static constexpr char kKey[] = "WebRTC-Bwe-SeparateAudioPackets";

  BweSeparateAudioPacketsSettings() = default;
  explicit BweSeparateAudioPacketsSettings(
      const FieldTrialsView* key_value_config);

  bool enabled = false;
  int packet_threshold = 10;
  TimeDelta time_threshold = TimeDelta::Seconds(1);

  std::unique_ptr<StructParametersParser> Parser();
};

class DelayBasedBwe {
 public:
  struct Result {
    Result();
    ~Result() = default;
    bool updated;
    bool probe;
    DataRate target_bitrate = DataRate::Zero();
    bool recovered_from_overuse;
    BandwidthUsage delay_detector_state;
  };

  explicit DelayBasedBwe(const FieldTrialsView* key_value_config,
                         RtcEventLog* event_log,
                         NetworkStatePredictor* network_state_predictor);

  DelayBasedBwe() = delete;
  DelayBasedBwe(const DelayBasedBwe&) = delete;
  DelayBasedBwe& operator=(const DelayBasedBwe&) = delete;

  virtual ~DelayBasedBwe();

  Result IncomingPacketFeedbackVector(
      const TransportPacketsFeedback& msg,
      std::optional<DataRate> acked_bitrate,
      std::optional<DataRate> probe_bitrate,
      std::optional<NetworkStateEstimate> network_estimate,
      bool in_alr);
  void OnRttUpdate(TimeDelta avg_rtt);
  bool LatestEstimate(std::vector<uint32_t>* ssrcs, DataRate* bitrate) const;
  void SetStartBitrate(DataRate start_bitrate);
  void SetMinBitrate(DataRate min_bitrate);
  TimeDelta GetExpectedBwePeriod() const;
  DataRate TriggerOveruse(Timestamp at_time,
                          std::optional<DataRate> link_capacity);
  DataRate last_estimate() const { return prev_bitrate_; }
  BandwidthUsage last_state() const { return prev_state_; }

 private:
  friend class GoogCcStatePrinter;
  void IncomingPacketFeedback(const PacketResult& packet_feedback,
                              Timestamp at_time);
  Result MaybeUpdateEstimate(std::optional<DataRate> acked_bitrate,
                             std::optional<DataRate> probe_bitrate,
                             std::optional<NetworkStateEstimate> state_estimate,
                             bool recovered_from_overuse,
                             bool in_alr,
                             Timestamp at_time);
  // Updates the current remote rate estimate and returns true if a valid
  // estimate exists.
  bool UpdateEstimate(Timestamp at_time,
                      std::optional<DataRate> acked_bitrate,
                      DataRate* target_rate);

  RaceChecker network_race_;
  RtcEventLog* const event_log_;
  const FieldTrialsView* const key_value_config_;

  // Alternatively, run two separate overuse detectors for audio and video,
  // and fall back to the audio one if we haven't seen a video packet in a
  // while.
  BweSeparateAudioPacketsSettings separate_audio_;
  int64_t audio_packets_since_last_video_;
  Timestamp last_video_packet_recv_time_;

  NetworkStatePredictor* network_state_predictor_;
  std::unique_ptr<InterArrival> video_inter_arrival_;
  std::unique_ptr<InterArrivalDelta> video_inter_arrival_delta_;
  std::unique_ptr<DelayIncreaseDetectorInterface> video_delay_detector_;
  std::unique_ptr<InterArrival> audio_inter_arrival_;
  std::unique_ptr<InterArrivalDelta> audio_inter_arrival_delta_;
  std::unique_ptr<DelayIncreaseDetectorInterface> audio_delay_detector_;
  DelayIncreaseDetectorInterface* active_delay_detector_;

  Timestamp last_seen_packet_;
  bool uma_recorded_;
  AimdRateControl rate_control_;
  DataRate prev_bitrate_;
  BandwidthUsage prev_state_;
};

}  // namespace webrtc

#endif  // MODULES_CONGESTION_CONTROLLER_GOOG_CC_DELAY_BASED_BWE_H_
