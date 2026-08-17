/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 *
 *  FEC and NACK added bitrate is handled outside class
 */

#ifndef MODULES_CONGESTION_CONTROLLER_GOOG_CC_SEND_SIDE_BANDWIDTH_ESTIMATION_H_
#define MODULES_CONGESTION_CONTROLLER_GOOG_CC_SEND_SIDE_BANDWIDTH_ESTIMATION_H_

#include <stdint.h>

#include <deque>
#include <memory>
#include <optional>
#include <utility>
#include <vector>

#include "api/field_trials_view.h"
#include "api/transport/bandwidth_usage.h"
#include "api/transport/network_types.h"
#include "api/units/data_rate.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "modules/congestion_controller/goog_cc/loss_based_bandwidth_estimation.h"
#include "modules/congestion_controller/goog_cc/loss_based_bwe_v2.h"
#include "rtc_base/experiments/field_trial_parser.h"

namespace webrtc {

class RtcEventLog;

class LinkCapacityTracker {
 public:
  LinkCapacityTracker() = default;
  ~LinkCapacityTracker() = default;
  // Call when a new delay-based estimate is available.
  void UpdateDelayBasedEstimate(Timestamp at_time,
                                DataRate delay_based_bitrate);
  void OnStartingRate(DataRate start_rate);
  void OnRateUpdate(std::optional<DataRate> acknowledged,
                    DataRate target,
                    Timestamp at_time);
  void OnRttBackoff(DataRate backoff_rate, Timestamp at_time);
  DataRate estimate() const;

 private:
  double capacity_estimate_bps_ = 0;
  Timestamp last_link_capacity_update_ = Timestamp::MinusInfinity();
  DataRate last_delay_based_estimate_ = DataRate::PlusInfinity();
};

class RttBasedBackoff {
 public:
  explicit RttBasedBackoff(const FieldTrialsView* key_value_config);
  ~RttBasedBackoff();
  void UpdatePropagationRtt(Timestamp at_time, TimeDelta propagation_rtt);
  bool IsRttAboveLimit() const;

  FieldTrialFlag disabled_;
  FieldTrialParameter<TimeDelta> configured_limit_;
  FieldTrialParameter<double> drop_fraction_;
  FieldTrialParameter<TimeDelta> drop_interval_;
  FieldTrialParameter<DataRate> bandwidth_floor_;

 public:
  TimeDelta rtt_limit_;
  Timestamp last_propagation_rtt_update_;
  TimeDelta last_propagation_rtt_;
  Timestamp last_packet_sent_;

 private:
  TimeDelta CorrectedRtt() const;
};

class SendSideBandwidthEstimation {
 public:
  SendSideBandwidthEstimation() = delete;
  SendSideBandwidthEstimation(const FieldTrialsView* key_value_config,
                              RtcEventLog* event_log);
  ~SendSideBandwidthEstimation();

  void OnRouteChange();

  DataRate target_rate() const;
  LossBasedState loss_based_state() const;
  // Return whether the current rtt is higher than the rtt limited configured in
  // RttBasedBackoff.
  bool IsRttAboveLimit() const;
  uint8_t fraction_loss() const { return last_fraction_loss_; }
  TimeDelta round_trip_time() const { return last_round_trip_time_; }

  DataRate GetEstimatedLinkCapacity() const;
  // Call periodically to update estimate.
  void UpdateEstimate(Timestamp at_time);
  void OnSentPacket(const SentPacket& sent_packet);
  void UpdatePropagationRtt(Timestamp at_time, TimeDelta propagation_rtt);

  // Call when we receive a RTCP message with TMMBR or REMB.
  void UpdateReceiverEstimate(Timestamp at_time, DataRate bandwidth);

  // Call when a new delay-based estimate is available.
  void UpdateDelayBasedEstimate(Timestamp at_time, DataRate bitrate);

  // Call when we receive a RTCP message with a ReceiveBlock.
  void UpdatePacketsLost(int64_t packets_lost,
                         int64_t number_of_packets,
                         Timestamp at_time);

  // Call when we receive a RTCP message with a ReceiveBlock.
  void UpdateRtt(TimeDelta rtt, Timestamp at_time);

  void SetBitrates(std::optional<DataRate> send_bitrate,
                   DataRate min_bitrate,
                   DataRate max_bitrate,
                   Timestamp at_time);
  void SetSendBitrate(DataRate bitrate, Timestamp at_time);
  void SetMinMaxBitrate(DataRate min_bitrate, DataRate max_bitrate);
  int GetMinBitrate() const;
  void SetAcknowledgedRate(std::optional<DataRate> acknowledged_rate,
                           Timestamp at_time);
  void UpdateLossBasedEstimator(const TransportPacketsFeedback& report,
                                BandwidthUsage delay_detector_state,
                                std::optional<DataRate> probe_bitrate,
                                bool in_alr);
  bool PaceAtLossBasedEstimate() const;

 private:
  friend class GoogCcStatePrinter;

  enum UmaState { kNoUpdate, kFirstDone, kDone };

  bool IsInStartPhase(Timestamp at_time) const;

  void UpdateUmaStatsPacketsLost(Timestamp at_time, int packets_lost);

  // Updates history of min bitrates.
  // After this method returns min_bitrate_history_.front().second contains the
  // min bitrate used during last kBweIncreaseIntervalMs.
  void UpdateMinHistory(Timestamp at_time);

  // Gets the upper limit for the target bitrate. This is the minimum of the
  // delay based limit, the receiver limit and the loss based controller limit.
  DataRate GetUpperLimit() const;
  // Prints a warning if `bitrate` if sufficiently long time has past since last
  // warning.
  void MaybeLogLowBitrateWarning(DataRate bitrate, Timestamp at_time);
  // Stores an update to the event log if the loss rate has changed, the target
  // has changed, or sufficient time has passed since last stored event.
  void MaybeLogLossBasedEvent(Timestamp at_time);

  // Cap `bitrate` to [min_bitrate_configured_, max_bitrate_configured_] and
  // set `current_bitrate_` to the capped value and updates the event log.
  void UpdateTargetBitrate(DataRate bitrate, Timestamp at_time);
  // Applies lower and upper bounds to the current target rate.
  // TODO(srte): This seems to be called even when limits haven't changed, that
  // should be cleaned up.
  void ApplyTargetLimits(Timestamp at_time);

  bool LossBasedBandwidthEstimatorV1Enabled() const;
  bool LossBasedBandwidthEstimatorV2Enabled() const;

  bool LossBasedBandwidthEstimatorV1ReadyForUse() const;
  bool LossBasedBandwidthEstimatorV2ReadyForUse() const;

  const FieldTrialsView* key_value_config_;
  RttBasedBackoff rtt_backoff_;
  LinkCapacityTracker link_capacity_;

  std::deque<std::pair<Timestamp, DataRate> > min_bitrate_history_;

  // incoming filters
  int lost_packets_since_last_loss_update_;
  int expected_packets_since_last_loss_update_;

  std::optional<DataRate> acknowledged_rate_;
  DataRate current_target_;
  DataRate last_logged_target_;
  DataRate min_bitrate_configured_;
  DataRate max_bitrate_configured_;
  Timestamp last_low_bitrate_log_;

  bool has_decreased_since_last_fraction_loss_;
  Timestamp last_loss_feedback_;
  Timestamp last_loss_packet_report_;
  uint8_t last_fraction_loss_;
  uint8_t last_logged_fraction_loss_;
  TimeDelta last_round_trip_time_;

  // The max bitrate as set by the receiver in the call. This is typically
  // signalled using the REMB RTCP message and is used when we don't have any
  // send side delay based estimate.
  DataRate receiver_limit_;
  DataRate delay_based_limit_;
  Timestamp time_last_decrease_;
  Timestamp first_report_time_;
  int initially_lost_packets_;
  DataRate bitrate_at_2_seconds_;
  UmaState uma_update_state_;
  UmaState uma_rtt_state_;
  std::vector<bool> rampup_uma_stats_updated_;
  RtcEventLog* const event_log_;
  Timestamp last_rtc_event_log_;
  float low_loss_threshold_;
  float high_loss_threshold_;
  DataRate bitrate_threshold_;
  LossBasedBandwidthEstimation loss_based_bandwidth_estimator_v1_;
  std::unique_ptr<LossBasedBweV2> loss_based_bandwidth_estimator_v2_;
  LossBasedState loss_based_state_;
  FieldTrialFlag disable_receiver_limit_caps_only_;
};
}  // namespace webrtc
#endif  // MODULES_CONGESTION_CONTROLLER_GOOG_CC_SEND_SIDE_BANDWIDTH_ESTIMATION_H_
